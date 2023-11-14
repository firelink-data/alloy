/*
* MIT License
*
* Copyright (c) 2023 Firelink Data
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*
* File created: 2023-11-11
* Last updated: 2023-11-14
*/

use chrono::Local;
use env_logger::Builder;
use log::{debug, error, info, trace, warn, LevelFilter};

use std::env;
use std::ffi::CStr;
use std::io::Write;

#[no_mangle]
pub extern "C" fn alloy_init_logging() {
    init();
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn alloy_test_logging(message: *const libc::c_char) {
    let cstr = unsafe { CStr::from_ptr(message) };
    let string = cstr.to_str().unwrap();

    trace!("{}", &string);
    debug!("{}", &string);
    info!("{}", &string);
    warn!("{}", &string);
    error!("{}", &string);
}

fn get_log_level_from_env() -> LevelFilter {
    match env::var("RUST_LOG") {
        Ok(val) => match val.as_str() {
            "OFF" => LevelFilter::Off,
            "TRACE" => LevelFilter::Trace,
            "INFO" => LevelFilter::Info,
            "WARN" => LevelFilter::Warn,
            "WARNING" => LevelFilter::Warn,
            "ERR" => LevelFilter::Error,
            "ERROR" => LevelFilter::Error,
            &_ => LevelFilter::Warn,
        },
        Err(_) => LevelFilter::Warn,
    }
}

pub fn init() {
    let log_level: LevelFilter = get_log_level_from_env();
    match Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [ {} ] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .filter(None, log_level)
        .try_init()
    {
        Ok(()) => {
            debug!("logger initialized");
        }
        Err(e) => {
            error!("tried to initialize logger but failed, {:?}", e);
        }
    };
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use log::{log_enabled, Level};
    use std::ffi::CString;

    #[test]
    fn test_get_log_level_from_env() {
        env::remove_var("RUST_LOG");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Warn, log_level);

        env::set_var("RUST_LOG", "COOL_LOGGING_LEVEL");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Warn, log_level);

        env::set_var("RUST_LOG", "OFF");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Off, log_level);

        env::set_var("RUST_LOG", "TRACE");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Trace, log_level);

        env::set_var("RUST_LOG", "INFO");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Info, log_level);

        env::set_var("RUST_LOG", "WARN");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Warn, log_level);

        env::set_var("RUST_LOG", "WARNING");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Warn, log_level);

        env::set_var("RUST_LOG", "ERR");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Error, log_level);

        env::set_var("RUST_LOG", "ERROR");
        let log_level: LevelFilter = get_log_level_from_env();
        assert_eq!(LevelFilter::Error, log_level);
    }

    #[test]
    #[ignore]
    fn test_init() {
        env::set_var("RUST_LOG", "TRACE");
        init();
        assert!(log_enabled!(Level::Trace));
    }

    #[test]
    fn test_alloy_init_logging() {
        alloy_init_logging();
    }

    #[test]
    fn test_alloy_test_logging() {
        alloy_test_logging(CString::new("cool logging test").unwrap().into_raw());
    }
}
