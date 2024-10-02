use config::Config;
use std::error::Error;

mod conf;
mod pure;
mod tg;

fn main() -> Result<(), Box<dyn Error>> {
    let conf = Config::builder()
        .add_source(config::File::with_name("conf.toml"))
        .build()?;

    let pureExecutorConf = conf::LivePureExecutorConfig::new
}
