+++
title = "Writing a GPU-Accelerated Path Tracer in Rust - Part 2"
date = "2018-07-12T19:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/writing-gpu-accelerated-path-tracer-part-2/"
+++
Hello, and welcome to part two of my series on writing a GPU-accelerated path tracer in Rust. I&rsquo;d meant to have this post up sooner, but nothing ruins my productivity quite like Games Done Quick. I&rsquo;m back now, though, so it&rsquo;s time to turn the GPU ray-tracer from the last post into a real path tracer.
Tracing Paths As mentioned last time, Path Tracing is an extension to Ray Tracing which attempts to simulate global illumination.