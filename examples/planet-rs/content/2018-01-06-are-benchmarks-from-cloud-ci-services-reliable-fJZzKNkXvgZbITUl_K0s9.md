+++
title = "Are Benchmarks From Cloud CI Services Reliable?"
date = "2018-01-06T16:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/benchmarking-in-the-cloud/"
+++
After I released the first version of Criterion.rs, (a statistics-driven benchmarking tool for Rust) I was asked about using it to detect performance regressions as part of a cloud-based continuous integration (CI) pipeline such as Travis-CI or Appveyor. That got me wondering - does it even make sense to do that?
Cloud-CI pipelines have a lot of potential to introduce noise into the benchmarking process - unpredictable load on the physical hosts of the build VM&rsquo;s, or even unpredictable migrations of VMs between physical hosts.