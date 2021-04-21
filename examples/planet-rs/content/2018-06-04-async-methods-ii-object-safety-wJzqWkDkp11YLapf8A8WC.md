+++
title = "Async Methods II: object safety"
date = "2018-06-04T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/async-methods-ii/"
+++
Last time, we introduced the idea of async methods, and talked about how they would be implemented: as a kind of anonymous associated type on the trait that declares the method, which corresponds to a different, anonymous future type for each implementation of that method. Starting this week we&rsquo;re going to look at some of the implications of that. The first one we&rsquo;re going to look at is object safety.