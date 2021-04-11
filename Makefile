.PHONY: run
run: libwk_we_adblock.so
	python3 demo.py

libwk_we_adblock.so: ./wk-we-adblock/src/lib.rs
	cd wk-we-adblock && cargo build --release
	strip ./wk-we-adblock/target/release/libwk_we_adblock.so
	cp ./wk-we-adblock/target/release/libwk_we_adblock.so .
