use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Context;

use crate::cli::CliArgs;
use crate::error::{Result, TwinexError};
use crate::format::{FormatOptions, Registry};
use crate::model::TwineFile;

pub fn run(args: CliArgs) -> anyhow::Result<()> {
    let mut twine_file = TwineFile::new();
    twine_file
        .read_from_path(&args.twine_file)
        .with_context(|| format!("Failed to read twine file: {}", args.twine_file))?;

    if let Some(ref dev_lang) = args.developer_language {
        twine_file.set_developer_language(&crate::model::Lang::new(dev_lang));
    }

    let options = FormatOptions {
        tags: args.tags.clone(),
        untagged: args.untagged,
        include: crate::format::IncludeMode::parse(&args.include),
        developer_language: args.developer_language.clone(),
        escape_all_tags: args.escape_all_tags,
    };

    match args.command.as_str() {
        "generate-localization-file" => {
            let output = args
                .output_path
                .as_ref()
                .ok_or_else(|| TwinexError::InvalidArgument("Missing output path".into()))?;
            Ok(cmd_generate_file(&args, &twine_file, &options, output)?)
        }
        "generate-all-localization-files" => {
            let output = args
                .output_path
                .as_ref()
                .ok_or_else(|| TwinexError::InvalidArgument("Missing output path".into()))?;
            Ok(cmd_generate_all(&args, &twine_file, &options, output)?)
        }
        "generate-localization-archive" => {
            let output = args
                .output_path
                .as_ref()
                .ok_or_else(|| TwinexError::InvalidArgument("Missing output path".into()))?;
            Ok(cmd_generate_archive(&args, &twine_file, &options, output)?)
        }
        "consume-localization-file" => {
            let input = args
                .input_path
                .as_ref()
                .ok_or_else(|| TwinexError::InvalidArgument("Missing input path".into()))?;
            Ok(cmd_consume_file(&args, &mut twine_file, input)?)
        }
        "consume-all-localization-files" => {
            let input = args
                .input_path
                .as_ref()
                .ok_or_else(|| TwinexError::InvalidArgument("Missing input path".into()))?;
            Ok(cmd_consume_all(&args, &mut twine_file, input)?)
        }
        "consume-localization-archive" => {
            let input = args
                .input_path
                .as_ref()
                .ok_or_else(|| TwinexError::InvalidArgument("Missing input path".into()))?;
            Ok(cmd_consume_archive(&args, &mut twine_file, input)?)
        }
        "validate-twine-file" => Ok(cmd_validate(&args, &twine_file)?),
        _ => Err(TwinexError::InvalidArgument(format!("Unknown command: {}", args.command)).into()),
    }
}

// ── Generate ──────────────────────────────────────────────────

fn cmd_generate_file(
    args: &CliArgs,
    twine_file: &TwineFile,
    options: &FormatOptions,
    output_path: &str,
) -> Result<()> {
    if args.validate {
        validate_twine_file(twine_file, args.pedantic)?;
    }
    let lang = args.languages.first().map(|s| s.as_str());
    let formatter = resolve_formatter(args.format.as_deref(), Some(output_path), lang)?;
    let lang = lang
        .map(|l| l.to_string())
        .or_else(|| formatter.detect_language(output_path))
        .ok_or_else(|| {
            TwinexError::InvalidArgument(format!(
                "Unable to determine language for {}. Try --lang.",
                output_path
            ))
        })?;

    let output = formatter
        .format(&lang, twine_file, options)?
        .ok_or(TwinexError::NothingToGenerate)?;

    write_file(output_path, &output, args.encoding.as_deref())?;
    if !args.quiet {
        println!("Generated {}", output_path);
    }
    Ok(())
}

