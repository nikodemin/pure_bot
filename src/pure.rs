use std::error::Error;
use thirtyfour::prelude::*;
use crate::conf;

mod executor {
    use thirtyfour::WebDriver;

    pub trait PureExecutor {
        fn authorize(&self);
        fn like(&self);
        fn get_match_count(&self) -> u8;
    }

    struct LivePureExecutor {
        match_count: u8,
        web_driver: WebDriver,
        config: conf::LiveConfig
    }
    
    impl LivePureExecutor {
        fn new(driver: WebDriver, config: LiveConfig) -> Self {
            Self {
                match_count: 0,
                web_driver: driver,
                config: config
            }
        }
    }

    impl PureExecutor for LivePureExecutor {
        fn authorize(&self) {
            todo!()
        }

        fn like(&self) {
            todo!()
        }

        fn get_match_count(&self) -> u8 {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn like() {}
    }
}

pub mod bot {
    pub trait PureBot {
        fn start(&self);
        fn stop(&self);
        fn set_match_limit(&self, limit: u8);
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn like() {}
    }
}
