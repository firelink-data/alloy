<div align="center">
<br/>
<div align="left">
<br/>
<p align="center">
<a href="https://github.com/wilhelmagren/finq">
<img align="center" width=75% src="https://github.com/firelink-data/alloy/blob/cd83e690cc0d73c42f2928bc7baefbcfc400dc24/docs/images/alloy-banner.png"></img>
</a>
</p>
</div>

[![Go Reference](https://pkg.go.dev/badge/github.com/firelink-data/alloy/alloy-go.svg)](https://pkg.go.dev/github.com/firelink-data/alloy/alloy-go)
[![Crates.io (latest)](https://img.shields.io/crates/v/alloy-rs)](https://crates.io/crates/alloy-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Go CI](https://github.com/firelink-data/alloy/actions/workflows/go_ci.yml/badge.svg)](https://github.com/firelink-data/alloy/actions/workflows/go_ci.yml)
[![Rust CI](https://github.com/firelink-data/alloy/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/firelink-data/alloy/actions/workflows/rust_ci.yml)
[![Rust tests](https://github.com/firelink-data/alloy/actions/workflows/rust_tests.yml/badge.svg)](https://github.com/firelink-data/alloy/actions/workflows/rust_tests.yml)
</div>

## 🔎 Overview
*alloy* is a standalone Go module that **enables calls to Rust code with Apache Arrow datatypes** through its defined C data interface.

The overarching goal is to enable calls between languages through an underlying **C** interface, in this case utilizing **cgo** and the **Rust ffi**. 
This implementation comes with **no overhead** due to using the [Apache Arrow](https://arrow.apache.org/) data format. 

The only data sent between the language binaries are **raw pointers** referencing the allocated memory and associated schemas. 
This allows for fast, (somewhat) robust, and colorful use cases in data engineering scenarios.

## 📦 Installation

To use the Go module, simply include it in your import with path name `github.com/firelink-data/alloy/alloy-go`. 
If you want to install any Go binary utilizing *alloy*, you need to enable **cgo** compilation by setting `CGO_ENABLED=0`.

To add the static Rust crate to your own Rust code, you can use the cargo package manager:
```
$ cargo add alloy-rs
```

## 🚀 Example usage

The example file [examples/main.go](https://github.com/firelink-data/alloy/blob/main/examples/main.go) should envision how your Go application could utilize *alloy* to call Rust code with Apache Arrow parameters.

<details>
    <summary>Show example code</summary>

```go
package main

import (
    "fmt"
    "github.com/firelink-data/alloy/alloy-go"
)

func main() {
    fmt.Println("Hello from Go!");

    alloy.InitLogging();
    alloy.TestLogging("Hello Rust, sent from Golang!");

    ...

    fmt.Println("Goodbye from Go!");
}

```
    
</details>

## 📋 License
All code written by **Firelink Data** is to be held under a general MIT license, please see [LICENSE](https://github.com/firelink-data/alloy/blob/main/LICENSE) for specific information.

Any other code included in this repository is to be held under its respective license(s).
