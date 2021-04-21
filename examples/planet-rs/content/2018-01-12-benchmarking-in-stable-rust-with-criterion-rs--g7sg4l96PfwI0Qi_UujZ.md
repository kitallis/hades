+++
title = "Benchmarking In Stable Rust With Criterion.rs"
date = "2018-01-12T19:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/benchmarking-with-criterion-rs/"
+++
When I initially announced the release of Criterion.rs, I didn&rsquo;t expect that there would be so much demand for benchmarking on stable Rust. Now, I&rsquo;d like to announce the release of Criterion.rs 0.1.2, which supports the stable compiler. This post is an introduction to benchmarking with Criterion.rs and a discussion of reasons why you might or might not want to do so.
What is Criterion.rs? Criterion.rs is a benchmarking library for Rust that aims to bring solid statistical confidence to benchmarking Rust code, while maintaining good ease-of-use, even for programmers without a background in statistics.