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

use arrow2::array::Array;
use arrow2::datatypes::{DataType, Field};
use arrow2::ffi;
use libc::c_uint;
use log::info;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn alloy_read_array_chunks(
    sch_ptr: *const ffi::ArrowSchema,
    arr_ptr: *const ffi::ArrowArray,
    n_chunks: usize,
) -> c_uint {
    info!("reading raw pointers passed from C ffi now...");

    let mut data: Vec<Box<dyn Array>> = Vec::with_capacity(n_chunks);

    unsafe {
        for chunk_idx in 0..n_chunks {
            let field = read_field_from_schema_ptr(&sch_ptr.add(chunk_idx).read());
            let chunk = read_data_from_array_ptr(arr_ptr.add(chunk_idx).read(), field.data_type);
            data.push(chunk);
        }
    }

    // TODO: load this data to some DB?
    data.len() as c_uint
}

/// Imports a `Field` from the C data interface.
/// # Safety
/// This function relies on an `ArrowSchema` being valid according to the C data interface (FFI).
/// See [arrow2](https://github.com/jorgecarleitao/arrow2/blob/main/src/ffi/mod.rs) for details.
pub unsafe fn read_field_from_schema_ptr(schema: &ffi::ArrowSchema) -> Field {
    match ffi::import_field_from_c(schema) {
        Ok(f) => f,
        Err(e) => panic!("could not import Field from C ffi: {:?}", e),
    }
}

/// Imports an `Array` from the C data interface.
/// # Safety
/// This function relies on an `ArrowArray` being valid according to the C data interface (FFI).
/// See [arrow2](https://github.com/jorgecarleitao/arrow2/blob/main/src/ffi/mod.rs) for details.
pub unsafe fn read_data_from_array_ptr(
    array: ffi::ArrowArray,
    data_type: DataType,
) -> Box<dyn Array> {
    match ffi::import_array_from_c(array, data_type) {
        Ok(d) => d,
        Err(e) => panic!("could not import data from C ffi: {:?}", e),
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn test_alloy_read_array_chunks_all_empty() {
        let _n_chunks_read: c_uint =
            alloy_read_array_chunks(&ffi::ArrowSchema::empty(), &ffi::ArrowArray::empty(), 1);
    }
}
