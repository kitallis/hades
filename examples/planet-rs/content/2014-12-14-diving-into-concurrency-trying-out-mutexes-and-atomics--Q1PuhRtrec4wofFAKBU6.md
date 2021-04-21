+++
title = "Diving into concurrency: trying out mutexes and atomics"
date = "2014-12-14T12:58:55+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2014/12/14/fun-with-threads/"
+++
<p>I hadn&rsquo;t written any threaded programs before yesterday. I knew sort of
abstractly about some concurrency concepts (mutexes! people say
compare-and-swap but I don&rsquo;t totally get it!), but actually
understanding a Thing is hard if I&rsquo;ve never done it. So yesterday I
decided to write a program with threads! In this post, we&rsquo;re going to:</p>

<ol>
<li>Write a threaded program that gets the wrong answer because of a race
condition</li>
<li>Fix that race condition in C and Rust, using 2 different approaches
(mutexes and atomics)</li>
<li>Find out why Rust is slower than C</li>
<li>Talk a little about the actual system calls and instructions that
make some of this work</li>
</ol>

<p></p>