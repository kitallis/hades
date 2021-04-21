+++
title = "Criterion.rs v0.3 - Custom Measurements, Profiling Hooks, Custom Test Framework, API Changes"
date = "2019-08-25T10:30:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/criterion-rs-0-3/"
+++
I&rsquo;m pleased to announce the release of Criterion.rs v0.3, available today. Version 0.3 provides a number of new features including preliminary support for plugging in custom measurements (eg. hardware timers or POSIX CPU time), hooks to start/stop profilers, a new BenchmarkGroup struct that provides more flexibility than the older Benchmark and ParameterizedBenchmark structs, and an implementation of a #[criterion] custom-test-framework macro for those on Nightly.
What is Criterion.rs? Criterion.rs is a statistics-driven benchmarking library for Rust.