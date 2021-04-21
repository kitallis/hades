+++
title = "This Week in Rust Docs 63"
date = "2017-07-09T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-63"
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

<p>None.</p>

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
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/42503">releases notes for 1.19</a>.</li>
  <li><a href="https://github.com/rthomas">@rthomas</a> updated <a href="https://github.com/rust-lang/rust/pull/42837">docs on Error struct</a>.</li>
  <li><a href="https://github.com/Fourchaux">@Fourchaux</a> fixed <a href="https://github.com/rust-lang/rust/pull/42812">basic typos in Doc Comments</a>.</li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/42670">hint about the return code of panic!</a>.</li>
  <li><a href="https://github.com/Emilgardis">@Emilgardis</a> fixed <a href="https://github.com/rust-lang/rust/pull/42270">wrong report on closure args mismatch when a ref is expected but not found</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> refactored <a href="https://github.com/rust-lang/rust/pull/42897">pretty printing slightly</a> and allowed <a href="https://github.com/rust-lang/rust/pull/42697">setting the limit on std::io::Take</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, added <a href="https://github.com/rust-lang/rust/pull/43009">errors when doc comments are added where they’re unused</a>, cleaned up <a href="https://github.com/rust-lang/rust/pull/43006">some code</a> and add <a href="https://github.com/rust-lang/rust/pull/43130">spacing between trait functions</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/rthomas">@rthomas</a> updated <a href="https://github.com/rust-lang/rust/pull/42836">docs for Debug* structs</a> and added <a href="https://github.com/rust-lang/rust/pull/43093">annotations to the resize fn</a>.</li>
  <li><a href="https://github.com/cengizIO">@cengizIO</a> added <a href="https://github.com/rust-lang/rust/pull/42732">pager support for <code class="highlighter-rouge">rustc --explain EXXXX</code></a>.</li>
  <li><a href="https://github.com/Boreeas">@Boreeas</a> folded <a href="https://github.com/rust-lang/rust/pull/42996">E0612, E0613 into E0609</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/43001"><code class="highlighter-rouge">rustc_on_unimplemented</code> message to <code class="highlighter-rouge">std::ops::Try</code></a> and made <a href="https://github.com/rust-lang/rust/pull/42904">suggestion include the line number</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> fixed <a href="https://github.com/rust-lang/rust/pull/43075">links for typeck diagnostics without tripping tidy</a>.</li>
  <li><a href="https://github.com/arielb1">@arielb1</a> reported <a href="https://github.com/rust-lang/rust/pull/43015">the total number of errors on compilation failure</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/43068">markdown tests being run twice</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> made <a href="https://github.com/rust-lang/rust/pull/43050">a minor fix in docs for Vec</a>.</li>
  <li><a href="https://github.com/andersk">@andersk</a> documented <a href="https://github.com/rust-lang/rust/pull/43041">unintuitive argument order for Vec::dedup_by relation</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/42972">toggle wrappers not generated correctly in rustdoc</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 12th of July 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>