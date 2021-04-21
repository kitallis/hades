+++
title = "Implementing a key-value store, part 2: Linear Hashing implementation in Rust"
date = "2017-11-09T00:00:00+00:00"

[extra]
author = "Samrat Man Singh"
link = "https://samrat.me/posts/2017-11-09-kvstore-rust-hashtable/"
+++
In the last post, I introduced the idea of linear hashing. This post will describe a Rust implementation of the algorithm. I won&rsquo;t go through every last line of code, but hopefully enough to give you a good understanding of how the whole thing works. I should also mention that even though this is a post about implementing linear hashing, it spends quite some time talking about how storing data to disk works.