cfg_if::cfg_if! {
	if #[cfg(target_os = "macos")] {
		mod macos;
		pub use macos::*;
	}
}

cfg_if::cfg_if! {
	if #[cfg(target_os = "linux")] {
		mod linux;
		pub use linux::*;
	}
}