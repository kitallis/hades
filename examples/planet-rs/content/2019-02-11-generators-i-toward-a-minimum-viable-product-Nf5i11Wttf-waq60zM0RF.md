+++
title = "Generators I: Toward a minimum viable product"
date = "2019-02-11T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/generators-i/"
+++
We&rsquo;re still not finished with the design of async/await, but it&rsquo;s already become clear that it&rsquo;s time to get the next phases of the feature into the pipeline. There are two extensions to the minimal async/await feature we&rsquo;ve currently got that seem like the clear high priority:
 Async methods: allowing async fn to be used in traits. Generators: allowing imperative control flow to create Iterators and Streams the same way async fn allows imperative control flow to create a Future.