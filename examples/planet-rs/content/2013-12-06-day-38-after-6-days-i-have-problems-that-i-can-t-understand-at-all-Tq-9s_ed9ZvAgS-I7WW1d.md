+++
title = "Day 38: After 6 days, I have problems that I can't understand at all"
date = "2013-12-06T00:00:00+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2013/12/06/day-38-after-7-days/"
+++
tl;dr: I expect NUMS[2] to equal NUMS[keycode] when keycode == 2. This doesn&rsquo;t appear to be the case, and I don&rsquo;t understand how this is possible.
I&rsquo;m trying to set up keycode handling in my kernel, and I&rsquo;m having a strange problem with array indexing that I can&rsquo;t really fathom at all (except &ldquo;something is wrong&rdquo;).
When I run this code, and press 1 several times, it prints |2C |2C |2C |2C |2C |2C |2C |2C |2C.