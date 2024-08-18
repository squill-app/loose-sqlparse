/// Parser options.
pub struct Options {
    /// The delimiter used to separate statements.
    /// The default is `;`.
    pub statement_delimiter: String,
}

impl Default for Options {
    fn default() -> Self {
        Self { statement_delimiter: ";".to_string() }
    }
}
