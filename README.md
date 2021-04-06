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


## Similar things
* [wyebadblock](https://github.com/jun7/wyebadblock)


## Not-benchmark

The initial dumb for loop without trie takes
```
Took 54.412µs
Took 85.47µs
Took 75.35µs
Took 46.748µs
Took 42.448µs
Took 75.821µs
Took 64.25µs
Took 43µs
Took 40.867µs
Took 39.735µs
Took 43.802µs
Took 32.691µs
Took 39.635µs
Took 42.599µs
Took 44.693µs
```

in release mode, and 

```
Took 559.737µs
Took 517.137µs
Took 571.69µs
Took 554.316µs
Took 557.162µs
Took 588.81µs
Took 483.575µs
Took 600.725µs
Took 691.013µs
Took 694.9µs
Took 548.886µs
Took 535.332µs
Took 442.407µs
Took 443.119µs
Took 929.147µs
```

in debug mode.

and with a trie, in release mode:
```
Took 41.879µs
Took 33.202µs
Took 37.661µs
Took 58.4µs
Took 40.547µs
Took 37.511µs
Took 47.739µs
Took 35.186µs
Took 45.295µs
Took 11.842µs
Took 40.666µs
Took 35.576µs
Took 34.604µs
```
