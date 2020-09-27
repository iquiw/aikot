use thiserror::Error;

#[derive(Debug, Error)]
pub enum AikotError {
    #[error("command execution fail: {stderr:}")]
    CommandFail { stderr: String },

    #[error("password file is empty: {name:}")]
    EmptyPassword { name: String },

    #[error("password genaration fail, {pwgen:}")]
    GenerationFail { pwgen: String },

    #[error("gpg or gpg2 command not found")]
    GpgNotFound,

    #[error("invalid environment: {name:}")]
    InvalidEnv { name: String },

    #[error("password less than minimum length: {min_len:} > {pwgen:}")]
    MinimumLength { pwgen: String, min_len: usize },

    #[error("password file already exists: {name:}")]
    PassAlreadyExists { name: String },

    #[error("password file not found: {name:}")]
    PassNotFound { name: String },

    #[error("recipient not found")]
    RecipientNotFound,

    #[error("url field not found: {name:}")]
    UrlNotFound { name: String },
}
