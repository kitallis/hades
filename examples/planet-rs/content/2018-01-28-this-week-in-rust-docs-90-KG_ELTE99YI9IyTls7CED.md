+++
title = "This Week in Rust Docs 90"
date = "2018-01-28T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-90"
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

<p>This week was a big one: the whole new <a href="https://github.com/rust-lang/rust/pull/47046">url system</a> and the <a href="https://github.com/rust-lang/rust/pull/47620">multiple themes support</a> have been added in rustdoc!</p>

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
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, added <a href="https://github.com/rust-lang/rust/pull/47613">filtering options to <code class="highlighter-rouge">rustc_on_unimplemented</code></a>, added <a href="https://github.com/rust-lang/rust/pull/46961">env flags <code class="highlighter-rouge">RUSTC_COLOR</code> and <code class="highlighter-rouge">RUSTC_ERROR_FORMAT</code></a>, included <a href="https://github.com/rust-lang/rust/pull/47465">space in suggestion <code class="highlighter-rouge">mut</code> in bindings</a>, tweaked <a href="https://github.com/rust-lang/rust/pull/47818">error message when reborrowing <code class="highlighter-rouge">&amp;mut self</code> as <code class="highlighter-rouge">mut</code></a> and tweaked <a href="https://github.com/rust-lang/rust/pull/47791">presentation on lifetime trait mismatch</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> documented <a href="https://github.com/rust-lang/rust/pull/46962">std::os::raw</a> and move <a href="https://github.com/rust-lang/rust/pull/46666">Duration to libcore</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vramana">@vramana</a> improved <a href="https://github.com/rust-lang/rust/pull/47020">error messages in the case of partial and collateral moves for NLL</a> and improved <a href="https://github.com/rust-lang/rust/pull/47093">move related error messages</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/47286">RELEASES.md for 1.24.0</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/47496">documentation from doc(include) to analysis data</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/47753">rust book</a>.</li>
  <li><a href="https://github.com/varkor">@varkor</a> added <a href="https://github.com/rust-lang/rust/pull/47780">line numbers and columns to error messages spanning multiple files</a> and documented <a href="https://github.com/rust-lang/rust/pull/47547">the behaviour of infinite iterators on potentially-computable methods</a>.</li>
  <li><a href="https://github.com/jimmantooth">@jimmantooth</a> improved <a href="https://github.com/rust-lang/rust/pull/47515">punctuation and clarity</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/47761">tests for themes</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47810">themes rendering issues on mobile</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/47247">casting on numeric type error</a>, added <a href="https://github.com/rust-lang/rust/pull/47144">a custom error when moving arg outside of its closure</a>, removed <a href="https://github.com/rust-lang/rust/pull/47534">private traits suggestions on missing method</a>, correctly <a href="https://github.com/rust-lang/rust/pull/47767">formatted <code class="highlighter-rouge">extern crate</code> conflict resolution help</a>, pointed <a href="https://github.com/rust-lang/rust/pull/47690">at the “head” expression for E0277 on <code class="highlighter-rouge">for</code> loops, point</a> and pointed <a href="https://github.com/rust-lang/rust/pull/47691">at unknown lang item attribute</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> checked <a href="https://github.com/rust-lang/rust/pull/47423">for deadlinks from the summary during book generation</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> implemented <a href="https://github.com/rust-lang/rust/pull/47046">RFC 1946 - intra-rustdoc links</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47701">intra-doc-links</a>.</li>
  <li><a href="https://github.com/sdroege">@sdroege</a> fixed <a href="https://github.com/rust-lang/rust/pull/47632">broken links to other slice functions in chunks/chunks_mut/exact_…</a>.</li>
  <li><a href="https://github.com/PieterPenninckx">@PieterPenninckx</a> made <a href="https://github.com/rust-lang/rust/pull/47595">small improvements to the documentation of VecDeque</a>.</li>
  <li><a href="https://github.com/astraw">@astraw</a> fixed <a href="https://github.com/rust-lang/rust/pull/47625">doctests for BTreeSet to use BTreeSet (not BTreeMap)</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/47675"> methods from #[doc(masked)] crates from the search index in rustdoc</a> and showed <a href="https://github.com/rust-lang/rust/pull/47672">when traits are auto traits in rustdoc</a>.</li>
  <li><a href="https://github.com/evelynmitchell">@evelynmitchell</a> fixed <a href="https://github.com/rust-lang/rust/pull/47717">invalid URL</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/47620">multiple themes support in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/47512">E0659 for ambiguous names</a>, fixed <a href="https://github.com/rust-lang/rust/pull/47721">experimental text display on default theme</a>, made <a href="https://github.com/rust-lang/rust/pull/47686">a few fixes for multiple themes support feature</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47667">quoted search</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 30th of January 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>