fn cmd_generate_all(
    args: &CliArgs,
    twine_file: &TwineFile,
    options: &FormatOptions,
    output_path: &str,
) -> Result<()> {
    if args.validate {
        validate_twine_file(twine_file, args.pedantic)?;
    }
    let path = Path::new(output_path);
    if !path.is_dir() {
        if args.create_folders {
            fs::create_dir_all(path).map_err(|e| {
                TwinexError::Io(std::io::Error::other(format!(
                    "Cannot create {}: {}",
                    output_path, e
                )))
            })?;
        } else {
            return Err(TwinexError::InvalidArgument(format!(
                "Directory does not exist: {}",
                output_path
            )));
        }
    }

    let formatter = resolve_formatter(args.format.as_deref(), Some(output_path), None)?;
    let file_name = args
        .file_name
        .as_deref()
        .unwrap_or(formatter.default_file_name());

    if args.create_folders {
        for lang in &twine_file.language_codes {
            let lang_dir = formatter.output_dir_for_lang(lang.as_str());
            let dir = path.join(&lang_dir);
            fs::create_dir_all(&dir).map_err(|e| {
                TwinexError::Io(std::io::Error::other(format!(
                    "Cannot create {}: {}",
                    dir.display(),
                    e
                )))
            })?;
            generate_one_file(
                &*formatter,
                twine_file,
                options,
                lang.as_str(),
                &dir.join(file_name),
                args,
            )?;
        }
    } else {
        let mut found = false;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let dir = entry.path();
            if !dir.is_dir() {
                continue;
            }
            if let Some(lang) = formatter.detect_language(dir.to_str().unwrap_or("")) {
                found = true;
                let out = dir.join(file_name);
                generate_one_file(&*formatter, twine_file, options, &lang, &out, args)?;
            }
        }
        if !found {
            return Err(TwinexError::InvalidArgument(format!(
                "No languages found at {}",
                output_path
            )));
        }
    }
    Ok(())
}

fn generate_one_file(
    formatter: &dyn crate::format::Formatter,
    twine_file: &TwineFile,
    options: &FormatOptions,
    lang: &str,
    output: &Path,
    args: &CliArgs,
) -> Result<()> {
    match formatter.format(lang, twine_file, options)? {
        Some(content) => {
            write_file(output.to_str().unwrap(), &content, args.encoding.as_deref())?;
            if !args.quiet {
                println!("Generated {}", output.display());
            }
        }
        None => {
            if !args.quiet {
                println!("Skipping {} — would be empty.", output.display());
            }
        }
    }
    Ok(())
}

fn cmd_generate_archive(
    args: &CliArgs,
    twine_file: &TwineFile,
    options: &FormatOptions,
    output_path: &str,
) -> Result<()> {
    if args.validate {
        validate_twine_file(twine_file, args.pedantic)?;
    }
    let format_name = args.format.as_deref().ok_or_else(|| {
        TwinexError::InvalidArgument("--format is required for archive generation".into())
    })?;
    let formatter = Registry::get(format_name)
        .ok_or_else(|| TwinexError::InvalidArgument(format!("Unknown format: {}", format_name)))?;

    let file = fs::File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.add_directory("Locales/", opts)?;

    for lang in &twine_file.language_codes {
        if !args.languages.is_empty() && !args.languages.contains(&lang.as_str().to_string()) {
            continue;
        }
        let name = format!("{}{}", lang, formatter.extension());
        match formatter.format(lang.as_str(), twine_file, options)? {
            Some(content) => {
                zip.start_file(format!("Locales/{}", name), opts)?;
                zip.write_all(content.as_bytes())?;
                if !args.quiet {
                    println!("Added {} to archive", name);
                }
            }
            None => {
                if !args.quiet {
                    println!("Skipping {} — would be empty.", name);
                }
            }
        }
    }
    zip.finish()?;
    Ok(())
}

// ── Consume ───────────────────────────────────────────────────

fn cmd_consume_file(args: &CliArgs, twine_file: &mut TwineFile, input_path: &str) -> Result<()> {
    let lang = args.languages.first().map(|s| s.as_str());
    read_localization_file(args, twine_file, input_path, lang)?;
    let out = args.output_path.as_deref().unwrap_or(&args.twine_file);
    twine_file.write_to_path(out)?;
    Ok(())
}

