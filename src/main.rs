// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate tera;

#[macro_use]
extern crate juniper;

extern crate actix;
extern crate actix_web;

extern crate fern;
extern crate regex;
extern crate serde;
extern crate chrono;
extern crate futures;
extern crate serde_json;
extern crate serde_yaml;

pub mod server;

mod errors {
  error_chain! {
    foreign_links {
      Serde(::serde_json::Error);
      Log(::log::SetLoggerError);
      Io(::std::io::Error);
    }
  }
}

use self::errors::*;

/// Setup logging
fn setup_logger() -> Result<()> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.level(),
        message
      ))
    })
    .level(log::LevelFilter::Debug)
    .level_for("tokio_reactor", log::LevelFilter::Warn)
    .level_for("actix_web::server::server", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .chain(fern::log_file("output.log")?)
    .apply()?;

  Ok(())
}

fn main() {
  if let Err(ref err) = run() {
    use std::io::Write;

    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "Corgi encountered one or more errors:").expect(errmsg);
    for err in err.iter().skip(1) {
      writeln!(stderr, "  - {}", err).expect(errmsg);
    }

    if let Some(backtrace) = err.backtrace() {
      writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }

    ::std::process::exit(1);
  }
}

fn run() -> Result<()> {
  setup_logger()?;

  let sys = actix::System::new("corgi");
  server::start("127.0.0.1:8080")?;
  let _ = sys.run();

  Ok(())
}
