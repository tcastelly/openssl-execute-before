use chrono::{DateTime, LocalResult, Utc};

pub trait Parse {
    fn parse(&self) -> Result<LocalResult<DateTime<Utc>>, String>;
}

pub trait OpenSSHCmd {
    fn get_expiration_cert(&self, ca: &str) -> Box<dyn Parse + 'static>;
}
