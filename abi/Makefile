OUT_DIR := $(CURDIR)/../target/x86_64-unknown-none/release
LIB_ARCHIVE := $(OUT_DIR)/libsys.a

all: _sys_lib
	ls -l $(OUT_DIR)
	cargo rustc --release -- -L $(OUT_DIR) -l static=sys -Clink-args="-no-pie"

_sys_lib:
	cargo rustc --release \
		--manifest-path ../sys/Cargo.toml \
		--target x86_64-unknown-none
	cp $(CURDIR)/libsys.redefine-syms.template $(CURDIR)/libsys.redefine-syms
	objdump -t $(LIB_ARCHIVE) | grep "sys_call_" | \
		awk -F '[ ]+' '{print $$5}' | \
		xargs -Isymbol echo 'libsys_symbol symbol' \
		>> $(CURDIR)/libsys.redefine-syms
	objcopy --prefix-symbols=libsys_ $(LIB_ARCHIVE)
	objcopy --redefine-syms=$(CURDIR)/libsys.redefine-syms $(LIB_ARCHIVE)

run:
	../target/release/abi

clean:
	@rm -f $(LIB_ARCHIVE)
	@rm -f *.redefine-syms
	cargo clean

dump:
	objdump -D ../target/release/abi >/tmp/abi.txt
