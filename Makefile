all: star-probe star-collect

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
	docker build -t caseydavenport/probe ./probe

collect-image: star-collect
	docker build -t caseydavenport/collect ./collect

dockerhub: probe-image collect-image
	docker push caseydavenport/probe
	docker push caseydavenport/collect

tar: probe-image collect-image 
	docker save caseydavenport/probe > probe.tar
	docker save caseydavenport/collect > collect.tar
