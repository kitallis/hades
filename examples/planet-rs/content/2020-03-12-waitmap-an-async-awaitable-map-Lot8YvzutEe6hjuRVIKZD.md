+++
title = "waitmap - an async awaitable map"
date = "2020-03-12T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/waitmap/"
+++
I&rsquo;ve just released a new crate called waitmap. This is a concurrent hash map (built on top of dashmap) intended for use as a concurrency primitive with async/await. It extends the API of dashmap by having an additional wait method.
The wait future looks up an entry in the map and suspends this task if the entry was not present when wait was called. The task will be woken whenever a value is inserted under that key.