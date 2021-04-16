# Ad blocker for GTK/WebKit2

WebKit2 does not expose a direct way to block requests, see
[here](https://lists.webkit.org/pipermail/webkit-gtk/2013-March/001395.html). 
You need to build a WebExtension shared object, which it'll load at runtime and *that* can process / reject requests.

This should block all sub-domains from blocked domains, basically `doubleclick.net` should block `pubads.g.doubleclick.net`.

Copy-pasted everything from

* https://blogs.igalia.com/carlosgc/2013/09/10/webkit2gtk-web-process-extensions/
* https://github.com/aperezdc/webkit2gtk-python-webextension-example/blob/master/browse-with-extension

I am not confident *at all* writing C - so I'm trying to write the shittiest rust lib to determine whether
uris are ads or not.

## Docs:

* https://users.rust-lang.org/t/what-is-the-difference-between-dylib-and-cdylib/28847
* https://docs.rust-embedded.org/book/interoperability/rust-with-c.html
* https://doc.rust-lang.org/std/ffi/struct.CString.html
* https://webkitgtk.org/reference/webkit2gtk/stable/WebKitURIRequest.html
* [Generating the bindings from C to rust](https://rust-lang.github.io/rust-bindgen/tutorial-1.html)
* [pkg-config at build time](https://github.com/rust-lang/pkg-config-rs)


## Similar things
* [wyebadblock](https://github.com/jun7/wyebadblock)


## Not-benchmark

Randomly clicking on pages for a while, the average time to decide whether or not a page was an ad was ~30µs.


## Building

```
sudo apt install llvm clang libwebkit2gtk-4.0 pkg-config libglib2.0-dev
```
