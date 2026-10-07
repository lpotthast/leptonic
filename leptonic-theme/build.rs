// The stylesheets are embedded with `include_dir!`, which doesn't tell Cargo about them (0.7): rebuild
// when any of them changes, so apps' generated `style-dir` follows theme edits.
fn main() {
    println!("cargo:rerun-if-changed=scss");
}
