+++
title = "Why Rust Has Macros"
date = "2018-03-04T00:00:00+00:00"

[extra]
author = "Bloggedy blog"
link = "https://kasma1990.gitlab.io/2018/03/04/why-rust-has-macros/"
+++
When I recently told a coworker that Rust has macros, his first reaction was that this was bad. Previously I would have had the same reaction, but a part of what learning Rust has taught me is that macros don&rsquo;t need to be bad. This post exists to help explain why that is, by diving into what problems macros solve, with a brief look at their downsides as well. In other words, this post is not a technical deep dive on how macros work, but focuses on the use cases for macros, and doesn&rsquo;t require much knowledge about Rust to follow.