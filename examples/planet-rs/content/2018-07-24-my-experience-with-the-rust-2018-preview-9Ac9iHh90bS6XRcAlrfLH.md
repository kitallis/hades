+++
title = "My experience with the Rust 2018 preview"
date = "2018-07-24T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/my-experience-with-rust-2018/"
+++
Recently, I wrote a little a side project to sign git commits without gpg. When I did this, I decided to use the Rust 2018 edition. I also transitioned an existing library from Rust 2015 to Rust 2018 to see how that tooling worked. I thought I&rsquo;d write a blog post about my experience using the Rust 2018 preview and the state of things right now.
Module changes The main thing I noticed vividly was the new experience of the module system.