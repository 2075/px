dev:
	 		cargo watch -x run

test:
			@echo "run preflight..."
			cargo check
			cargo build --tests

bump: 
			@echo "version..."	
			cargo bump

build:
			@echo "build binary..."	
			cargo build

release:	test bump build

install:	test build
# 			@echo "remove old binary..."
# 			@rm /usr/local/bin/px
			@echo "install binary..."
			@mv target/debug/px /usr/local/bin 
			@echo "ok."

reset:
			@echo "removing px..."
			@rm /usr/local/bin/px
