use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("Glob pattren error")]
    GlobPatternError(#[from] glob::PatternError),

    #[error("Regex pattern error")]
    RegexPatternError(#[from] regex::Error),

    #[error("I/O error")]
    IOError(#[from] std::io::Error),
}
