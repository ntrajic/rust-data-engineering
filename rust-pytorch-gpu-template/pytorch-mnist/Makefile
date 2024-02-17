install:
	python3 -m pip install --upgrade pip \
		&& pip install -r requirements.txt
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

build:
	cargo build

clean:
	#cargo install cargo-cache
	#cargo cache -a
	#rm -rf Cargo.lock
	cargo clean

run:
	cargo run -- conv
	#cargo run --example stable-diffusion --features clap -- --prompt "A rusty robot holding a fire torch." 
	#cargo run --example stable-diffusion-img2img --features clap -- --input-image media/in_img2img.jpg
	

all: format lint test run
