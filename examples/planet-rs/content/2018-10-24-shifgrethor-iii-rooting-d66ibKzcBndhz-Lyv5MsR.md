+++
title = "Shifgrethor III: Rooting"
date = "2018-10-24T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/shifgrethor-iii/"
+++
After the digression in the previous post, it&rsquo;s time to get back to what I promised in the first post: a look at how shifgrethor handles rooting. Shifgrethor&rsquo;s solution is somewhat novel and takes advantage of some of Rust&rsquo;s specific features, so I want to start by looking briefly at some of the other options.
How to root a GC&rsquo;d object There are two broad categories of rooting strategies that are common among precise, tracing garbage collectors: