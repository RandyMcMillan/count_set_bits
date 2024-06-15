-:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo
all: doc install## 	 all
## make install && cargo t
	cargo test --verbose -- --nocapture
install: release## 	install
## cargo install --path .
	cargo install --path .
build:## 	build
## cargo build
	cargo build
release:## 	release
## cargo build --release
	cargo build --release
doc:## 	doc
## cargo doc
	cargo doc
more:## 	more help
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/	/'

.PHONY:doc
