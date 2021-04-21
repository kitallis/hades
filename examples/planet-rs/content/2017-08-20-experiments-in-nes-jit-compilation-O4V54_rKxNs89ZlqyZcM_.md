+++
title = "Experiments In NES JIT Compilation"
date = "2017-08-20T11:35:37-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/experiments-in-nes-jit-compilation/"
+++
Inspired by the always-incredible work on Dolphin, I decided to write myself an NES emulator called Corrosion a couple years ago. I managed to get it working well enough to play basic games, and then put the project aside. This post is not about the emulator itself, but rather the JIT compiler I added to it last year and the upgrades to said JIT compiler I&rsquo;ve made over the past few weeks.