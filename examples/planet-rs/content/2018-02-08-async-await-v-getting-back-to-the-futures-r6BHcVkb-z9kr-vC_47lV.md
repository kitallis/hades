+++
title = "Async/Await V: Getting back to the futures"
date = "2018-02-08T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-02-08-async-v-getting-back-to-the-futures/"
+++
Two posts ago I proposed a particular interface for shipping self-referential generators this year. Immediately after that, eddyb showed me a better interface, which I described in the next post. Now, to tie everything together, its time to talk about how we can integrate this into the futures ecosystem.
Starting point: this Generator API To begin, I want to document the generator API I&rsquo;ll be using in this post, which is roughly what followed from my previous post: