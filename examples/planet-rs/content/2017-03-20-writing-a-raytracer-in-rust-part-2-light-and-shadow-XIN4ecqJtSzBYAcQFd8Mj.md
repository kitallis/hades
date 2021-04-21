+++
title = "Writing a Raytracer in Rust - Part 2 - Light and Shadow"
date = "2017-03-20T00:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/"
+++
Welcome to Part 2 of my series on writing a raytracer in Rust. If you haven&rsquo;t already, you may wish to read Part 1. Previously, we implemented a basic raytracer which can render only a single sphere with no lighting. This time, we&rsquo;ll add multiple objects, planes, and basic lighting.
Multiple Objects It&rsquo;s pretty easy to change our scene definition to contain a Vec of spheres instead of just a single one.