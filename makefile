contracts_dir=contracts
compiled_dir=compiled
checksum_dir=${compiled_dir}/checksum

build-release=RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
build-debug=RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown  # --features="debug-print"  --locked

# args (no extensions): wasm_name, contract_dir_name
define opt_and_compress = 
# wasm-opt -Oz ./target/wasm32-unknown-unknown/release/$(2).wasm -o ./$(1).wasm
echo $(md5sum $(1).wasm | cut -f 1 -d " ") >> ${checksum_dir}/$(1).txt
cat ./$(1).wasm | gzip -n -9 > ${compiled_dir}/$(1).wasm.gz
rm ./$(1).wasm
endef

CONTRACTS = snip20 lp_token factory amm_pair router staking 
debug: setup
	(cd ${contracts_dir}; ${build-debug})
	@$(MAKE) compress_all

release: setup
	(cd ${contracts_dir}; ${build-release})
	@$(MAKE) compress_all

compress_all: setup
	@$(MAKE) $(addprefix compress-,$(CONTRACTS))

compress-snip20: setup
	$(call opt_and_compress,snip20,snip20_reference_impl)

compress-%: setup
	$(call opt_and_compress,$*,$*)

$(CONTRACTS): setup
	(cd ${contracts_dir}/$@; ${build-debug})
	@$(MAKE) $(addprefix compress-,$(@))

snip20: setup
	(cd ${contracts_dir}/snip20; ${build-release})
	@$(MAKE) $(addprefix compress-,snip20)

test:
	@$(MAKE) $(addprefix test-,$(CONTRACTS))

test-%:
	(cd ${contracts_dir}/$*; cargo unit-test)

setup: $(compiled_dir) $(checksum_dir)

$(compiled_dir) $(checksum_dir):
	mkdir $@

check:
	cargo check

clippy:
	cargo clippy

clean:
	rm -r $(compiled_dir)

format:
	cargo fmt

# Downloads the docker server
server-download:
	docker pull cryptobrokersglobal/localsecret:beta-nested-attributes

# Starts the docker server / private testnet
server-start:
	docker run -it --rm -p 9091:9091 -p 26657:26657 -p 1317:1317 -p 5000:5000 -v $$(pwd):/root/code --name shade-testnet cryptobrokersglobal/localsecret:v1.5.1-patch.3

server-start-background:
	docker run  -p 9091:9091 -p 26657:26657 -d -p 1317:1317 -p 5000:5000 -v $$(pwd):/root/code --name shade-testnet cryptobrokersglobal/localsecret:beta-nested-attributes
# Connects to the docker server
server-connect:
	docker exec -it shade-testnet /bin/bash

# Runs integration tests
integration-tests:
	cargo test run_testnet -- --nocapture --test-threads=1

	
deploy-test:
	cargo test run_test_deploy -- --nocapture --test-threads=1

build-deploy:
	cargo build --manifest-path ./packages/network_integration/Cargo.toml

deploy:
	./target/debug/deploy


