+++
title = "Exploring lock-free Rust 3: Crossbeam"
date = "2017-10-12T08:58:39+00:00"

[extra]
author = "rust – More Stina Blog!"
link = "https://morestina.net/blog/784/exploring-lock-free-rust-3-crossbeam"
+++
In the previous installment we showed that while atomic types provided by the Rust standard library can be used for lock-free access to shared values, memory reclamation must be ensured manually because Rust&#8217;s normal scoping rules do not cleanly map to lock-free concurrency. Crossbeam The problem of memory reclamation in lock-free data structures is not &#8230; <a href="https://morestina.net/blog/784/exploring-lock-free-rust-3-crossbeam" class="more-link">Continue reading <span class="screen-reader-text">Exploring lock-free Rust 3: Crossbeam</span> <span class="meta-nav">&#8594;</span></a>