+++
title = "Day 36: On programming without malloc"
date = "2013-12-03T00:00:00+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2013/12/03/day-36-programming-without-malloc/"
+++
So right now I&rsquo;m working on writing a kernel in Rust. My current goal is to press keys on the keyboard and have them echoed to the screen. This is going okay! I anticipate being able to type by the end of the week.
One thing that&rsquo;s interesting is that my expectations around what programs should be able to do is really different right now. Normally I write Python or other high-level languages, so my programs don&rsquo;t run too quickly, but have tons of resources available to them (the Internet, a standard library, memory allocation, garbage collection, &hellip;).