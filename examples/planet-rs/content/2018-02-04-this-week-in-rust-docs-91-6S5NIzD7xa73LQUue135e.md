+++
title = "This Week in Rust Docs 91"
date = "2018-02-04T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-91"
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
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/47613">filtering options to <code class="highlighter-rouge">rustc_on_unimplemented</code></a> and added <a href="https://github.com/rust-lang/rust/pull/47843"><code class="highlighter-rouge">-Zteach</code> documentation</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> documented <a href="https://github.com/rust-lang/rust/pull/46962">std::os::raw</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vramana">@vramana</a> improved <a href="https://github.com/rust-lang/rust/pull/47020">error messages in the case of partial and collateral moves for NLL</a> and improved <a href="https://github.com/rust-lang/rust/pull/47093">move related error messages</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/47496">documentation from doc(include) to analysis data</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/47753">rust book</a>.</li>
  <li><a href="https://github.com/varkor">@varkor</a> documented <a href="https://github.com/rust-lang/rust/pull/47547">the behaviour of infinite iterators on potentially-computable methods</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> fixed <a href="https://github.com/rust-lang/rust/pull/47959">rustdoc ICE on macros defined within functions</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/47958">shared file handler behavior of File::try_clone</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> added <a href="https://github.com/rust-lang/rust/pull/47894">foldable impl blocks in rustdoc</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> fixed <a href="https://github.com/rust-lang/rust/pull/47731">how paths are printed by error messages during bootstrap</a>.</li>
  <li><a href="https://github.com/Aaron1011">@Aaron1011</a> generated <a href="https://github.com/rust-lang/rust/pull/47833">documentation for auto-trait impls</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> declined <a href="https://github.com/rust-lang/rust/pull/47896">to lint technically-unnecessary parens in function or method arguments inside of nested macros</a> and corrected <a href="https://github.com/rust-lang/rust/pull/47922">unused field pattern suggestions</a>.</li>
  <li><a href="https://github.com/PramodBisht">@PramodBisht</a> changed <a href="https://github.com/rust-lang/rust/pull/47806">color of struct link from #ff794d to #2dbfb8 for Rust docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/47761">tests for themes</a>, fixed <a href="https://github.com/rust-lang/rust/pull/47810">themes rendering issues on mobile</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47862">const evaluation ICE in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, included <a href="https://github.com/rust-lang/rust/pull/47465">space in suggestion <code class="highlighter-rouge">mut</code> in bindings</a>, tweaked <a href="https://github.com/rust-lang/rust/pull/47791">presentation on lifetime trait mismatch</a>, minimized <a href="https://github.com/rust-lang/rust/pull/47942">weird spans involving macro context</a> and suggested <a href="https://github.com/rust-lang/rust/pull/47829">removing value from <code class="highlighter-rouge">break</code> when invalid</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/varkor">@varkor</a> added <a href="https://github.com/rust-lang/rust/pull/47780">line numbers and columns to error messages spanning multiple files</a>.</li>
  <li><a href="https://github.com/jimmantooth">@jimmantooth</a> improved <a href="https://github.com/rust-lang/rust/pull/47515">punctuation and clarity</a>.</li>
  <li><a href="https://github.com/vmx">@vmx</a> fixed <a href="https://github.com/rust-lang/rust/pull/47916">lang items box example code</a>.</li>
  <li><a href="https://github.com/etaoins">@etaoins</a> improved <a href="https://github.com/rust-lang/rust/pull/47914">char escaping in lexer messages</a>, fixed <a href="https://github.com/rust-lang/rust/pull/47794">ICE on const eval of union field</a> and avoided <a href="https://github.com/rust-lang/rust/pull/47677">underflow in render_source_line</a>.</li>
  <li><a href="https://github.com/euclio">@euclio</a> used <a href="https://github.com/rust-lang/rust/pull/47838">correct casing for rename suggestions</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> documented <a href="https://github.com/rust-lang/rust/pull/47839">that <code class="highlighter-rouge">Index</code> ops can panic on <code class="highlighter-rouge">HashMap</code> &amp; <code class="highlighter-rouge">BTreeMap</code></a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/47855">link title rendering with hoedown in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/47951">ugly hover in sidebar</a> and updated <a href="https://github.com/rust-lang/rust/pull/47876">associated constants error message</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 6th of February 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>