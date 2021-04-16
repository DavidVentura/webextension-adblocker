.PHONY: run install
run: libwk_we_adblock.so
	python3 demo.py

libwk_we_adblock.so: ./wk-we-adblock/src/*.rs
	cd wk-we-adblock && cargo build --release
	strip ./wk-we-adblock/target/release/libwk_we_adblock.so
	cp ./wk-we-adblock/target/release/libwk_we_adblock.so .

install:
	strip ./wk-we-adblock/target/release/libwk_we_adblock.so
	mkdir -p $(DESTDIR)/usr/lib/wk-adblock/
	cp ./wk-we-adblock/target/release/libwk_we_adblock.so $(DESTDIR)/usr/lib/wk-adblock/
