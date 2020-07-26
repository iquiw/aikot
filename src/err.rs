use failure::Fail;

#[derive(Debug, Fail)]
pub enum AikotError {
    #[fail(display = "invalid environment: {}", name)]
    InvalidEnv { name: String },

    #[fail(display = "command execution fail: {}", stderr)]
    CommandFail { stderr: String },

    #[fail(display = "password file not found: {}", name)]
    PassNotFound { name: String },
}
