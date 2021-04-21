+++
title = "Async/Await IV: An Even Better Proposal"
date = "2018-02-07T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-02-07-async-iv-an-even-better-proposal/"
+++
I did not plan to write this blog post. I thought that the fourth post in my series would explain how we could go from the generator API in my previous post to a futures API in which you don&rsquo;t have to heap allocate every async call. But eddyb surprised me, and now I have to revisit the API in the previous post, because we can implement everything we need from immovability with a safe interface afterall.