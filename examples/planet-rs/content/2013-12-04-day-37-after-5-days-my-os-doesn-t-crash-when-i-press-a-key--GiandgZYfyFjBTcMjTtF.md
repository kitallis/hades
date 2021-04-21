+++
title = "Day 37: After 5 days, my OS doesn't crash when I press a key"
date = "2013-12-04T00:00:00+00:00"

[extra]
author = "Rust on Julia Evans"
link = "https://jvns.ca/blog/2013/12/04/day-37-how-a-keyboard-works/"
+++
Right now I&rsquo;m working towards being able to:
 press keys on my keyboard having the OS not crash and have the key I pressed be echoed back  I just achieved step 2, and this has been kind of a saga, so here&rsquo;s an explanation of the blood and tears involved. First up, some resources that really helped me out:
 The fantastic OSDev wiki. This tutorial on how to make a basic x86 kernel, especially the IDT page.