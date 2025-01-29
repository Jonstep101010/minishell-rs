#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
	// add unix dependencies below
	// println!("cargo:rustc-flags=-l readline");
	println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup"); // Add additional linker flags
}

#[cfg(target_os = "macos")]
fn main() {
	// Add the necessary linker flags for macOS
	//  println!("cargo:rustc-link-lib=c");
	//  println!("cargo:rustc-link-lib=readline");
	//  println!("cargo:rustc-link-lib=iconv");

	//  // Add the library path for the Readline library
	//  if let Ok(readline_path) = std::process::Command::new("brew")
	//      .arg("--prefix")
	//      .arg("readline")
	//      .output()
	//  {
	//      let readline_path = String::from_utf8(readline_path.stdout).unwrap();
	//      println!("cargo:rustc-link-search=native={}/lib", readline_path.trim());
	//  }
	// // Add additional linker flags
	println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup"); // Add additional linker flags
}
