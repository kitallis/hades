+++
title = "This Week in Rust Docs 12"
date = "2016-07-11T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-12"
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

<p>This week’s edition was edited by: <a href="https://github.com/GuillaumeGomez">GuillaumeGomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>In the last team meeting, we decided that the <a href="https://github.com/rust-lang/rfcs/pull/1574">RFC “More api documentation conventions”</a> was ready to get merged. We now wait for the core team confirmation.</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They both need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 50 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on github!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> clarified <a href="https://github.com/rust-lang/rust/pull/34520">UnsafeCell docs</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for missplaced doc comments</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/34637">libsyntax error code explanations</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/alexandermerritt">@alexandermerritt</a> made <a href="https://github.com/rust-lang/rust/pull/34745">docs for clone_from_slice consistent with copy_from_slice</a>.</li>
  <li><a href="https://github.com/ubsan">@ubsan</a> added <a href="https://github.com/rust-lang/rust/pull/34609">more docs to std::mem::transmute</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34461">ABI string docs in reference.md</a> and fixed <a href="https://github.com/rust-lang/rust/pull/34461">ABI string docs in reference.md</a>.</li>
  <li><a href="https://github.com/rdotdk">@rdotdk</a> updated <a href="https://github.com/rust-lang/rust/pull/34615">a cargo doc link</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> fixed <a href="https://github.com/rust-lang/rust/pull/34619">a broken markdown link in README</a>, improved <a href="https://github.com/rust-lang/rust/pull/34709">primitive integers documentation</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34717">an unnecessarily mutable reference in doc example</a> and added <a href="https://github.com/rust-lang/rust/pull/34612">doc examples for <code class="highlighter-rouge">io::Error::from_raw_os_error</code></a>.</li>
  <li><a href="https://github.com/Xmasreturns">@Xmasreturns</a> updated <a href="https://github.com/rust-lang/rust/pull/34602">glossary.md</a>.</li>
  <li><a href="https://github.com/jaredmanning">@jaredmanning</a> fixed <a href="https://github.com/rust-lang/rust/pull/34625">spacing in for loop enumeration example</a>.</li>
  <li><a href="https://github.com/sylvestre">@sylvestre</a> fixed <a href="https://github.com/rust-lang/rust/pull/34626">typos</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> documented <a href="https://github.com/rust-lang/rust/pull/34732">DoubleEndedIterator::next_back</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a>, added <a href="https://github.com/rust-lang/rust/pull/34750">examples for std::Error module</a>, improved <a href="https://github.com/rust-lang/rust/pull/34749">std::any module doc</a>, added <a href="https://github.com/rust-lang/rust/pull/34736">missing examples for std::cell types</a>, removed <a href="https://github.com/rust-lang/rust/pull/34685">an invalid CSS rule for doc titles</a>, improved <a href="https://github.com/rust-lang/rust/pull/34740">boxed docs</a>, improved <a href="https://github.com/rust-lang/rust/pull/34725">slice docs</a>. fixed <a href="https://github.com/rust-lang/rust/pull/34659"><code class="highlighter-rouge">std::path::Path::file_name()</code> doc</a>, removed <a href="https://github.com/rust-lang/rust/pull/34723">an useless doc comment for slice</a> and improved <a href="https://github.com/rust-lang/rust/pull/34688">DoubleEndedIterator examples</a>.</li>
  <li><a href="https://github.com/llogiq">@llogiq</a> improved the <a href="https://github.com/rust-lang/rust/pull/34521">internal documentation of HIR</a></li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 13th of July 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>