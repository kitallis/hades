+++
title = "Async/Await I: Self-Referential Structs"
date = "2018-01-25T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-01-25-async-i-self-referential-structs/"
+++
This is the first in a series of blog posts about generators, async &amp; await in Rust. By the end of this series, I will have a proposal for how we could expediently (within the next 12 months) bring generators, async &amp; await to stable Rust, and resolve some of the most difficult ergonomics problems in the futures ecosystem.
But that proposal is still several posts away. Before we can get to a concrete proposition, we need to understand the scope &amp; nature of the problem we need to solve.