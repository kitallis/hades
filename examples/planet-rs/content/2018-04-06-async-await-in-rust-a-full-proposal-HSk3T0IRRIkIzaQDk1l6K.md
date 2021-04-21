+++
title = "Async & Await in Rust: a full proposal"
date = "2018-04-06T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/"
+++
I&rsquo;m really excited to announce the culmination of much of our work over the last four months: a pair of RFCs for supporting async &amp; await notation in Rust. This will be very impactful for Rust in the network services space. The change is proposed as two RFCs:
 RFC #2394: which adds async &amp; await notation to the language. RFC #2395: which moves a part of the futures library into std to support that syntax.