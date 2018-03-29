extern crate fs2;
extern crate log4rs;
extern crate simple_logger;
extern crate toml;

use self::fs2::FileExt;
use failure::Error;
use std;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;
use structopt::StructOpt;

type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = std::result::Result<T, Error>;

// remove allow in actual code
#[allow(dead_code)]
#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Configuration initialization error: {}", _0)]
    InitConf(#[cause] std::io::Error),

    #[fail(display = "Dummy error")]
    Dummy,

    #[fail(display = "Pos error: {}", _0)]
    Pos(usize),

    #[fail(display = "Status error code: {}, msg: {}", code, msg)]
    DummyStatus { code: u32, msg: String },
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::InitConf(err)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "App config", about = "Configuration for Rust HDFS-to-Local")]
pub struct ArgConf {
    #[structopt(
        short = "c",
        long = "conf",
        default_value = "config/app.toml",
        help = "Configuration file path"
    )]
    pub conf: String,
}

#[derive(Deserialize, Debug)]
pub struct FileConf {
    pub log_conf_path: Option<String>,
    pub lock_file_path: String,
}

pub fn init_conf() -> Result<FileConf> {
    let arg_conf = ArgConf::from_args();
    let file_conf: FileConf = toml::from_str(&read_from_file(&arg_conf.conf)?)?;

    match file_conf.log_conf_path {
        Some(ref log_conf_path) => {
            log4rs::init_file(log_conf_path, Default::default())?
        }
        None => simple_logger::init()?,
    }

    Ok(file_conf)
}

pub fn lock_file<P: AsRef<Path>>(path: P) -> Result<File> {
    let flock = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path.as_ref())?;

    flock.try_lock_exclusive()?;
    Ok(flock)
}

pub fn read_from_file<P: AsRef<Path>>(p: P) -> StdResult<String, AppError> {
    let mut buf = String::new();
    let mut file = File::open(p.as_ref())?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn print_run_status<T>(res: &Result<T>) {
    match *res {
        Ok(_) => info!("Session completed!"),
        Err(ref e) => error!(
            "ERROR: {}\n > BACKTRACE: {}",
            e.cause(),
            e.backtrace()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_conf() {
        const CONF: &str = r#"
            log_conf_path = 'config/log.toml'
            lock_file_path = '/var/lock/app.lock'
        "#;

        let file_conf = toml::from_str::<FileConf>(CONF);
        assert!(file_conf.is_ok());
    }
}
