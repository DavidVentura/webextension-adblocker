#! /usr/bin/env python3
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright (C) 2015-2016 Adrian Perez <aperez@igalia.com>
# Copyright (C) 2016 Nathan Hoad <nathan@getoffmalawn.com>
#
# Distributed under terms of the MIT license.

import gi
gi.require_version("WebKit2", "4.0")

from gi.repository import WebKit2, Gtk, GLib
from os import path
import sys

mydir = path.abspath(path.dirname(__file__))
print("Extension directory:", mydir)

ctx = WebKit2.WebContext.get_default()
ctx.set_web_extensions_directory(mydir)
#ctx.set_web_extensions_initialization_user_data(GLib.Variant.new_string("test string"))

wnd = Gtk.Window()
web = WebKit2.WebView.new_with_context(ctx)
wnd.connect("destroy", Gtk.main_quit)
wnd.add(web)
wnd.set_default_size(1152, 800)
wnd.show_all()

def on_load_failed(webview, event, url, error):
    print("Error loading", url, "-", error)

web.connect("load-failed", on_load_failed)

if len(sys.argv) > 1:
    web.load_uri(sys.argv[1])
else:
    web.load_uri("http://ddg.gg")

Gtk.main()

