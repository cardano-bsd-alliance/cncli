use std::process::Command;
use std::path::Path;
use std::env;

macro_rules! ok (($expression:expr) => ($expression.unwrap()));
macro_rules! log {
    ($fmt:expr) => (println!(concat!("cncli/build.rs:{}: ", $fmt), line!()));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("cncli/build.rs:{}: ", $fmt),
    line!(), $($arg)*));
}

fn main() {
    // Build and link IOHK libsodium
    //run("git", |command| {
    //    command
    //        .arg("submodule")
    //        .arg("update")
    //        .arg("--init")
    //        .arg("--recursive")
    //        .arg("--force")
    //    });

    // Build libsodium automatically (as part of rust build)
    #[cfg(not(feature = "libsodium-sys"))]
    {
        let dir = env::var("SODIUM_LIB_DIR").unwrap();
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());
        println!("cargo:rustc-link-lib=static=sodium");
    }

    // Link with libsodium system library
    //#[cfg(feature = "libsodium-sys")]
    //{
    //    pkg_config::Config::new().probe("libsodium").unwrap();
    //}

    println!("cargo:return-if-changed=build.rs");
}

fn run<F>(name: &str, mut configure: F)
where
    F: FnMut(&mut Command) -> &mut Command,
{
    let mut command = Command::new(name);
    let configured = configure(&mut command);
    log!("Executing {:?}", configured);
    if !ok!(configured.status()).success() {
        panic!("failed to execute {:?}", configured);
    }
    log!("Command {:?} finished successfully", configured);
}
