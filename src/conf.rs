use config::{Config, ConfigError};

pub struct LivePureExecutorConfig {
    like_delay: u16,
    start_page_url: String,
}

impl LivePureExecutorConfig {
    pub fn new(conf: Config) -> Result<Self, ConfigError> {
        let error = Err(ConfigError::Message("like_delay must be positive u16 number".to_string()));
        let like_delay = match  conf.get_int("like_delay") {
            Ok(v) if v > std::u16::MAX as i64 => return error,
            Ok(v) if v <0 => return error,
            Ok(v) => v as u16,
            Err(err) => return Err(err)
        };

        Ok(Self {
            like_delay: like_delay,
            start_page_url: conf.get_string("start_page_url")?,
        })
    }
}
