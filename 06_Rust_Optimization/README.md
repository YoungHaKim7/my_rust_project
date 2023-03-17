# Optimizations: the speed size tradeoff

https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html

# pacak/cargo-show-asm

cargo subcommand showing the assembly, LLVM-IR and MIR generated for Rust code

https://github.com/pacak/cargo-show-asm

- Install

```
$ cargo install cargo-show-asm
```

- cargo asm

```
$ cargo asm --lib
Try one of those
"<&T as core::fmt::Display>::fmt" [17, 12, 12, 12, 12, 19, 19, 12]
"<&mut W as core::fmt::Write>::write_char" [20]
"<&mut W as core::fmt::Write>::write_fmt" [38]
"<&mut W as core::fmt::Write>::write_str" [90]
"<F as nom::internal::Parser<I,O,E>>::parse" [263]
#
```

# The Rust Performance Book

https://nnethercote.github.io/perf-book/

Here are some tips to help you optimize your code and squeeze out every last bit of performance!



1. To improve performance, you must first understand the performance characteristics of your program. Criterion is a statistics-driven microbenchmarking library that helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately.


2 .Use the #[inline] attribute to hint to the compiler that a function should be inlined. This can help reduce the overhead of function calls, especially for small functions that are called frequently.

3. Avoid using the Box type unless you need to allocate dynamically sized types on the heap. Prefer using stack-allocated types, such as arrays or tuples, whenever possible. For trait object, instead of Box you can typically get away with using &dyn Trait, which also has dynamic dispatch but saves an allocation.


4. Use Rust's const and static variables to store values that do not change at runtime. Constants are evaluated at compile-time, while statics are stored in the binary and initialized at runtime. Using these variables can improve the performance of your program by allowing the compiler to optimize them more effectively.


<br>


<hr>

# Rust Optimization 

https://gist.github.com/kvark/f067ba974446f7c5ce5bd544fe370186

<br>

<hr>

# How Rust Views Tradeoffs

https://youtu.be/2ajos-0OWts

<br>


# Installation

Clone the repo

```
git clone https://github.com/bheisler/criterion.rs.git
```

Change directories

```
cd criterion
```

Install dependencies and build app

```
cargo build
```

Run the benchmarks

```
cargo bench
    Finished release [optimized] target(s) in 0.09s
     Running target/release/deps/bencher-ef385ec0112399e3

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/my_benchmark-44ff04d7dba46a3e
fib 20                  time:   [17.262 us 17.425 us 17.587 us]
                        change: [-4.2772% -1.0604% +2.3046%] (p = 0.55 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high severe

```

View the plots

```
open target/criterion/report/index.html
```

