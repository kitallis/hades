+++
title = "ripgrep is faster than {grep, ag, git grep, ucg, pt, sift}"
date = "2016-09-23T07:05:00-04:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/ripgrep/"
+++
<p>In this article I will introduce a new command line search tool,
<a href="https://github.com/BurntSushi/ripgrep"><code>ripgrep</code></a>,
that combines the usability of
<a href="https://github.com/ggreer/the_silver_searcher">The Silver Searcher</a>
(an <a href="http://beyondgrep.com/"><code>ack</code></a> clone) with the
raw performance of GNU grep. <code>ripgrep</code> is fast, cross platform (with binaries
available for Linux, Mac and Windows) and written in
<a href="https://www.rust-lang.org">Rust</a>.</p>
<p><code>ripgrep</code> is available on
<a href="https://github.com/BurntSushi/ripgrep">Github</a>.</p>
<p>We will attempt to do the impossible: a fair benchmark comparison between
several popular code search tools. Specifically, we will dive into a series of
25 benchmarks that substantiate the following claims:</p>
<ul>
<li>For both searching single files <em>and</em> huge directories of files, no other
tool obviously stands above <code>ripgrep</code> in either performance or correctness.</li>
<li><code>ripgrep</code> is the only tool with proper Unicode support that doesn&rsquo;t make
you pay dearly for it.</li>
<li>Tools that search many files at once are generally <em>slower</em> if they use
memory maps, not faster.</li>
</ul>
<p>As someone who has worked on text search in Rust in their free time for the
last 2.5 years, and as the author of both <code>ripgrep</code> and
<a href="https://github.com/rust-lang-nursery/regex">the underlying regular expression engine</a>,
I will use this opportunity to provide detailed insights into the performance
of each code search tool. No benchmark will go unscrutinized!</p>
<p><strong>Target audience</strong>: Some familiarity with Unicode, programming and some
experience with working on the command line.</p>
<p><strong>NOTE</strong>: I&rsquo;m hearing reports from some people that <code>rg</code> isn&rsquo;t as fast as I&rsquo;ve
claimed on their data. I&rsquo;d love to help explain what&rsquo;s going on, but to do
that, I&rsquo;ll need to be able to reproduce your results. If you
<a href="https://github.com/BurntSushi/ripgrep/issues">file an issue</a>
with something I can reproduce, I&rsquo;d be happy to try and explain it.</p>