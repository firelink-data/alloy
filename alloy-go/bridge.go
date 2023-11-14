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
* File created: 2023-11-14
* Last updated: 2023-11-14
*/

package alloy

import (
    "errors"
    "unsafe"
	"github.com/apache/arrow/go/v14/arrow"
	"github.com/apache/arrow/go/v14/arrow/cdata"
	"github.com/apache/arrow/go/v14/arrow/memory"
)

// #cgo LDFLAGS: -L ${SRCDIR}/clib -lalloy_rs
// #include "./clib/alloy_chunks.h"
import "C"

type Bridge struct {
    GoAllocator *memory.GoAllocator;
}

func (b Bridge) SendArrayAsChunks(arrays []arrow.Array) (int, error) {
    var c_schemas []cdata.CArrowSchema;
    var c_arrays []cdata.CArrowArray;

    for _, array := range arrays {
        c_sch := cdata.CArrowSchema{};
        c_arr := cdata.CArrowArray{};

        cdata.ExportArrowArray(array, &c_arr, &c_sch);

        c_schemas = append(c_schemas, c_sch);
        c_arrays = append(c_arrays, c_arr);
    }

    n_chunks_read := int(C.alloy_read_array_chunks(
        unsafe.Pointer(&c_schemas[0]),
        unsafe.Pointer(&c_arrays[0]),
        C.uintptr_t(len(c_schemas)),
    ))

    if n_chunks_read != len(c_schemas) {
        return n_chunks_read, errors.New("did not read all array chunks!")
    }

    return n_chunks_read, nil
}
