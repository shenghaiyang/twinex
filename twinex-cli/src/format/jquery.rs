use crate::error::Result;

use crate::format::{self, FormatOptions, Formatter};
use crate::model::{Lang, TwineFile};
use crate::output::OutputProcessor;

pub struct JQueryFormatter;

impl Formatter for JQueryFormatter {
    fn name(&self) -> &'static str {
        "jquery"
    }
    fn extension(&self) -> &'static str {
        ".json"
    }
    fn default_file_name(&self) -> &'static str {
        "localize.json"
    }
    fn detect_language(&self, path: &str) -> Option<String> {
        let re = regex::Regex::new(r"^.+-([^-]{2})\.json$").unwrap();
        let basename = std::path::Path::new(path).file_name()?.to_str()?;
        re.captures(basename).map(|c| c[1].to_string())
    }

    fn read(&self, content: &str, lang: &str, twine_file: &mut TwineFile) -> Result<()> {
        let json_re = regex::Regex::new(r#""([^"]+)"\s*:\s*"((?:[^"\\]|\\.)*)""#).unwrap();
        for caps in json_re.captures_iter(content) {
            let key = caps[1].to_string();
            let value = caps[2].replace("\\\"", "\"");
            format::set_translation(twine_file, &key, lang, &value);
        }
        Ok(())
    }

    fn format(
        &self,
        lang: &str,
        twine_file: &TwineFile,
        options: &FormatOptions,
    ) -> Result<Option<String>> {
        let processor = OutputProcessor::new(twine_file, options);
        let processed = processor.process(lang);

        let pairs: Vec<String> = processed
            .sections
            .iter()
            .flat_map(|s| &s.definitions)
            .map(|def| {
                let value = def
                    .translations
                    .get(&Lang::new(lang))
                    .map(|s| s.as_str())
                    .unwrap_or("");
                format!(
                    "\"{}\":\"{}\"",
                    json_escape(def.key.as_str()),
                    json_escape(value)
                )
            })
            .collect();

        if pairs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(format!("{{\n{}\n}}\n", pairs.join(",\n"))))
        }
    }
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
