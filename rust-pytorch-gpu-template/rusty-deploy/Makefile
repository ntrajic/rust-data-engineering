install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	@echo "Formatting all projects with cargo"
	./format.sh

lint:
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	./lint.sh

test:
	export LD_LIBRARY_PATH=/usr/local/lib/libtorch/lib:${LD_LIBRARY_PATH}
	@echo "Testing all projects with cargo"
	./test.sh --lib /usr/local/lib/libtorch/lib/libtorch_cpu.so

check-gpu-linux:
	sudo lshw -C display


all: format lint test