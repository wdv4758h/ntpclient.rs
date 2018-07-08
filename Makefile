no_std:
	cargo rustc --no-default-features --features no_std -- -Z pre-link-arg=-nostartfiles

std:
	cargo build
