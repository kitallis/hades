+++
title = "This Week in Rust Docs 11"
date = "2016-07-04T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-11"
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

<p>In the last team meeting (renamed “schrodinger’s doc meeting”, thanks for the idea <a href="https://github.com/jonathandturner">@jonathandturner</a>!), we mainly talked about The Rust Doc Days. The feedback is globally good but I’ll let read the great <a href="https://facility9.com/2016/07/rust-doc-days-follow-up/">post</a> that <a href="https://github.com/peschkaj">@peschkaj</a> wrote about it.</p>

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
  <li><a href="https://github.com/sylvestre">@sylvestre</a> fixed <a href="https://github.com/rust-lang/rust/pull/34626">typos</a>.</li>
  <li><a href="https://github.com/jaredmanning">@jaredmanning</a> fixed <a href="https://github.com/rust-lang/rust/pull/34625">spacing in for loop enumeration example</a>.</li>
  <li><a href="https://github.com/Xmasreturns">@Xmasreturns</a> updated <a href="https://github.com/rust-lang/rust/pull/34602">glossary.md</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> fixed <a href="https://github.com/rust-lang/rust/pull/34619">broken markdown link in README</a> and added <a href="https://github.com/rust-lang/rust/pull/34612">doc examples for <code class="highlighter-rouge">io::Error::from_raw_os_error</code></a>.</li>
  <li><a href="https://github.com/rdotdk">@rdotdk</a> updated <a href="https://github.com/rust-lang/rust/pull/34615">cargo doc link</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> clarified <a href="https://github.com/rust-lang/rust/pull/34520">UnsafeCell docs</a>.</li>
  <li><a href="https://github.com/ubsan">@ubsan</a> added <a href="https://github.com/rust-lang/rust/pull/34609">more docs to std::mem::transmute</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34461">ABI string docs in reference.md</a> and fixed <a href="https://github.com/rust-lang/rust/pull/34461">ABI string docs in reference.md</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">specific error message for missplaced doc comments</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/34637">libsyntax error codes explanation</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/jupp0r">@jupp0r</a> improved <a href="https://github.com/rust-lang/rust/pull/34540">code example for try!</a>.</li>
  <li><a href="https://github.com/jonmarkprice">@jonmarkprice</a> made <a href="https://github.com/rust-lang/rust/pull/34532">small grammatical and stylistic edits to The Book</a>.</li>
  <li><a href="https://github.com/tatsuya6502">@tatsuya6502</a> fixed <a href="https://github.com/rust-lang/rust/pull/34442">links in Ownership section of the book</a>.</li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/34462">example with leading zeros</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expanded <a href="https://github.com/rust-lang/rust/pull/34475"><code class="highlighter-rouge">std::path::Component</code> documentation</a>, added <a href="https://github.com/rust-lang/rust/pull/34524">doc example for <code class="highlighter-rouge">std::io::sink</code></a>, added <a href="https://github.com/rust-lang/rust/pull/34518">doc example for <code class="highlighter-rouge">std::io::repeat</code></a>, made <a href="https://github.com/rust-lang/rust/pull/34517">a minor rewrite of <code class="highlighter-rouge">std::io::empty</code> doc example</a> and added <a href="https://github.com/rust-lang/rust/pull/34406">example for <code class="highlighter-rouge">std::thread::sleep</code></a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/34479">inlined renamed reexports in import lists</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34513">a few stripping issues in rustdoc</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34415">float examples</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34477">search result layout for enum variants and struct fields</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34536">empty Implementations section on some module pages in rustdoc</a> and removed <a href="https://github.com/rust-lang/rust/pull/34105">Remove Derived Implementations title</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/34471">E0269 explanation</a>, added <a href="https://github.com/rust-lang/rust/pull/34531">error codes in libsyntax</a> and added <a href="https://github.com/rust-lang/rust/pull/34467">new error codes and improve some explanations</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 6th of July 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>