fn cmd_consume_all(args: &CliArgs, twine_file: &mut TwineFile, input_path: &str) -> Result<()> {
    if !Path::new(input_path).is_dir() {
        return Err(TwinexError::InvalidArgument(format!(
            "Directory does not exist: {}",
            input_path
        )));
    }
    visit_dirs(Path::new(input_path), args, twine_file)?;
    let out = args.output_path.as_deref().unwrap_or(&args.twine_file);
    twine_file.write_to_path(out)?;
    Ok(())
}

fn visit_dirs(dir: &Path, args: &CliArgs, twine_file: &mut TwineFile) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Err(e) = read_localization_file(args, twine_file, path.to_str().unwrap(), None) {
                eprintln!("{}", e);
            }
        } else if path.is_dir() {
            visit_dirs(&path, args, twine_file)?;
        }
    }
    Ok(())
}

fn cmd_consume_archive(args: &CliArgs, twine_file: &mut TwineFile, input_path: &str) -> Result<()> {
    if !Path::new(input_path).is_file() {
        return Err(TwinexError::InvalidArgument(format!(
            "File does not exist: {}",
            input_path
        )));
    }
    let file = fs::File::open(input_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut errors = false;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        if name.ends_with('/') {
            continue;
        }
        if Path::new(&name)
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with('.'))
        {
            continue;
        }
        let mut content = String::new();
        std::io::Read::read_to_string(&mut entry, &mut content)?;

        if let Err(e) = read_localization_from_content(args, twine_file, &name, &content) {
            eprintln!("{}", e);
            errors = true;
        }
    }
    let out = args.output_path.as_deref().unwrap_or(&args.twine_file);
    twine_file.write_to_path(out)?;
    if errors {
        Err(TwinexError::Format(
            "At least one file could not be consumed".into(),
        ))
    } else {
        Ok(())
    }
}

// ── Validate ──────────────────────────────────────────────────

fn cmd_validate(args: &CliArgs, twine_file: &TwineFile) -> Result<()> {
    validate_twine_file(twine_file, args.pedantic)?;
    if !args.quiet {
        println!("{} is valid.", args.twine_file);
    }
    Ok(())
}

