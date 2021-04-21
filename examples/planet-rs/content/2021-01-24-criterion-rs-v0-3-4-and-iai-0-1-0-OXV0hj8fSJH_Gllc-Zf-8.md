+++
title = "Criterion.rs v0.3.4 And Iai 0.1.0"
date = "2021-01-24T14:30:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/criterion-rs-0-3-4/"
+++
Today I&rsquo;ve released Criterion 0.3.4 and Iai 0.1.0. I&rsquo;m particularly excited by Iai, so read on to find out what I&rsquo;ve been up to.
Criterion Updates The main new feature in this release is that Criterion.rs now has built-in support for benchmarking async functions.
 This feature requires the async feature to be enabled. In addition to this, four other features - async-std, async-tokio, async-smol, and async-futures can be enabled to add support for benchmarking with the respective futures executors.