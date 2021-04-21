+++
title = "Shifgrethor I: Garbage collection as a Rust library"
date = "2018-10-16T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/shifgrethor-i/"
+++
I&rsquo;m really excited to share with you an experiment that I&rsquo;ve been working on for the past 5 or 6 weeks. It&rsquo;s a Rust library called shifgrethor. shifgrethor implements a garbage collector in Rust with an API I believe to be properly memory safe.
I&rsquo;ll be going through all of the technical details in future blog posts, so I want to kick this series off with a high level overview of the project&rsquo;s purpose and design decisions.