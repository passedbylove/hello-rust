use std::env;

fn main() {
    let staging_dir = env::var("STAGING_DIR").unwrap();
    println!(
        r"cargo:rustc-link-search={}/mipsel-openwrt-linux-musl/lib",
        staging_dir
    );
}
