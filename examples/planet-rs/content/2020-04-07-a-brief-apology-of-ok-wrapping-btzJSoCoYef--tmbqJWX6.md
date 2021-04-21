+++
title = "A brief apology of Ok-Wrapping"
date = "2020-04-07T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/why-ok-wrapping/"
+++
I&rsquo;ve long been a proponent of having some sort of syntax in Rust for writing functions which return results which &ldquo;ok-wrap&rdquo; the happy path. This is has also always been a feature with very vocal, immediate, and even emotional opposition from many of our most enthusiastic users. I want to write, in one place, why I think this feature would be awesome and make Rust much better.
I don&rsquo;t want to get into the details too much of the specific proposal, but here&rsquo;s a sketch of one way this could work (there are a number of variables).