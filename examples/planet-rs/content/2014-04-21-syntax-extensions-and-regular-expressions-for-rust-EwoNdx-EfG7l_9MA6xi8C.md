+++
title = "Syntax extensions and regular expressions for Rust"
date = "2014-04-21T19:51:00-04:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/rust-regex-syntax-extensions/"
+++
<p><strong>WARNING:</strong> <!-- raw HTML omitted -->2018-04-12<!-- raw HTML omitted -->: The code snippets for this post are no longer
available. This is just as well anyway, since they all depended on an unstable
internal compiler interface, which hasn&rsquo;t existed for years.</p>
<p>A few weeks ago, I set out to add regular expressions to the
<a href="http://www.rust-lang.org/">Rust</a>
distribution with an implementation and feature set heavily inspired by
<a href="http://swtch.com/~rsc/regexp/">Russ Cox&rsquo;s RE2</a>.
It was just recently added to the
<a href="http://static.rust-lang.org/doc/master/regex/index.html">Rust distribution</a>.</p>
<p>This regex crate includes a syntax extension that compiles a regular expression
to native Rust code <em>when a Rust program is compiled</em>. This can be thought of
as &ldquo;ahead of time&rdquo; compilation or
something similar to <a href="http://en.wikipedia.org/wiki/Compile_time_function_execution">compile time function
execution</a>.
These special natively compiled regexes have the <em>same exact</em> API as regular
expressions compiled at runtime.</p>
<p>In this article, I will outline my implementation strategy&mdash;including code
samples on how to write a Rust syntax extension&mdash;and describe how I was able
to achieve an identical API between regexes compiled at compile time and
regexes compiled at runtime.</p>