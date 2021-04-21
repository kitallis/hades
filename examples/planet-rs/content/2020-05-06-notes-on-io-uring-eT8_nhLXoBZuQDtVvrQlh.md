+++
title = "Notes on io-uring"
date = "2020-05-06T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/io-uring/"
+++
Last fall I was working on a library to make a safe API for driving futures on top of an an io-uring instance. Though I released bindings to liburing called iou, the futures integration, called ostkreuz, was never released. I don&rsquo;t know if I will pick this work up again in the future but several different people have started writing other libraries with similar goals, so I wanted to write up some notes on what I learned working with io-uring and Rust&rsquo;s futures model.