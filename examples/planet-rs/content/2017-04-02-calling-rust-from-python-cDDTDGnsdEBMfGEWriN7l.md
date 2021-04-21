+++
title = "Calling Rust From Python"
date = "2017-04-02T00:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/calling-rust-in-python/"
+++
Hello! This is a detailed example of exposing Rust code to other languages (in this case, Python). Most articles I&rsquo;ve seen that cover this topic uses really trivial example functions, skipping over a lot of the complexity. Even the better ones out there typically don&rsquo;t have a pre-existing, reasonably complex program to work with. I&rsquo;m going to start with trivial functions and build my way up to being able to define a scene for my raytracer in Python using a series of calls to Rust, then render it and return the resulting image data back to Python.