+++
title = "The Waker API I: what does a waker do?"
date = "2019-01-07T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/wakers-i/"
+++
Work on supporting async/await in Rust continues to progress rapidly. I&rsquo;m hoping to write a retrospective on everything that happened in 2018 in a few weeks. Right now we&rsquo;re closing in on an important milestone: stabilizing the futures API that will be used to interact programmatically with asynchronous computations. The biggest remaining area of work is the design of the waker API, an essential but somewhat opaque part of how our asynchronous programming system works.