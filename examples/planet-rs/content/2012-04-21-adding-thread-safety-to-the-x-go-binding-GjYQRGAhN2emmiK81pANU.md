+++
title = "Adding Thread Safety to the X Go Binding"
date = "2012-04-21T20:52:00-05:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/thread-safety-x-go-binding/"
+++
<p>The <a href="http://code.google.com/p/x-go-binding/">X Go Binding (XGB)</a> is a low level
library that provides an API to interact with running X servers. One can only
communicate with an X server by sending data over a network connection;
protocol requests, replies and errors need to be perfectly constructed down to
the last byte. Xlib did precisely this, and then some. As a result, Xlib became
huge and difficult to maintain.</p>
<p>In recent years, the <a href="http://xcb.freedesktop.org/">XCB project</a> made things a
bit more civilized by generating C code from <a href="http://cgit.freedesktop.org/xcb/proto/tree/src">XML
files</a> describing the X client
protocol using Python. While <a href="http://cgit.freedesktop.org/xcb/libxcb/tree/src/c_client.py">the Python to generate said
code</a> is no walk
in the park, it is typically preferred to the alternative: keeping the X core
protocol up to date along with any number of extensions that exist as well.
(There are other benefits to XCB, like easier asynchronicity, but that is
beyond the scope of this post.)</p>
<p>XGB proceeds in a similar vain; <a href="http://code.google.com/r/jamslam-x-go-binding/source/browse/xgb/go_client.py">Python is used to generate Go
code</a>
that provides an API to interact with the X protocol. Unlike its sister project
XCB, it is not thread safe. In particular, if X requests are made in parallel,
the best case scenario is a jumbled request or reply and the worst case
scenario is an X server crash. Parallel requests can be particularly useful
when images are being sent to the X server to be painted on the screen; other
useful work could be done in the interim.</p>