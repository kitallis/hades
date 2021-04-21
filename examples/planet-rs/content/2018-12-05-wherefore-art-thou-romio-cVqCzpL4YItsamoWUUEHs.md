+++
title = "Wherefore art thou Romio?"
date = "2018-12-05T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/romio/"
+++
This blog post is about a project called Romio that I&rsquo;ve been working on over the past two or three weeks. Romio is a port of a small part of the Tokio project to the newer futures APIs.
I started the project to get some experience porting code from the old futures API to the new API. However, we realized that this code could also be useful to other people who want to experiment with networking code using the new async/await syntax, so with the help of others we polished it up during the RustFest Rome &ldquo;impl days&rdquo; and now its being released for people to experiment with.