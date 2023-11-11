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
* Last updated: 2023-11-11
*/

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use std::env;
use std::io::Write;

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

    Builder::new()
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
        .init();
}
