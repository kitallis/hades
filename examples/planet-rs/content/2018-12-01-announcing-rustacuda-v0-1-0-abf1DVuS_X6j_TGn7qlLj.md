+++
title = "Announcing RustaCUDA v0.1.0"
date = "2018-12-01T15:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/announcing-rustacuda/"
+++
In my post on GPGPU in Rust, I declared that I intended to work on improving the state of CUDA support in Rust. Since then, I&rsquo;ve been mostly radio-silent. I&rsquo;m pleased to announce that I have actually been working on something, and I&rsquo;ve finally published that something.
RustaCUDA RustaCUDA is a new wrapper crate for the CUDA driver API. It allows the programmer to allocate and free GPU memory, copy data to and from the GPU, load CUDA modules and launch kernels, all with a mostly-safe, programmer-friendly, Rust-y interface.