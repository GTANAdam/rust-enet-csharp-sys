# Low-level Rust bindings for [ENet-CSharp](https://github.com/nxrighthere/ENet-CSharp)

**Warning:** enet header file 'enet.h' differs from the one found on [ENet-CSharp](https://github.com/nxrighthere/ENet-CSharp).
All of the protocols and functionilties were kept intact but due to some compatiblity woes with [enet-rs](https://github.com/futile/enet-rs), so the IPv6 functionality has been omitted.

Current header version: 2.4.5

### Dependencies
* [LLVM Clang](https://clang.llvm.org)

**Note**: Compiler can be changed to CL (MSVC), GCC or else in the build.rs file.

### Building
```
cargo build
```

### Running examples
```
cargo run --example inittest
cargo run --example host
```
