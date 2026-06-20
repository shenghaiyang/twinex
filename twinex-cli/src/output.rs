use crate::format::FormatOptions;
use crate::model::{Definition, Lang, Section, TwineFile};

/// Processes a TwineFile to produce a filtered output for a given language.
pub struct OutputProcessor {
    twine_file: TwineFile,
    options: FormatOptions,
}

impl OutputProcessor {
    pub fn new(twine_file: TwineFile, options: FormatOptions) -> Self {
        OutputProcessor {
            twine_file,
            options,
        }
    }

    pub fn process(&self, lang: &str) -> TwineFile {
        let mut result = TwineFile::new();
        result.language_codes = self.twine_file.language_codes.clone();

        for section in &self.twine_file.sections {
            let mut new_section = Section::new(section.name.clone());

            for def in &section.definitions {
                if !def.matches_tags(&self.options.tags, self.options.untagged) {
                    continue;
                }

                let value = def.translations.get(&Lang::new(lang));

                // Filter by include mode
                match self.options.include {
                    crate::format::IncludeMode::Translated if value.is_none() => continue,
                    crate::format::IncludeMode::Untranslated if value.is_some() => continue,
                    _ => {}
                }

                let final_value = if let Some(v) = value {
                    v.clone()
                } else {
                    match self.fallback_value(def, lang) {
                        Some(v) => v,
                        None => continue,
                    }
                };

                let mut new_def = def.clone();
                new_def.translations.insert(Lang::new(lang), final_value);
                new_section.definitions.push(new_def);
            }

            result.sections.push(new_section);
        }

        result
    }

    fn fallback_value(&self, def: &Definition, lang: &str) -> Option<String> {
        let fallback = match lang {
            "zh-CN" => Some("zh-Hans"),
            "zh-TW" => Some("zh-Hant"),
            _ => None,
        };
        let mut langs = Vec::new();
        if let Some(fb) = fallback {
            langs.push(Lang::new(fb));
        }
        if let Some(ref dev) = self.options.developer_language {
            langs.push(Lang::new(dev));
        } else if let Some(dev) = self.twine_file.language_codes.first() {
            langs.push(dev.clone());
        }
        def.translation_for(&langs).map(|s| s.to_string())
    }
}