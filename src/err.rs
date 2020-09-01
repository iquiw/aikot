use failure::Fail;

#[derive(Debug, Fail)]
pub enum AikotError {
    #[fail(display = "command execution fail: {}", stderr)]
    CommandFail { stderr: String },

    #[fail(display = "password file is empty: {}", name)]
    EmptyPassword { name: String },

    #[fail(display = "password genaration fail, {}", pwgen)]
    GenerationFail { pwgen: String },

    #[fail(display = "gpg or gpg2 command not found")]
    GpgNotFound,

    #[fail(display = "invalid environment: {}", name)]
    InvalidEnv { name: String },

    #[fail(display = "password less than minimum length: {} > {}", min_len, pwgen)]
    MinimumLength { pwgen: String, min_len: usize },

    #[fail(display = "password file already exists: {}", name)]
    PassAlreadyExists { name: String },

    #[fail(display = "password file not found: {}", name)]
    PassNotFound { name: String },

    #[fail(display = "recipient not found")]
    RecipientNotFound,
}
