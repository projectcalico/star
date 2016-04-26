all: star-probe star-collect
images: probe-image collect-image frontend backend

clean:
	cargo clean

test:
	cargo test

star-probe:
	cargo build --bin star-probe
	cp target/debug/star-probe ./probe

star-collect:
	cargo build --bin star-collect
	cp target/debug/star-collect ./collect

probe-image: star-probe 
	docker build -t calico/star-probe ./probe

collect-image: star-collect
	docker build -t calico/star-collect ./collect
