+++
title = "Exploring lock-free Rust 2: Atomics"
date = "2017-10-12T08:58:35+00:00"

[extra]
author = "rust – More Stina Blog!"
link = "https://morestina.net/blog/749/exploring-lock-free-rust-2-atomics"
+++
The previous article showed a mutex-based implementation of the LazyTransform value container. However, the exercise was explicit about implementing a &#8220;lock-free&#8221; container. What does that mean, exactly? Limitations of locks The interaction between LazyTransform methods was carefully designed to prevent deadlock, but in a busy system one could expect significant contention for the source mutex &#8230; <a href="https://morestina.net/blog/749/exploring-lock-free-rust-2-atomics" class="more-link">Continue reading <span class="screen-reader-text">Exploring lock-free Rust 2: Atomics</span> <span class="meta-nav">&#8594;</span></a>