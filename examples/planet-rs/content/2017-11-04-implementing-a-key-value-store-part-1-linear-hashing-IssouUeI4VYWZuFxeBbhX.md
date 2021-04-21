+++
title = "Implementing a key-value store, part 1: Linear Hashing"
date = "2017-11-04T16:38:34+00:00"

[extra]
author = "Samrat Man Singh"
link = "https://samrat.me/posts/2017-11-04-kvstore-linear-hashing/"
+++
In this series of posts, I want to walk you through how to build a simple, persistent key-value store. We will be using an on-disk hashtable(also called external hashtable) to store the key-value mappings. In this post, I will explain hashtables briefly and an algorithm called linear hashing. In the next post, we&rsquo;ll look at an implementation of linear hashing(written in Rust).
How would you store a hashtable on disk?