use crate::error::Result;
use crate::format::{self, FormatOptions, Formatter};
use crate::model::{Lang, TwineFile};
use crate::output::OutputProcessor;

pub struct GettextFormatter;

impl Formatter for GettextFormatter {
    fn name(&self) -> &'static str {
        "gettext"
    }
    fn extension(&self) -> &'static str {
        ".po"
    }
    fn default_file_name(&self) -> &'static str {
        "strings.po"
    }

    fn read(&self, content: &str, lang: &str, twine_file: &mut TwineFile) -> Result<()> {
        let comment_re = regex::Regex::new(r#"#.? *"(.*)"$"#).unwrap();
        let key_re = regex::Regex::new(r#"msgctxt *"(.*)"$"#).unwrap();
        let value_re = regex::Regex::new(r#"msgstr *"(.*)"$"#).unwrap();

        let mut key: Option<String> = None;
        let mut value: Option<String> = None;
        let mut comment: Option<String> = None;

        for line in content.lines() {
            if let Some(caps) = comment_re.captures(line) {
                comment = Some(caps[1].to_string());
            }
            if let Some(caps) = key_re.captures(line) {
                let k = caps[1].replace("\\\"", "\"");
                if !k.is_empty() {
                    key = Some(k);
                    value = None;
                }
            }
            if let Some(caps) = value_re.captures(line) {
                let v = caps[1].replace("\"\n\"", "").replace("\\\"", "\"");
                if !v.is_empty() {
                    value = Some(v);
                }
            }
            if let (Some(k), Some(v)) = (&key, &value) {
                if !k.is_empty() && !v.is_empty() {
                    format::set_translation(twine_file, k, lang, v);
                    if let Some(ref c) = comment {
                        if !c.is_empty() && !c.starts_with("SECTION:") {
                            format::set_comment(twine_file, k, c);
                        }
                    }
                    comment = None;
                }
                key = None;
                value = None;
            }
        }
        Ok(())
    }

    fn format(&self, lang: &str, twine_file: &TwineFile, options: &FormatOptions) -> Result<Option<String>> {
        let default_lang = twine_file.language_codes.first().cloned().unwrap_or_default();
        let processor = OutputProcessor::new(twine_file.clone(), options.clone());
        let processed = processor.process(lang);

        let mut out = format!(
            "msgid \"\"\nmsgstr \"\"\n\"Language: {}\"\n\"X-Generator: Twinex\"\n\n",
            lang
        );
        let mut has_content = false;

        for section in &processed.sections {
            if section.definitions.is_empty() {
                continue;
            }
            if !section.name.as_str().is_empty() {
                out.push_str(&format!("# SECTION: {}\n", section.name));
            }
            for def in &section.definitions {
                if def.translations.get(&default_lang).is_none() {
                    continue;
                }
                if let Some(ref comment) = def.comment {
                    out.push_str(&format!("#. \"{}\"\n", format::escape_quotes(comment)));
                }
                let value = def.translations.get(&Lang::new(lang)).map(|s| s.as_str()).unwrap_or("");
                let base = def.translations.get(&default_lang).map(|s| s.as_str()).unwrap_or("");
                out.push_str(&format!(
                    "msgctxt \"{}\"\nmsgid \"{}\"\nmsgstr \"{}\"\n",
                    format::escape_quotes(def.key.as_str()),
                    format::escape_quotes(base),
                    format::escape_quotes(value)
                ));
                has_content = true;
            }
        }
        if has_content { Ok(Some(out)) } else { Ok(None) }
    }
}