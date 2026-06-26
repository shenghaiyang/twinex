use std::fs;
use std::io::Write;
use std::path::Path;

use crate::encoding;
use crate::error::{Result, TwinexError};
use crate::format::{FormatOptions, Registry};
use crate::model::TwineFile;
use crate::twinex_params::{ConsumeParams, GenerateParams, ValidateParams};

// ── Generate ──────────────────────────────────────────────────

pub fn generate(params: &GenerateParams) -> Result<()> {
    let twine = read_twine_file(&params.twine_file, params.developer_language.as_deref())?;
    let options = make_format_options(params);

    if params.validate {
        validate_twine_file(&twine, false)?;
    }

    if params.archive {
        generate_archive_impl(&twine, &options, params)
    } else if params.all {
        generate_all_impl(&twine, &options, params)
    } else {
        generate_file_impl(&twine, &options, params)
    }
}

fn generate_file_impl(
    twine: &TwineFile,
    options: &FormatOptions,
    params: &GenerateParams,
) -> Result<()> {
    let lang = params.languages.first().map(|s| s.as_str());
    let formatter = resolve_formatter(params.format.as_deref(), Some(&params.output_path), lang)?;
    let lang = lang
        .map(|l| l.to_string())
        .or_else(|| formatter.detect_language(&params.output_path))
        .ok_or_else(|| {
            TwinexError::InvalidArgument(format!(
                "Unable to determine language for {}. Try --lang.",
                params.output_path
            ))
        })?;

    let output = formatter
        .format(&lang, twine, options)?
        .ok_or(TwinexError::NothingToGenerate)?;

    encoding::write_file(&params.output_path, &output, params.encoding.as_deref())?;
    if !params.quiet {
        println!("Generated {}", params.output_path);
    }
    Ok(())
}

fn generate_all_impl(
    twine: &TwineFile,
    options: &FormatOptions,
    params: &GenerateParams,
) -> Result<()> {
    let path = Path::new(&params.output_path);
    if !path.is_dir() {
        if params.create_folders {
            fs::create_dir_all(path).map_err(|e| {
                TwinexError::Io(std::io::Error::other(format!(
                    "Cannot create {}: {}",
                    params.output_path, e
                )))
            })?;
        } else {
            return Err(TwinexError::InvalidArgument(format!(
                "Directory does not exist: {}",
                params.output_path
            )));
        }
    }

    let formatter = resolve_formatter(params.format.as_deref(), Some(&params.output_path), None)?;
    let file_name = params
        .file_name
        .as_deref()
        .unwrap_or(formatter.default_file_name());

    if params.create_folders {
        for lang in &twine.language_codes {
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
                twine,
                options,
                lang.as_str(),
                &dir.join(file_name),
                params.quiet,
                params.encoding.as_deref(),
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
                generate_one_file(
                    &*formatter,
                    twine,
                    options,
                    &lang,
                    &out,
                    params.quiet,
                    params.encoding.as_deref(),
                )?;
            }
        }
        if !found {
            return Err(TwinexError::InvalidArgument(format!(
                "No languages found at {}",
                params.output_path
            )));
        }
    }
    Ok(())
}

