+++
title = "iou: Rust bindings for liburing"
date = "2019-11-08T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/iou/"
+++
Today I&rsquo;m releasing a library called iou. This library provides idiomatic Rust bindings to the C library called liburing, which itself is a higher interface for interacting with the io_uring Linux kernel interface. Here are the answers to some questions I expect that may provoke.
What is io_uring? io_uring is an interface added to the Linux kernel in version 5.1. Concurrent with that, the primary maintainer of that interface has also been publishing a library for interacting with it called liburing.