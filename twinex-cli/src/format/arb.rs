use crate::error::Result;
use crate::format::{self, FormatOptions, Formatter};
use crate::model::{Lang, TwineFile};
use crate::output::OutputProcessor;
use serde_json::{Map, Value};

pub struct ArbFormatter;

impl Formatter for ArbFormatter {
    fn name(&self) -> &'static str {
        "arb"
    }

    fn extension(&self) -> &'static str {
        ".arb"
    }

    fn default_file_name(&self) -> &'static str {
        "app_en.arb"
    }

    fn detect_language(&self, path: &str) -> Option<String> {
        let basename = std::path::Path::new(path).file_stem()?.to_str()?;
        // Extract language from filename like "intl_en" or "app_fr"
        let re = regex::Regex::new(r".*_([a-z]{2}(?:-[A-Z]{2})?)$").unwrap();
        re.captures(basename).map(|c| c[1].to_string())
    }

    fn read(&self, content: &str, lang: &str, twine_file: &mut TwineFile) -> Result<()> {
        let json: Value = serde_json::from_str(content)
            .map_err(|e| crate::error::TwinexError::Format(format!("Invalid ARB JSON: {}", e)))?;

        let obj = json.as_object().ok_or_else(|| {
            crate::error::TwinexError::Format("ARB root must be a JSON object".into())
        })?;

        for (key, value) in obj {
            // Skip metadata keys (starting with @)
            if key.starts_with('@') {
                continue;
            }

            if let Some(translation) = value.as_str() {
                format::set_translation(twine_file, key, lang, translation);

                // Check for @key.description metadata
                let meta_key = format!("@{}", key);
                if let Some(meta) = obj.get(&meta_key) {
                    if let Some(desc) = meta.get("description").and_then(|v| v.as_str()) {
                        format::set_comment(twine_file, key, desc);
                    }
                }
            }
        }
        Ok(())
    }

    fn format(
        &self,
        lang: &str,
        twine_file: &TwineFile,
        options: &FormatOptions,
    ) -> Result<Option<String>> {
        let processor = OutputProcessor::new(twine_file.clone(), options.clone());
        let processed = processor.process(lang);

        let mut map = Map::new();
        map.insert("@@locale".to_string(), Value::String(lang.to_string()));

        for section in &processed.sections {
            for def in &section.definitions {
                let value = def
                    .translations
                    .get(&Lang::new(lang))
                    .map(|s| s.as_str())
                    .unwrap_or("");
                map.insert(
                    def.key.as_str().to_string(),
                    Value::String(value.to_string()),
                );

                if let Some(ref comment) = def.comment {
                    let mut meta = Map::new();
                    meta.insert("description".to_string(), Value::String(comment.clone()));
                    map.insert(format!("@{}", def.key.as_str()), Value::Object(meta));
                }
            }
        }

        // Only @@locale present — nothing to generate
        if map.len() <= 1 {
            return Ok(None);
        }

        let output = serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| {
            crate::error::TwinexError::Format(format!("ARB serialization error: {}", e))
        })?;

        Ok(Some(output + "\n"))
    }
}
