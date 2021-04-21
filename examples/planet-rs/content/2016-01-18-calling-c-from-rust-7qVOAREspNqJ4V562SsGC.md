+++
title = "Calling C from Rust"
date = "2016-01-18T09:31:42+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2016/01/18/calling-c-from-rust/"
+++
Yesterday I asked Kamal how to call C code from Rust, for a project I&rsquo;m thinking about. It turned out to be a little harder than I expected! Largely because I don&rsquo;t know Rust well, and fixing compiler errors is nontrivial. 30 minutes and some number of inscrutable-to-me compiler errors later, we figured it out.
I want to do something pretty simple &ndash; copy the string &ldquo;Hello, world!&rdquo; and print it.