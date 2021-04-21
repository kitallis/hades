+++
title = "Daemonizing Go Programs (with a BSD-style rc.d example)"
date = "2012-04-27T21:12:00-05:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/golang-daemonize-bsd/"
+++
<p>Go, by its very nature, is multithreaded. This makes a traditional approach of
daemonizing Go programs by forking a bit difficult.</p>
<p>To get around this, you could try something as simple as backgrounding your Go
program and instructing it to <a href="http://en.wikipedia.org/wiki/Nohup">ignore the HUP
signal</a>:</p>