//! Embeds the CI build number (`UOF_BUILD`, set to `github.run_number`) in the binary.
//! Local builds without it get 0, meaning a "dev" build.
fn main() {
    println!("cargo:rerun-if-env-changed=UOF_BUILD");
    let build = std::env::var("UOF_BUILD")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);
    println!("cargo:rustc-env=UOF_BUILD={build}");
}
