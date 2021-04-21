+++
title = "Shifgrethor IV: Tracing"
date = "2018-10-31T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/shifgrethor-iv/"
+++
The post before this one covered how shifgrethor handles rooting: how we track for the garbage collector that this object is alive. That isn&rsquo;t sufficient for implementing a tracing garbage collector though: the idea of a tracing garbage collector is that we can trace from rooted objects through all of the objects they reference. That way, instead of having to root everything you use, you can only root a few objects from which all of the live objects can be traced.