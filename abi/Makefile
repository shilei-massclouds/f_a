all: _sys_lib
	cargo rustc --release \
		-- -L ./target/release -l static=sys

_sys_lib:
	cargo rustc --manifest-path ../sys/Cargo.toml --release

run:
	../target/release/abi

clean:
	cargo clean

dump:
	objdump -D ../target/release/abi >/tmp/abi.txt
