+++
title = "This Week in Rust Docs 38"
date = "2017-01-08T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-38"
+++
<p>Hello and welcome to <em>This Week in Rust Docs</em>!</p>

<p><em>This Week in Rust Docs</em> is openly developed <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs">on GitHub</a>.
If you find any errors in this week’s issue, <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs/pulls">please submit a PR</a>.</p>

<p>And of course, don’t forget to look at the docs:</p>

<ul>
  <li><a href="https://doc.rust-lang.org/">Stable</a></li>
  <li><a href="http://doc.rust-lang.org/beta/">Beta</a></li>
  <li><a href="http://doc.rust-lang.org/nightly/">Nightly</a></li>
</ul>

<p>This week’s edition was edited by <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/chris-morgan">@chris-morgan</a> fixed <a href="https://github.com/rust-lang/rust/pull/38922">a couple of bad Markdown links</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38569">rustdoc highlighting of <code class="highlighter-rouge">&amp;</code> and <code class="highlighter-rouge">*</code></a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> resolved: <a href="https://github.com/rust-lang/rust/pull/38890">do not use “resolve”/”resolution” in error messages</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/38605">solutions for <code class="highlighter-rouge">fn foo(&amp;foo: Foo)</code></a>, removed <a href="https://github.com/rust-lang/rust/pull/38902"><code class="highlighter-rouge">note: </code> prefix from type mismatch errors</a> and escaped <a href="https://github.com/rust-lang/rust/pull/38244">the deprecated and unstable reason text in rustdoc</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> improved <a href="https://github.com/rust-lang/rust/pull/38910">safety warning on ptr::swap</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> added <a href="https://github.com/rust-lang/rust/pull/37910">suggestions of #[macro_use] in every case for undefined macros</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expanded <a href="https://github.com/rust-lang/rust/pull/38839">{Path,OsStr}::{to_str,to_string_lossy} doc examples.</a>, clarified <a href="https://github.com/rust-lang/rust/pull/38581">behavior of <code class="highlighter-rouge">VecDeque::insert</code>.</a> and clarified <a href="https://github.com/rust-lang/rust/pull/38310">zero-value behavior of <code class="highlighter-rouge">ctlz</code>/<code class="highlighter-rouge">cttz</code> intrinsics.</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> improved <a href="https://github.com/rust-lang/rust/pull/38843">rustdoc rendering for unstable features</a> and added <a href="https://github.com/rust-lang/rust/pull/38816">more docs for CoerceUnsized and Unsize</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/derekdreery">@derekdreery</a> updated <a href="https://github.com/rust-lang/rust/pull/38874">vec.rs</a>.</li>
  <li><a href="https://github.com/linclark">@linclark</a> add <a href="https://github.com/rust-lang/rust/pull/38108">error explanation for E0328.</a>.</li>
  <li><a href="https://github.com/shahn">@shahn</a> clarified <a href="https://github.com/rust-lang/rust/pull/38636">Extend behaviour wrt existing keys</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> updated <a href="https://github.com/rust-lang/rust/pull/38841">usage of rustc</a>.</li>
  <li><a href="https://github.com/emilio">@emilio</a> added <a href="https://github.com/rust-lang/rust/pull/38825">an attribute to ignore collecting tests per-item in rustdoc</a>.</li>
  <li><a href="https://github.com/ShaunKarran">@ShaunKarran</a> fixed <a href="https://github.com/rust-lang/rust/pull/38845">docs for TcpListener example</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/38836">typo in tuple docs</a>.</li>
  <li><a href="https://github.com/Freyskeyd">@Freyskeyd</a> improved <a href="https://github.com/rust-lang/rust/pull/38823">doc for cfg(test) and tests directory</a>.</li>
  <li><a href="https://github.com/cbreeden">@cbreeden</a> updated <a href="https://github.com/rust-lang/rust/pull/38704">sign formatting for numeric types in docs</a>.</li>
  <li><a href="https://github.com/rkruppe">@rkruppe</a> replaced <a href="https://github.com/rust-lang/rust/pull/38138">loop {} with abort() in The Book</a>.</li>
  <li><a href="https://github.com/bombless">@bombless</a> fixed <a href="https://github.com/rust-lang/rust/pull/38629">doc for <code class="highlighter-rouge">escape_debug</code></a>.</li>
  <li><a href="https://github.com/chriskrycho">@chriskrycho</a> documented <a href="https://github.com/rust-lang/rust/pull/37928">RFC 1623: static lifetime elision.</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/38819">a distinct error code and description for “main function has wrong prototype”</a>, added <a href="https://github.com/rust-lang/rust/pull/38362">instant doc</a>, fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a> and added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> disambiguated <a href="https://github.com/rust-lang/rust/pull/38414">Implementors when the type name is not unique in rustdoc</a>.</li>
  <li><a href="https://github.com/jonathandturner">@jonathandturner</a> fixed <a href="https://github.com/rust-lang/rust/pull/38859">E0088/E0090</a>.</li>
  <li><a href="https://github.com/sanxiyn">@sanxiyn</a> removed <a href="https://github.com/rust-lang/rust/pull/38773">rustdoc ICE when an unstable feature is used</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> documented <a href="https://github.com/rust-lang/rust/pull/38770">custom derive</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> reworded <a href="https://github.com/rust-lang/rust/pull/38782">‘stupid’ and ‘crazy’ in docs</a>.</li>
  <li><a href="https://github.com/programble">@programble</a> added <a href="https://github.com/rust-lang/rust/pull/38711">links to methods on all slice iterator struct docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/38548">missing example for Thread struct</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 11th of January 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>