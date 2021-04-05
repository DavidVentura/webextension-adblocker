webext.so: trivial_webext.c
	gcc trivial_webext.c `pkg-config --cflags --libs gtk+-3.0,webkit2gtk-4.0` -shared -o webext.so
