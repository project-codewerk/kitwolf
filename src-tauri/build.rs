fn main() {
  // tauri-build only watches tauri.conf.json, but the icons are embedded at compile time too
  // (exe resource + window icon), so rebuild when they're regenerated with `tauri icon`.
  println!("cargo:rerun-if-changed=icons");
  tauri_build::build()
}
