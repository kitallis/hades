+++
title = "Error Handling in Rust"
date = "2015-05-14T10:47:26-04:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/rust-error-handling/"
+++
<p>Like most programming languages, Rust encourages the programmer to handle
errors in a particular way. Generally speaking, error handling is divided into
two broad categories: exceptions and return values. Rust opts for return
values.</p>
<p>In this article, I intend to provide a comprehensive treatment of how to deal
with errors in Rust. More than that, I will attempt to introduce error handling
one piece at a time so that you&rsquo;ll come away with a solid working knowledge of
how everything fits together.</p>
<p>When done naively, error handling in Rust can be verbose and annoying. This
article will explore those stumbling blocks and demonstrate how to use the
standard library to make error handling concise and ergonomic.</p>
<p><strong>Target audience</strong>: Those new to Rust that don&rsquo;t know its error handling
idioms yet. Some familiarity with Rust is helpful. (This article makes heavy
use of some standard traits and some very light use of closures and macros.)</p>
<p><strong>Update (2018/04/14)</strong>: Examples were converted to <code>?</code>, and some text was
added to give historical context on the change.</p>
<p><strong>Update (2020/01/03)</strong>: A recommendation to use
<a href="https://crates.io/crates/failure"><code>failure</code></a> was removed and replaced with
a recommendation to use either <code>Box&lt;Error + Send + Sync&gt;</code> or
<a href="https://crates.io/crates/anyhow"><code>anyhow</code></a>.</p>