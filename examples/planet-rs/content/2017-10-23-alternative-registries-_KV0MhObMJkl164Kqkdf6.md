+++
title = "Alternative Registries"
date = "2017-10-23T14:10:21-07:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2017-10-28-alternative-registries/"
+++
cargo gained a new feature this week! You can now download dependencies from alternative registries, alongside the dependencies you download from crates.io. This is an important step in enabling organizations to distribute their internal libraries through cargo without requiring them to upload those libraries to a public registry.
This feature will be available on nightly only, and it is gated behind the alternative-registries feature gate. We&rsquo;ve used feature gates to iterate on new and unstable features in rustc since the 1.