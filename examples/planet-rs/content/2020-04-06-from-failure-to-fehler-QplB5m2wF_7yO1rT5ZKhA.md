+++
title = "From failure to Fehler"
date = "2020-04-06T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/failure-to-fehler/"
+++
About two and a half years ago I wrote a Rust library called failure, which quickly became one of the most popular error handling libraries in Rust. This week, its current maintainer decided to deprecate it, a decision I strongly support. This week, I also released a new and very different error-handling library, called fehler. I wanted to discuss these two libraries briefly.
A brief history of failure When I released failure, the most popular error handling library by far was error-chain.