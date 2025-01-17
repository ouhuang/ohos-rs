use std::env;

pub fn setup() {
  let ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set.");
  let name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME should not be empty.");
  let target = env::var("TARGET").expect("Try to get build target failed.");

  let lib_dir = match target.as_ref() {
    "aarch64-unknown-linux-ohos" => "aarch64-linux-ohos",
    "armv7-unknown-linux-ohos" => "arm-linux-ohos",
    "x86_64-unknown-linux-ohos" => "x86_64-linux-ohos",
    _ => "",
  };

  println!("cargo:rustc-env=NAPI_BUILD_TARGET_NAME={}", name.as_str());
  // for zig-build avoid to use RUSTFLAGS="-L xxxx"
  println!(
    "cargo:rustc-link-search={}/native/sysroot/usr/lib/{}",
    ndk, lib_dir
  );
  // link libace_napi.z.so
  println!("cargo:rustc-link-lib=dylib=ace_napi.z");
}
