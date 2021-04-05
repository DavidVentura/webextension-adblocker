.PHONY: run
run: webext.so
	python3 demo.py

webext.so: libwk_we_adblock.so trivial_webext.c
	gcc trivial_webext.c ./libwk_we_adblock.so  -Wl,-soname,libwk_we_adblock.so `pkg-config --cflags --libs gtk+-3.0,webkit2gtk-4.0` -shared -o webext.so

libwk_we_adblock.so: ./wk-we-adblock/src/lib.rs
	cd wk-we-adblock && cargo build
	cp ./wk-we-adblock/target/debug/libwk_we_adblock.so .
