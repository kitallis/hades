+++
title = "Async/Await II: Narrowing the Scope of the Problem"
date = "2018-01-31T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-01-30-async-ii-narrowing-the-scope/"
+++
Last time we talked about the broader problem that generators with references across yield points represent: self-referential structs. This time, I want to narrow in on the specific problem that needs to be solved to make generators work, and also discuss some ideas for solutions that I think are false starts.
(I still don&rsquo;t have a proposal about what to do in this post, but it will come soon enough!