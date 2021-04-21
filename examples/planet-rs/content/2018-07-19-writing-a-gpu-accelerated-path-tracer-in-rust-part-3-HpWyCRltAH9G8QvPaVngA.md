+++
title = "Writing a GPU-Accelerated Path Tracer in Rust - Part 3"
date = "2018-07-19T18:30:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/writing-gpu-accelerated-path-tracer-part-3/"
+++
Hello! Welcome to my third and final post on my GPU-accelerated Path Tracer in Rust. In the last post, we implemented all of the logic necessary to build a true path tracer. Problem is, even on the GPU it&rsquo;s terrifically slow. This post is (mostly) about fixing that.
But first, we need to fix a bug or two, because I goofed. *sad trombone*
Step -1: Fixing Bugs /u/anderslanglands on Reddit pointed out that, since I&rsquo;m using Cosine-weighted Importance Sampling, I need to do some extra math to avoid biasing the results.