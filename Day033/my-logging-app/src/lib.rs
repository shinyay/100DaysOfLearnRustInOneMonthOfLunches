use anyhow::{anyhow, Result};
use log::{LevelFilter, trace, info};
use simple_logger::SimpleLogger;
use spin_sdk::{
    http::{Request, Response},
    http_component, config,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_my_logging_app(req: Request) -> Result<Response> {
    init_logger()?;
    trace!("method={}, uri={}", req.method(), req.uri());
    info!("returning 200");

    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}

fn init_logger() -> Result<()> {
    const LOG_LEVEL_CONFIG_VARIABLE: &str = "loglevel";

    let level: LevelFilter = config::get(LOG_LEVEL_CONFIG_VARIABLE)?
        .parse()
        .map_err(|e| anyhow!("parsing log level: {e}"))?;

    SimpleLogger::new()
        .with_level(level)
        .init()?;

    Ok(())
}