![1](https://user-images.githubusercontent.com/67513038/223292095-13074f11-4373-41c1-9078-939e2b22b1c0.png)


https://github.com/cgcardona/bencher



# rustc

```
rustc -C opt-level=3 --target aarch64-apple-darwin main.rs
```


```
rustc --print target-list
aarch64-apple-darwin
aarch64-apple-ios
aarch64-apple-ios-macabi
aarch64-apple-ios-sim
aarch64-apple-tvos
aarch64-apple-watchos-sim
aarch64-fuchsia
aarch64-kmc-solid_asp3
aarch64-linux-android
aarch64-nintendo-switch-freestanding
aarch64-pc-windows-gnullvm
aarch64-pc-windows-msvc
aarch64-unknown-freebsd
aarch64-unknown-fuchsia
aarch64-unknown-hermit
aarch64-unknown-linux-gnu
aarch64-unknown-linux-gnu_ilp32
aarch64-unknown-linux-musl
aarch64-unknown-netbsd
aarch64-unknown-none
aarch64-unknown-none-softfloat
aarch64-unknown-nto-qnx710
aarch64-unknown-openbsd
aarch64-unknown-redox
aarch64-unknown-uefi
aarch64-uwp-windows-msvc
aarch64-wrs-vxworks
aarch64_be-unknown-linux-gnu
aarch64_be-unknown-linux-gnu_ilp32
arm-linux-androideabi
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
arm64_32-apple-watchos
armeb-unknown-linux-gnueabi
armebv7r-none-eabi
armebv7r-none-eabihf
armv4t-none-eabi
armv4t-unknown-linux-gnueabi
armv5te-none-eabi
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv5te-unknown-linux-uclibceabi
armv6-unknown-freebsd
armv6-unknown-netbsd-eabihf
armv6k-nintendo-3ds
armv7-apple-ios
armv7-linux-androideabi
armv7-sony-vita-newlibeabihf
armv7-unknown-freebsd
armv7-unknown-linux-gnueabi
armv7-unknown-linux-gnueabihf
armv7-unknown-linux-musleabi
armv7-unknown-linux-musleabihf
armv7-unknown-linux-uclibceabi
armv7-unknown-linux-uclibceabihf
armv7-unknown-netbsd-eabihf
armv7-wrs-vxworks-eabihf
armv7a-kmc-solid_asp3-eabi
armv7a-kmc-solid_asp3-eabihf
armv7a-none-eabi
armv7a-none-eabihf
armv7k-apple-watchos
armv7r-none-eabi
armv7r-none-eabihf
armv7s-apple-ios
asmjs-unknown-emscripten
avr-unknown-gnu-atmega328
bpfeb-unknown-none
bpfel-unknown-none
hexagon-unknown-linux-musl
i386-apple-ios
i586-pc-windows-msvc
i586-unknown-linux-gnu
i586-unknown-linux-musl
i686-apple-darwin
i686-linux-android
i686-pc-windows-gnu
i686-pc-windows-msvc
i686-unknown-freebsd
i686-unknown-haiku
i686-unknown-linux-gnu
i686-unknown-linux-musl
i686-unknown-netbsd
i686-unknown-openbsd
i686-unknown-uefi
i686-uwp-windows-gnu
i686-uwp-windows-msvc
i686-wrs-vxworks
m68k-unknown-linux-gnu
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips-unknown-linux-uclibc
mips64-openwrt-linux-musl
mips64-unknown-linux-gnuabi64
mips64-unknown-linux-muslabi64
mips64el-unknown-linux-gnuabi64
mips64el-unknown-linux-muslabi64
mipsel-sony-psp
mipsel-sony-psx
mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
mipsel-unknown-linux-uclibc
mipsel-unknown-none
mipsisa32r6-unknown-linux-gnu
mipsisa32r6el-unknown-linux-gnu
mipsisa64r6-unknown-linux-gnuabi64
mipsisa64r6el-unknown-linux-gnuabi64
msp430-none-elf
nvptx64-nvidia-cuda
powerpc-unknown-freebsd
powerpc-unknown-linux-gnu
powerpc-unknown-linux-gnuspe
powerpc-unknown-linux-musl
powerpc-unknown-netbsd
powerpc-unknown-openbsd
powerpc-wrs-vxworks
powerpc-wrs-vxworks-spe
powerpc64-ibm-aix
powerpc64-unknown-freebsd
powerpc64-unknown-linux-gnu
powerpc64-unknown-linux-musl
powerpc64-unknown-openbsd
powerpc64-wrs-vxworks
powerpc64le-unknown-freebsd
powerpc64le-unknown-linux-gnu
powerpc64le-unknown-linux-musl
riscv32gc-unknown-linux-gnu
riscv32gc-unknown-linux-musl
riscv32i-unknown-none-elf
riscv32im-unknown-none-elf
riscv32imac-unknown-none-elf
riscv32imac-unknown-xous-elf
riscv32imc-esp-espidf
riscv32imc-unknown-none-elf
riscv64gc-unknown-freebsd
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-linux-musl
riscv64gc-unknown-none-elf
riscv64gc-unknown-openbsd
riscv64imac-unknown-none-elf
s390x-unknown-linux-gnu
s390x-unknown-linux-musl
sparc-unknown-linux-gnu
sparc64-unknown-linux-gnu
sparc64-unknown-netbsd
sparc64-unknown-openbsd
sparcv9-sun-solaris
thumbv4t-none-eabi
thumbv5te-none-eabi
thumbv6m-none-eabi
thumbv7a-pc-windows-msvc
thumbv7a-uwp-windows-msvc
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv7neon-unknown-linux-musleabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
wasm32-unknown-emscripten
wasm32-unknown-unknown
wasm32-wasi
wasm64-unknown-unknown
x86_64-apple-darwin
x86_64-apple-ios
x86_64-apple-ios-macabi
x86_64-apple-tvos
x86_64-apple-watchos-sim
x86_64-fortanix-unknown-sgx
x86_64-fuchsia
x86_64-linux-android
x86_64-pc-nto-qnx710
x86_64-pc-solaris
x86_64-pc-windows-gnu
x86_64-pc-windows-gnullvm
x86_64-pc-windows-msvc
x86_64-sun-solaris
x86_64-unknown-dragonfly
x86_64-unknown-freebsd
x86_64-unknown-fuchsia
x86_64-unknown-haiku
x86_64-unknown-hermit
x86_64-unknown-illumos
x86_64-unknown-l4re-uclibc
x86_64-unknown-linux-gnu
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-musl
x86_64-unknown-netbsd
x86_64-unknown-none
x86_64-unknown-openbsd
x86_64-unknown-redox
x86_64-unknown-uefi
x86_64-uwp-windows-gnu
x86_64-uwp-windows-msvc
x86_64-wrs-vxworks
```
