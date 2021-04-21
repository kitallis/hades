+++
title = "Failure 0.1.1 released"
date = "2017-11-30T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2017-11-30-failure-0-1-1/"
+++
I&rsquo;ve just published failure 0.1.1 to crates.io. It&rsquo;s mostly some incremental improvements to failure that have been suggested since the first release two weeks ago.
Improvements to the derive A big change in version 0.1.1 is that the derive can be used without depending on the failure_derive crate separately. All that needs to be done is tagging the extern crate statement with #[macro_use]:
// No direct dependency on `failure_derive` #[macro_use] extern crate failure; #[derive(Fail, Debug)] #[fail(display = &quot;An error occurred.