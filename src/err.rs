use failure::Fail;

#[derive(Debug, Fail)]
pub enum AikotError {
    #[fail(display = "invalid environment: {}", name)]
    InvalidEnv { name: String },
}
