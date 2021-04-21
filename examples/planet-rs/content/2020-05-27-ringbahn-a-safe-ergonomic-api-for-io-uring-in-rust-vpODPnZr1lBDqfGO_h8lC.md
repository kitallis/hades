+++
title = "Ringbahn: a safe, ergonomic API for io-uring in Rust"
date = "2020-05-27T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/ringbahn/"
+++
In my previous post, I discussed the new io-uring interface for Linux, and how to create a safe API for using io-uring from Rust. In the time since that post, I have implemented a prototype of such an API. The crate is called ringbahn, and it is intended to enable users to perform IO on io-uring without any risk of memory unsafety.
io-uring is going to be majorly important to the development of async IO on Linux in the future, and Linux is the most used platform for the kinds of high performance network services that are a major user of Rust.