fn generate_archive_impl(
    twine: &TwineFile,
    options: &FormatOptions,
    params: &GenerateParams,
) -> Result<()> {
    let format_name = params.format.as_deref().ok_or_else(|| {
        TwinexError::InvalidArgument("--format is required for archive generation".into())
    })?;
    let formatter = Registry::get(format_name)
        .ok_or_else(|| TwinexError::InvalidArgument(format!("Unknown format: {}", format_name)))?;

    let file = fs::File::create(&params.output_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.add_directory("Locales/", opts)?;

    for lang in &twine.language_codes {
        if !params.languages.is_empty() && !params.languages.contains(&lang.as_str().to_string()) {
            continue;
        }
        let name = format!("{}{}", lang, formatter.extension());
        match formatter.format(lang.as_str(), twine, options)? {
            Some(content) => {
                zip.start_file(format!("Locales/{}", name), opts)?;
                zip.write_all(content.as_bytes())?;
                if !params.quiet {
                    println!("Added {} to archive", name);
                }
            }
            None => {
                if !params.quiet {
                    println!("Skipping {} — would be empty.", name);
                }
            }
        }
    }
    zip.finish()?;
    Ok(())
}

// ── Consume ──────────────────────────────────────────────────

pub fn consume(params: &ConsumeParams) -> Result<()> {
    let mut twine = read_twine_file(&params.twine_file, params.developer_language.as_deref())?;

    if params.all {
        consume_all_impl(&mut twine, params)
    } else if params.input_path.ends_with(".zip") {
        consume_archive_impl(&mut twine, params)
    } else {
        consume_file_impl(&mut twine, params)
    }
}

fn consume_file_impl(twine: &mut TwineFile, params: &ConsumeParams) -> Result<()> {
    let lang = params.languages.first().map(|s| s.as_str());
    read_localization_file(
        twine,
        &params.input_path,
        lang,
        params.format.as_deref(),
        params.encoding.as_deref(),
        &params.languages,
    )?;
    let out = params.output_path.as_deref().unwrap_or(&params.twine_file);
    twine.write_to_path(out)?;
    Ok(())
}

fn consume_all_impl(twine: &mut TwineFile, params: &ConsumeParams) -> Result<()> {
    if !Path::new(&params.input_path).is_dir() {
        return Err(TwinexError::InvalidArgument(format!(
            "Directory does not exist: {}",
            params.input_path
        )));
    }
    visit_dirs(
        Path::new(&params.input_path),
        twine,
        params.format.as_deref(),
        params.encoding.as_deref(),
        &[],
    )?;
    let out = params.output_path.as_deref().unwrap_or(&params.twine_file);
    twine.write_to_path(out)?;
    Ok(())
}

fn consume_archive_impl(twine: &mut TwineFile, params: &ConsumeParams) -> Result<()> {
    if !Path::new(&params.input_path).is_file() {
        return Err(TwinexError::InvalidArgument(format!(
            "File does not exist: {}",
            params.input_path
        )));
    }
    let file = fs::File::open(&params.input_path)?;
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

        if let Err(e) =
            read_localization_from_content(twine, &name, &content, params.format.as_deref(), &[])
        {
            eprintln!("{}", e);
            errors = true;
        }
    }
    let out = params.output_path.as_deref().unwrap_or(&params.twine_file);
    twine.write_to_path(out)?;
    if errors {
        Err(TwinexError::Format(
            "At least one file could not be consumed".into(),
        ))
    } else {
        Ok(())
    }
}

// ── Validate ──────────────────────────────────────────────────

pub fn validate(params: &ValidateParams) -> Result<()> {
    let twine = read_twine_file(&params.twine_file, params.developer_language.as_deref())?;
    validate_twine_file(&twine, params.pedantic)?;
    if !params.quiet {
        println!("{} is valid.", params.twine_file);
    }
    Ok(())
}

// ── Free helpers ──────────────────────────────────────────────

fn read_twine_file(path: &str, dev_lang: Option<&str>) -> Result<TwineFile> {
    let mut twine = TwineFile::new();
    twine.read_from_path(path)?;
    if let Some(lang) = dev_lang {
        twine.set_developer_language(&crate::model::Lang::new(lang));
    }
    Ok(twine)
}

fn make_format_options(params: &GenerateParams) -> FormatOptions {
    FormatOptions {
        tags: wrap_tags(params.tags.clone()),
        untagged: params.untagged,
        include: crate::format::IncludeMode::parse(&params.include),
        developer_language: params.developer_language.clone(),
        escape_all_tags: params.escape_all_tags,
    }
}

