export RUSTC_BOOTSTRAP=1

all: build

build: clean
	@make -C func
	@make -C abi

run: build
	@make -C func run
	@make -C abi run

clean:
	@make -C func clean
	@make -C abi clean

dump: build
	@make -C func dump
	@make -C abi dump
