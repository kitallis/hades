+++
title = "How to call Rust from assembly, and vice versa"
date = "2013-12-01T00:00:00+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2013/12/01/how-to-call-rust-from-assembly/"
+++
In the last few days I&rsquo;ve been working on a kernel in Rust. This has entailed learning about linkers and foreign function interfaces and all kinds of things.
To learn this stuff, I read this guide to linkers, looked at the Rust foreign function interface tutorial, and asked a million questions on the Rust IRC channel.
Disclaimer: even more than usual, some of this is probably wrong.
So. Linkers.