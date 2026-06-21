/// Parameters for the `generate` operation.
pub struct GenerateParams {
    pub twine_file: String,
    pub output_path: String,
    pub developer_language: Option<String>,
    pub encoding: Option<String>,
    pub escape_all_tags: bool,
    pub format: Option<String>,
    pub include: String,
    pub quiet: bool,
    pub tags: Vec<String>,
    pub untagged: bool,
    pub validate: bool,

    // ── mode-specific ──
    /// Generate all localization files to a directory.
    pub all: bool,
    /// Generate a zip archive.
    pub archive: bool,
    /// Filter output to specific languages.
    pub languages: Vec<String>,
    /// Create output directories if they don't exist (--all mode).
    pub create_folders: bool,
    /// Custom output file name (--all mode).
    pub file_name: Option<String>,
}

/// Parameters for the `consume` operation.
pub struct ConsumeParams {
    pub twine_file: String,
    pub input_path: String,
    pub consume_all: bool,
    pub consume_comments: bool,
    pub developer_language: Option<String>,
    pub encoding: Option<String>,
    pub format: Option<String>,
    pub output_path: Option<String>,
    pub quiet: bool,
    pub tags: Vec<String>,

    // ── mode-specific ──
    /// Consume all localization files from a directory.
    pub all: bool,
    /// Filter to specific languages (single-file mode).
    pub languages: Vec<String>,
}

/// Parameters for the `validate` operation.
pub struct ValidateParams {
    pub twine_file: String,
    pub developer_language: Option<String>,
    pub pedantic: bool,
    pub quiet: bool,
}