fn wrap_tags(tags: Vec<String>) -> Vec<Vec<String>> {
    if tags.is_empty() {
        Vec::new()
    } else {
        vec![tags]
    }
}

fn resolve_formatter(
    name: Option<&str>,
    path: Option<&str>,
    _lang: Option<&str>,
) -> Result<Box<dyn crate::format::Formatter>> {
    Registry::detect(name, path).ok_or_else(|| {
        TwinexError::InvalidArgument("Unable to determine format. Use --format to specify.".into())
    })
}

fn generate_one_file(
    formatter: &dyn crate::format::Formatter,
    twine: &TwineFile,
    options: &FormatOptions,
    lang: &str,
    output: &Path,
    quiet: bool,
    encoding: Option<&str>,
) -> Result<()> {
    match formatter.format(lang, twine, options)? {
        Some(content) => {
            encoding::write_file(output.to_str().unwrap(), &content, encoding)?;
            if !quiet {
                println!("Generated {}", output.display());
            }
        }
        None => {
            if !quiet {
                println!("Skipping {} — would be empty.", output.display());
            }
        }
    }
    Ok(())
}

fn read_localization_file(
    twine: &mut TwineFile,
    path: &str,
    _lang: Option<&str>,
    format: Option<&str>,
    encoding: Option<&str>,
    languages: &[String],
) -> Result<()> {
    if !Path::new(path).is_file() {
        return Err(TwinexError::InvalidArgument(format!(
            "File does not exist: {}",
            path
        )));
    }
    let content = encoding::read_file_with_encoding(path, encoding)?;
    read_localization_from_content(twine, path, &content, format, languages)
}

fn read_localization_from_content(
    twine: &mut TwineFile,
    path: &str,
    content: &str,
    format: Option<&str>,
    languages: &[String],
) -> Result<()> {
    let lang = languages.first().map(|s| s.as_str());
    let formatter = resolve_formatter(format, Some(path), lang)?;
    let lang = lang
        .map(|l| l.to_string())
        .or_else(|| formatter.detect_language(path))
        .ok_or_else(|| {
            TwinexError::InvalidArgument(format!(
                "Unable to determine language for {}. Try --lang.",
                path
            ))
        })?;

    if !twine
        .language_codes
        .contains(&crate::model::Lang::new(&lang))
    {
        twine.add_language(&crate::model::Lang::new(&lang));
    }
    formatter.read(content, &lang, twine)
}

fn visit_dirs(
    dir: &Path,
    twine: &mut TwineFile,
    format: Option<&str>,
    encoding: Option<&str>,
    languages: &[String],
) -> Result<()> {
    let mut errors: Vec<String> = Vec::new();
    visit_dirs_recursive(dir, twine, format, encoding, languages, &mut errors)?;
    if errors.is_empty() {
        Ok(())
    } else {
        Err(TwinexError::Format(format!(
            "Encountered {} error(s) while consuming files:\n{}",
            errors.len(),
            errors.join("\n")
        )))
    }
}

fn visit_dirs_recursive(
    dir: &Path,
    twine: &mut TwineFile,
    format: Option<&str>,
    encoding: Option<&str>,
    languages: &[String],
    errors: &mut Vec<String>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Err(e) = read_localization_file(
                twine,
                path.to_str().unwrap_or(""),
                None,
                format,
                encoding,
                languages,
            ) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        } else if path.is_dir() {
            visit_dirs_recursive(&path, twine, format, encoding, languages, errors)?;
        }
    }
    Ok(())
}

fn validate_twine_file(twine: &TwineFile, pedantic: bool) -> Result<()> {
    use std::collections::HashSet;
    let mut all_keys = HashSet::new();
    let mut dupes = HashSet::new();
    let mut no_tags = HashSet::new();
    let mut invalid = HashSet::new();
    let mut python_placeholders = HashSet::new();
    let valid_key = regex::Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
    let mut total = 0usize;

    for section in &twine.sections {
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
