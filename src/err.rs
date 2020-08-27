use failure::Fail;

#[derive(Debug, Fail)]
pub enum AikotError {
    #[fail(display = "command execution fail: {}", stderr)]
    CommandFail { stderr: String },

    #[fail(display = "password file is empty: {}", name)]
    EmptyPassword { name: String },

    #[fail(
        display = "password genaration fail, length: {}, pwclass: {}",
        length, pwclass
    )]
    GenerationFail { pwclass: String, length: usize },

    #[fail(display = "invalid environment: {}", name)]
    InvalidEnv { name: String },

    #[fail(
        display = "password less than minimum length: {}, class: {}",
        length, pwclass
    )]
    MinimumLength { pwclass: String, length: usize },

    #[fail(display = "password file already exists: {}", name)]
    PassAlreadyExists { name: String },

    #[fail(display = "password file not found: {}", name)]
    PassNotFound { name: String },

    #[fail(display = "recipient not found")]
    RecipientNotFound,
}
