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
All code is to be held under a general MIT license, please see [LICENSE](https://github.com/firelink-data/alloy/blob/main/LICENSE) for specific information.
