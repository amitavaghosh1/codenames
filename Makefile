release:
	cargo build --release
	echo "cp ./target/releases/codename $$PATH/"

