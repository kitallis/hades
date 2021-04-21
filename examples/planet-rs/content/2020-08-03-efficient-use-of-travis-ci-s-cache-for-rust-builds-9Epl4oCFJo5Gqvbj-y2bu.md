+++
title = "Efficient Use of Travis-CI's Cache For Rust Builds"
date = "2020-08-03T18:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/efficient-use-of-travis-ci-cache-for-rust/"
+++
A while ago, I complained on Reddit about the Travis-CI build times for my Rust crates. Aleksey Kladov, better known in the Rust community as &ldquo;matklad&rdquo;, responded to mention that Travis-CI&rsquo;s caching behavior is&hellip; suboptimal for Rust crates and gave me a way to fix it.
I figure there are probably other Rust developers out there who aren&rsquo;t aware of this, so I&rsquo;m writing a short post to explain it.