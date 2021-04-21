+++
title = "This Week in Rust Docs 10"
date = "2016-06-27T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-10"
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

<p>The “doc days” event is now over. Thanks to everyone who contributed! Here is a list of the contributions:</p>

<ul>
  <li><a href="https://github.com/rust-lang-nursery/tempdir/pull/11">tempdir</a></li>
  <li><a href="https://github.com/rust-lang-nursery/unix-socket/pull/26">unix-socket</a></li>
  <li><a href="https://github.com/rust-lang-nursery/uuid/pull/65">uuid (1)</a> and <a href="https://github.com/rust-lang-nursery/uuid/pull/64">uuid (2)</a></li>
  <li><a href="https://github.com/rust-lang-nursery/glob/pull/53">glob</a></li>
  <li><a href="https://github.com/rust-lang-nursery/rand/pull/106">rand</a></li>
  <li><a href="https://github.com/rust-lang-nursery/rustc-serialize/pull/153">rustc-serialize</a></li>
</ul>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues opened for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They both need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 50 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on github!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/tatsuya6502">@tatsuya6502</a> fixed <a href="https://github.com/rust-lang/rust/pull/34442">links in Ownership section of the book</a>.</li>
  <li><a href="https://github.com/ubsan">@ubsan</a> fixed <a href="https://github.com/rust-lang/rust/pull/34461">ABI string docs in reference.md</a>.</li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/34462">example with leading zeros</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expanded <a href="https://github.com/rust-lang/rust/pull/34475"><code class="highlighter-rouge">std::path::Component</code> documentation</a> and added <a href="https://github.com/rust-lang/rust/pull/34406">example for <code class="highlighter-rouge">std::thread::sleep</code></a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">specific error message for missplaced doc comments</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/34479">inlined renamed reexports in import lists</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34415">float examples</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34477">search result layout for enum variants and struct fields</a> and removed <a href="https://github.com/rust-lang/rust/pull/34105">Remove Derived Implementations title</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/34471">E0269 explanation</a> and added <a href="https://github.com/rust-lang/rust/pull/34467">new error codes and improve some explanations</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/34469">how the <code class="highlighter-rouge">std::path::Components</code> struct is created</a>, indicated <a href="https://github.com/rust-lang/rust/pull/34438">how the <code class="highlighter-rouge">JoinHandle</code> struct is created</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34410">doc parameters formatting</a>, added <a href="https://github.com/rust-lang/rust/pull/34465">doc example for <code class="highlighter-rouge">std::thread::Builder::name</code></a> and added <a href="https://github.com/rust-lang/rust/pull/34468">hyperlinks to <code class="highlighter-rouge">std::fs</code> functions from <code class="highlighter-rouge">std::path</code></a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> updated <a href="https://github.com/rust-lang/rust/pull/34378">rustc-ux-guidelines</a> and improved <a href="https://github.com/rust-lang/rust/pull/34379">E0425 explanation</a>.</li>
  <li><a href="https://github.com/cynicaldevil">@cynicaldevil</a> modified E0220 <a href="https://github.com/rust-lang/rust/pull/34364">to show error messages for more general cases</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/34372">more types to the rustdoc sidebar</a>, fixed <a href="https://github.com/rust-lang/rust/pull/34439">panic caused by doc(hidden) trait methods</a> and fixed <a href="https://github.com/rust-lang/rust/pull/34387">a couple of issues with src links to external crates</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/34401">error code flags</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 29th of June 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>