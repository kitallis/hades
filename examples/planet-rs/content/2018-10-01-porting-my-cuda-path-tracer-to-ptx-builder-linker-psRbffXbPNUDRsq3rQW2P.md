+++
title = "Porting My CUDA Path Tracer to ptx-builder/linker"
date = "2018-10-01T19:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/ptx-builder-and-linker/"
+++
A bunch of stuff has happened since I published my post on The State of GPGPU in Rust. Most importantly, Denys Zariaiev (@denzp) released his work on a custom linker for Rust CUDA kernels, and a build.rs helper crate to make it easier to use.
These two crates eliminate many of the problems I referred to in my previous post. The linker solves most of the &ldquo;invalid PTX file&rdquo; problems, while the ptx-builder crate does all of the magic that Accel was doing behind the scenes.