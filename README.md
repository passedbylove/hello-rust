* 准备环境
安装source包和xargo
```shell
rustup install nightly
rustup default nightly
rustup add component rust-src
cargo install xargo
```
* 指定openwrt的gcc环境
拷贝toolchain到指定目录，或者在源目录下不动，在bash脚本`~/.bashrc`下编辑指定toolchain的bin目录，供xargo工程使用。

* 编辑build.rs
指定连接时需要使用到的lib库，内容如下:
```rust
use std::env;

fn main() {
    let staging_dir = env::var("STAGING_DIR").unwrap();
    println!(
        r"cargo:rustc-link-search={}/mipsel-openwrt-linux-musl/lib",
        staging_dir
    );
}
```

* 在.cargo/config指定目标
```shell
[build]
target = "mipsel-unknown-linux-gnu"

[target.mipsel-unknown-linux-gnu]
linker = "mipsel-openwrt-linux-musl-gcc"
rustflags = ["-C", "embed-bitcode"]
#rustflags = ["-C", "embed-bitcode", "-C", "prefer-dynamic"]
```
* 下载github代码，放到openwrt的package某目录下
```
https://github.com/likon/hello-rust
```
然后make menuconfig应该会出现hello-rust了。
openwrt should check 
Library->libpcap
Networking->

https://www.cnblogs.com/sanmubai/p/12504795.html