fn validate_twine_file(twine_file: &TwineFile, pedantic: bool) -> Result<()> {
    use std::collections::HashSet;
    let mut all_keys = HashSet::new();
    let mut dupes = HashSet::new();
    let mut no_tags = HashSet::new();
    let mut invalid = HashSet::new();
    let mut python_placeholders = HashSet::new();
    let valid_key = regex::Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
    let mut total = 0usize;

    for section in &twine_file.sections {
        for def in &section.definitions {
            total += 1;
            if !all_keys.insert(def.key.clone()) {
                dupes.insert(def.key.clone());
            }
            if def.tags.is_empty() {
                no_tags.insert(def.key.clone());
            }
            if !valid_key.is_match(def.key.as_str()) {
                invalid.insert(def.key.clone());
            }
            if def
                .translations
                .values()
                .any(|v| crate::placeholders::contains_python_specific_placeholder(v))
            {
                python_placeholders.insert(def.key.clone());
            }
        }
    }

    let mut errors = Vec::new();
    if !dupes.is_empty() {
        errors.push(format!(
            "Found duplicate key(s):\n{}",
            dupes
                .iter()
                .map(|k| format!("  {}", k))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    if pedantic {
        if no_tags.len() == total {
            errors.push("None of your definitions have tags.".to_string());
        } else if !no_tags.is_empty() {
            errors.push(format!(
                "Found definitions without tags:\n{}",
                no_tags
                    .iter()
                    .map(|k| format!("  {}", k))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
    }
    if !invalid.is_empty() {
        errors.push(format!(
            "Found key(s) with invalid characters:\n{}",
            invalid
                .iter()
                .map(|k| format!("  {}", k))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    if !python_placeholders.is_empty() {
        errors.push(format!(
            "Found key(s) with Python-only placeholders:\n{}",
            python_placeholders
                .iter()
                .map(|k| format!("  {}", k))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    if !errors.is_empty() {
        return Err(TwinexError::Validation(errors.join("\n\n")));
    }
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────

fn resolve_formatter(
    name: Option<&str>,
    path: Option<&str>,
    _lang: Option<&str>,
) -> Result<Box<dyn crate::format::Formatter>> {
    Registry::detect(name, path).ok_or_else(|| {
        TwinexError::InvalidArgument("Unable to determine format. Use --format to specify.".into())
    })
}

fn read_localization_file(
    args: &CliArgs,
    twine_file: &mut TwineFile,
    path: &str,
    _lang: Option<&str>,
) -> Result<()> {
    if !Path::new(path).is_file() {
        return Err(TwinexError::InvalidArgument(format!(
            "File does not exist: {}",
            path
        )));
    }
    let content = read_file_with_encoding(path, args.encoding.as_deref())?;
    read_localization_from_content(args, twine_file, path, &content)
}

fn read_localization_from_content(
    args: &CliArgs,
    twine_file: &mut TwineFile,
    path: &str,
    content: &str,
) -> Result<()> {
    let lang = args.languages.first().map(|s| s.as_str());
    let formatter = resolve_formatter(args.format.as_deref(), Some(path), lang)?;
    let lang = lang
        .map(|l| l.to_string())
        .or_else(|| formatter.detect_language(path))
        .ok_or_else(|| {
            TwinexError::InvalidArgument(format!(
                "Unable to determine language for {}. Try --lang.",
                path
            ))
        })?;

    if !twine_file
        .language_codes
        .contains(&crate::model::Lang::new(&lang))
    {
        twine_file.add_language(&crate::model::Lang::new(&lang));
    }
    formatter.read(content, &lang, twine_file)
}

fn read_file_with_encoding(path: &str, encoding: Option<&str>) -> Result<String> {
    let enc = encoding.unwrap_or("UTF-8");
    let bytes = fs::read(path)?;
    let start = if bytes.len() >= 2 {
        match &bytes[..2] {
            [0xFE, 0xFF] | [0xFF, 0xFE] => 2,
            _ => 0,
        }
    } else {
        0
    };
    let content = &bytes[start..];
    match enc.to_uppercase().as_str() {
        "UTF-8" | "UTF8" => Ok(String::from_utf8(content.to_vec())
            .map_err(|e| TwinexError::Format(format!("Invalid UTF-8: {}", e)))?),
        "UTF-16BE" | "UTF16BE" => {
            let u16: Vec<u16> = content
                .chunks(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16(&u16).map_err(|_| TwinexError::Format("Invalid UTF-16".into()))
        }
        "UTF-16LE" | "UTF16LE" => {
            let u16: Vec<u16> = content
                .chunks(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16(&u16).map_err(|_| TwinexError::Format("Invalid UTF-16".into()))
        }
        _ => Ok(String::from_utf8(content.to_vec())
            .map_err(|e| TwinexError::Format(format!("Invalid encoding: {}", e)))?),
    }
}

fn write_file(path: &str, content: &str, encoding: Option<&str>) -> Result<()> {
    let enc = encoding.unwrap_or("UTF-8");
    match enc.to_uppercase().as_str() {
        "UTF-8" | "UTF8" => Ok(fs::write(path, content)?),
        "UTF-16BE" | "UTF16BE" => {
            let u16: Vec<u16> = content.encode_utf16().collect();
            let mut bytes = vec![0xFE, 0xFF];
            for c in &u16 {
                bytes.extend_from_slice(&c.to_be_bytes());
            }
            Ok(fs::write(path, bytes)?)
        }
        "UTF-16LE" | "UTF16LE" => {
            let u16: Vec<u16> = content.encode_utf16().collect();
            let mut bytes = vec![0xFF, 0xFE];
            for c in &u16 {
                bytes.extend_from_slice(&c.to_le_bytes());
            }
            Ok(fs::write(path, bytes)?)
        }
        _ => Ok(fs::write(path, content)?),
    }
}
