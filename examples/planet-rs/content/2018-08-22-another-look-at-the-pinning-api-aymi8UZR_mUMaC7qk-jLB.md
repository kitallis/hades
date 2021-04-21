+++
title = "Another look at the pinning API"
date = "2018-08-22T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/rethinking-pin/"
+++
A few months ago we introduced the concept of &ldquo;pinned&rdquo; references - pointers which &ldquo;pin&rdquo; the data they refer to in a particular memory location, guaranteeing that it will never move again. These are an important building block for certain patterns that had previously been hard for Rust to handle, like self-referential structs and intrusive lists, and we&rsquo;ve in the process of considering stabilizing the API.
One thing has always nagged about the API we have right now though: the proliferation of different reference types that it implies.