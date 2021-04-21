+++
title = "This Week in Rust Docs 79"
date = "2017-10-29T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-79"
+++
<p>Hello and welcome to <em>This Week in Rust Docs</em>!</p>

<p><em>This Week in Rust Docs</em> is openly developed <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs">on GitHub</a>.
If you find any errors in this week’s issue, <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs/pulls">please submit a PR</a>.</p>

<p>And of course, don’t forget to look at the docs:</p>

<ul>
  <li><a href="https://doc.rust-lang.org/">Stable</a></li>
  <li><a href="https://doc.rust-lang.org/beta/">Beta</a></li>
  <li><a href="https://doc.rust-lang.org/nightly/">Nightly</a></li>
</ul>

<p>This week’s edition was edited by <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>The switch to <a href="https://github.com/google/pulldown-cmark">Pulldown</a> for the rust doc rendering has finally <a href="https://github.com/rust-lang/rust/pull/41991">started</a>!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
  <li><a href="https://github.com/rust-lang-nursery/reference/issues/9">Document all features in the Rust reference</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/labels/T-doc">T-doc</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a> and showed <a href="https://github.com/rust-lang/rust/pull/45039">in docs whether the return type of a function impls Iterator/Read/Write</a>.</li>
  <li><a href="https://github.com/joshleeb">@joshleeb</a> fixed <a href="https://github.com/rust-lang/rust/pull/45603">duplicated display of error E0502</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/45454">Release notes for 1.22.0</a>.</li>
  <li><a href="https://github.com/leodasvacas">@leodasvacas</a> documented <a href="https://github.com/rust-lang/rust/pull/45579">that call expressions also represent ADT constructors</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> detected <a href="https://github.com/rust-lang/rust/pull/45452"><code class="highlighter-rouge">=</code> -&gt; <code class="highlighter-rouge">:</code> typo in let bindings</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/45187">sidebar rendering and added methods list</a>, added <a href="https://github.com/rust-lang/rust/pull/45582">missing links and examples</a>, add <a href="https://github.com/rust-lang/rust/pull/45470">missing docs for MetadataExt</a> and fixed <a href="https://github.com/rust-lang/rust/pull/45450">title heading overlap in rust doc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> updated <a href="https://github.com/rust-lang/rust/pull/45421">pulldown and fixed spurious rendering difference around footnotes</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/45429">docs around <code class="highlighter-rouge">Once::call_once_force</code> and <code class="highlighter-rouge">OnceState</code></a>, removed <a href="https://github.com/rust-lang/rust/pull/45585">‘future Rust version’ code block in diagnostic text</a> and improved <a href="https://github.com/rust-lang/rust/pull/45449">docs for UdpSocket::set_nonblocking</a>.</li>
  <li><a href="https://github.com/carols10cents">@carols10cents</a> corrected <a href="https://github.com/rust-lang/rust/pull/45398">misspelling in error text: re-assignment =&gt; reassignment</a>, updated <a href="https://github.com/rust-lang/rust/pull/45554">the book for a fix to the print button</a> and updated <a href="https://github.com/rust-lang/rust/pull/45555">the book on beta to fix the print button</a>.</li>
  <li><a href="https://github.com/Technius">@Technius</a> improved <a href="https://github.com/rust-lang/rust/pull/45295">std::process module docs</a>.</li>
  <li><a href="https://github.com/nzig">@nzig</a> fixed <a href="https://github.com/rust-lang/rust/pull/45574">rustc_on_unimplemented example in Unstable Book</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> removed <a href="https://github.com/rust-lang/rust/pull/45549">‘just’ in diagnostics</a> and fixed <a href="https://github.com/rust-lang/rust/pull/45531">formatting in unstable book’s attr-literals section</a>.</li>
  <li><a href="https://github.com/thombles">@thombles</a> improved <a href="https://github.com/rust-lang/rust/pull/45503">diagnostics when list of tokens has incorrect separators</a>.</li>
  <li><a href="https://github.com/michaelwoerister">@michaelwoerister</a> removed <a href="https://github.com/rust-lang/rust/pull/45519">duplicated compiler diagnostic emissions</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a>, limited <a href="https://github.com/rust-lang/rust/pull/45212">the sidebar height</a>, added <a href="https://github.com/rust-lang/rust/pull/45361">missing code examples</a> and showed <a href="https://github.com/rust-lang/rust/pull/45502">src button and function version on mobile version</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 31st of October 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>