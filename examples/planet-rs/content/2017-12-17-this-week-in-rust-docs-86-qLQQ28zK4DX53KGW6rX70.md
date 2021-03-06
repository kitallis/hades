+++
title = "This Week in Rust Docs 86"
date = "2017-12-17T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-86"
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
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, showed <a href="https://github.com/rust-lang/rust/pull/46350">closure signature on type errors</a> and fixed <a href="https://github.com/rust-lang/rust/pull/46720">incorrect type mismatch label pointing at return type</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/46461">type error method suggestions use whitelisted identity-like conversions</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/46327">RELEASES.md for 1.23.0</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/MaloJaffre">@MaloJaffre</a> added <a href="https://github.com/rust-lang/rust/pull/46278">compiler docs testing to CI</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/46359">display of hidden types in rustdoc</a>, stabilized <a href="https://github.com/rust-lang/rust/pull/46501">allow_fail flag test feature</a> and made <a href="https://github.com/rust-lang/rust/pull/46700">doc search more relevant</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> resolved <a href="https://github.com/rust-lang/rust/pull/46608">type on return type suggestion</a>, pointed <a href="https://github.com/rust-lang/rust/pull/46719">at var in short lived borrows instead of drop location</a> and pointed <a href="https://github.com/rust-lang/rust/pull/46633">at whole method call instead of args</a>.</li>
  <li><a href="https://github.com/canndrew">@canndrew</a> added <a href="https://github.com/rust-lang/rust/pull/46232">docs for never primitive</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/46248">one-time diagnostics for private enum variants glob reexport</a>.</li>
  <li><a href="https://github.com/matthewjasper">@matthewjasper</a> used <a href="https://github.com/rust-lang/rust/pull/46601">a better link for method resolution in Deref docs</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made <a href="https://github.com/rust-lang/rust/pull/46737">a better example in docs</a>.</li>
  <li><a href="https://github.com/rillian">@rillian</a> marked <a href="https://github.com/rust-lang/rust/pull/46411">ascii methods on primitive types stable in 1.23.0 not 1.21.0</a>.</li>
  <li><a href="https://github.com/emilio">@emilio</a> fixed <a href="https://github.com/rust-lang/rust/pull/46625">typo in infer README</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/46668">mobile doc style and improve search bar</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46672">type filter in rustdoc js</a> and fixed <a href="https://github.com/rust-lang/rust/pull/46611">switched types in type mismatch</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 19th of December 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>