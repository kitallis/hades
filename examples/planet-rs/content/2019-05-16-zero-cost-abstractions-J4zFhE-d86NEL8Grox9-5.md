+++
title = "Zero Cost Abstractions"
date = "2019-05-16T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/zero-cost-abstractions/"
+++
The idea of a zero cost abstraction is very important to certain programming languages, like Rust and C++, which intend to enable users to write programs with excellent performance profiles with relatively little effort. Since this idea is fundamental to the design of Rust and my work, I want to investigate, for a moment, what exactly a zero cost abstraction even is.
The idea is summarized in its original by Bjarne Stroustrup, the original developer of C++: