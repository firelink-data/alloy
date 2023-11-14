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
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://github.com/firelink-data/alloy/actions/workflows/tests.yml/badge.svg)](https://github.com/firelink-data/alloy/actions/workflows/tests.yml)

</div>

## 🔎 Overview
*alloy* is a **Go module that enables calls to Rust code with Apache Arrow datatypes**, and vice versa.

The overarching goal is to enable calls between languages through an underlying **C** interface, utilizing **cgo** and **Rust ffi**. 
This implementation comes with close to no overhead due to using the Apache Arrow data format. The only thing sent between the language binaries are raw data pointers referencing the allocated memory (in Arrow format). This allows for
fast, (somewhat) robust, and colorful use cases in data engineering scenarios.

## 📦 Installation
...

## 🚀 Example usage

The example file [main.go](./main.go) should envision how your Go application could utilize *alloy* to call Rust code.

<details>
    <summary>Show example code</summary>

```go
package main

...

```
    
</details>

## 📋 License
All firelink-data written code is to be held under a general MIT license, please see [LICENSE](https://github.com/firelink-data/alloy/blob/main/LICENSE) for specific information.
All other code included in this repository are to be held under their respective licenses.
