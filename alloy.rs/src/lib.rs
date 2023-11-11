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

pub mod logging;

use log::{debug, error, info, trace, warn};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn init_rs_logging() {
    logging::init();
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn test_rs_logging(message: *const libc::c_char) {
    let cstr = unsafe { CStr::from_ptr(message) };
    let string = cstr.to_str().unwrap();

    trace!("{}", &string);
    debug!("{}", &string);
    info!("{}", &string);
    warn!("{}", &string);
    error!("{}", &string);
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_init_rs_logging() {
        init_rs_logging();
    }

    #[test]
    fn test_test_rs_logging() {
        test_rs_logging(CString::new("cool code").unwrap().into_raw());
    }
}

