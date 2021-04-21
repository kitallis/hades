+++
title = "Day 39: Writing malloc wrong, for fun"
date = "2013-12-10T00:00:00+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2013/12/10/day-39-i-wrote-a-malloc/"
+++
My major achievement for today is writing the following five lines of code:
let a: ~u8 = ~('A' as u8); stdio::putc(*a); let b: ~u8 = ~('B' as u8); stdio::putc(*a); stdio::putc(*b);  and having them do the wrong thing. One would normally expect this to print &ldquo;AAB&rdquo;. But for me, right now, until I stop goofing off, it prints &ldquo;ABB&rdquo;. Why is that?
Well, it&rsquo;s because my malloc implementation looks like this:
