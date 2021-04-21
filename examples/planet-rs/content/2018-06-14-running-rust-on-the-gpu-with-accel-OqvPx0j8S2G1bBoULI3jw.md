+++
title = "Running Rust on the GPU with Accel"
date = "2018-06-14T19:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/rust-on-the-gpu-with-accel/"
+++
NOTE: Much of what I discuss below is no longer accurate.
For the past month or so, I&rsquo;ve been working on a follow-up to my series on Writing a Raytracer in Rust. This time around, I&rsquo;ll be talking about writing a GPU-accelerated Path Tracer. As always, I&rsquo;m writing it in Rust - including the GPU kernel code. Compiling Rust for GPUs at this point is difficult and error-prone, so I thought it would be good to start with some documentation on that aspect of the problem before diving into path tracing.