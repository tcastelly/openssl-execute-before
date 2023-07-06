use chrono::{DateTime, LocalResult, Utc};

#[cfg_attr(test, mockall::automock)]
pub trait Parse {
    fn parse(&self) -> Result<LocalResult<DateTime<Utc>>, String>;
}

#[cfg_attr(test, mockall::automock)]
pub trait OpenSSHCmd {
    fn get_expiration_cert(&self, ca: &str) -> Box<dyn Parse + 'static>;
}
