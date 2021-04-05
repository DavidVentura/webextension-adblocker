# Ad blocker for GTK/WebKit2

WebKit2 does not expose a direct way to block requests, see
[here](https://lists.webkit.org/pipermail/webkit-gtk/2013-March/001395.html). 
You need to build a WebExtension shared object, which it'll load at runtime and *that* can process / reject requests.


Copy-pasted everything from

* https://blogs.igalia.com/carlosgc/2013/09/10/webkit2gtk-web-process-extensions/
* https://github.com/aperezdc/webkit2gtk-python-webextension-example/blob/master/browse-with-extension

I am not confident *at all* writing C - so I'm trying to write the shittiest rust lib to determine whether
uris are ads or not.

Docs:

* https://users.rust-lang.org/t/what-is-the-difference-between-dylib-and-cdylib/28847
* https://docs.rust-embedded.org/book/interoperability/rust-with-c.html
* https://doc.rust-lang.org/std/ffi/struct.CString.html
* https://webkitgtk.org/reference/webkit2gtk/stable/WebKitURIRequest.html
