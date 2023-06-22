all:
	cargo rustc --release

run:
	../target/release/func

clean:
	cargo clean

dump:
	objdump -D ../target/release/func >/tmp/func.txt
