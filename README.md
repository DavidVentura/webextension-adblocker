# Ad blocker for GTK/WebKit2

This is an ad-blocker for WebKit2 based browsers.

## What does it do

This should block all sub-domains from blocked domains, basically `doubleclick.net` should block `pubads.g.doubleclick.net`.

## Building

```
sudo apt install llvm clang libwebkit2gtk-4.0 pkg-config libglib2.0-dev
cargo build --release
```

## How to use it

### Blocking domains

Create a file with **domains or subdomains** you want to block and put it in `~/.config/wk_adblock/hosts.txt`.  

An example:
```
very-bad.domain.com
something-bad.other-domain.com
```

Every request going to these domains or any subdomain (ie: to `really.very-bad.domain.com`) will be blocked.

### Blocking url fragments
Create a file with **url fragments** you want to block and put it in `~/.config/wk_adblock/fragments.txt`.  

An example:
```
-fb-pixel-
-coin-hive.js
```

Every request matching these substrings (ie: to `http://good.com/some-fb-pixel-ad.png`) will be blocked.

## How does it work

WebKit2 does not expose a direct way to block requests, see
[here](https://lists.webkit.org/pipermail/webkit-gtk/2013-March/001395.html). 
You need to build a WebExtension shared object, which webkit [can be instructed to load at runtime](https://github.com/DavidVentura/webextension-adblocker/blob/master/demo.py#L21) and *that WebExtension* can process / reject requests.

This crate implements the necessary bits to reject "bad" requests

Copy-pasted everything from

* https://blogs.igalia.com/carlosgc/2013/09/10/webkit2gtk-web-process-extensions/
* https://github.com/aperezdc/webkit2gtk-python-webextension-example/blob/master/browse-with-extension


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
