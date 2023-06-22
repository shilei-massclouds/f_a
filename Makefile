all: build

build: clean
	@make -C abi
	@make -C func

run: build
	@make -C abi run
	@make -C func run

clean:
	@make -C abi clean
	@make -C func clean

dump: build
	@make -C abi dump
	@make -C func dump
