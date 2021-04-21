+++
title = "Index 1,600,000,000 Keys with Automata and Rust"
date = "2015-11-11T22:45:00-04:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/transducers/"
+++
<p>It turns out that finite state machines are useful for things other than
expressing computation. Finite state machines can also be used to compactly
represent ordered sets or maps of strings that can be searched very quickly.</p>
<p>In this article, I will teach you about finite state machines as a <em>data
structure</em> for representing ordered sets and maps. This includes introducing
an implementation written in Rust called the
<a href="https://github.com/BurntSushi/fst"><code>fst</code> crate</a>.
It comes with
<a href="https://burntsushi.net/rustdoc/fst/">complete API documentation</a>.
I will also show you how to build them using a simple command line tool.
Finally, I will discuss a few experiments culminating in indexing over
1,600,000,000 URLs (134 GB) from the
<a href="http://blog.commoncrawl.org/2015/08/july-2015-crawl-archive-available/">July 2015 Common Crawl Archive</a>.</p>
<p>The technique presented in this article is also how
<a href="http://blog.mikemccandless.com/2010/12/using-finite-state-transducers-in.html">Lucene represents a part of its inverted
index</a>.</p>
<p>Along the way, we will talk about memory maps, automaton intersection with
regular expressions, fuzzy searching with Levenshtein distance and streaming
set operations.</p>
<p><strong>Target audience</strong>: Some familiarity with programming and fundamental data
structures. No experience with automata theory or Rust is required.</p>