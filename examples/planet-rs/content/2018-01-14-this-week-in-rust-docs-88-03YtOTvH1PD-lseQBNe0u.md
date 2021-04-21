+++
title = "This Week in Rust Docs 88"
date = "2018-01-14T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-88"
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

<p>The hoedown -&gt; pulldown migration moved forward by a step this last week: now the default renderer is pulldown.</p>

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
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, suggested <a href="https://github.com/rust-lang/rust/pull/47247">casting on numeric type error</a> and added <a href="https://github.com/rust-lang/rust/pull/47144">a custom error when moving arg outside of its closure</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> documented <a href="https://github.com/rust-lang/rust/pull/46962">std::os::raw</a>, better <a href="https://github.com/rust-lang/rust/pull/47120">Debug impl for io::Error</a>.</li>
  <li><a href="https://github.com/hellow554">@hellow554</a> added <a href="https://github.com/rust-lang/rust/pull/46938">kbd style tag to main.css in rustdoc</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vramana">@vramana</a> improved <a href="https://github.com/rust-lang/rust/pull/47020">error messages in the case of partial and collateral moves for NLL</a> and improved <a href="https://github.com/rust-lang/rust/pull/47093">move related error messages</a>.</li>
  <li><a href="https://github.com/etaoins">@etaoins</a> extended <a href="https://github.com/rust-lang/rust/pull/47424">macro name suggestion span</a>.</li>
  <li><a href="https://github.com/davidtwco">@davidtwco</a> fixed <a href="https://github.com/rust-lang/rust/pull/47329">bad error message when converting anonymous lifetime to <code class="highlighter-rouge">'static</code> for NLL</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> checked <a href="https://github.com/rust-lang/rust/pull/47423">for deadlinks from the summary during book generation</a>.</li>
  <li><a href="https://github.com/tspiteri">@tspiteri</a> showed <a href="https://github.com/rust-lang/rust/pull/47277">that <code class="highlighter-rouge">f32::log</code> and <code class="highlighter-rouge">f64::log</code> are not correctly rounded in docs</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> fixed <a href="https://github.com/rust-lang/rust/pull/47274">line offsets for doctests</a> and implemented <a href="https://github.com/rust-lang/rust/pull/47046">RFC 1946 - intra-rustdoc links</a>.</li>
  <li><a href="https://github.com/carols10cents">@carols10cents</a> standardized <a href="https://github.com/rust-lang/rust/pull/47404">on “re-export” rather than “reexport”</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> populated <a href="https://github.com/rust-lang/rust/pull/47313">external_traits with traits only seen in impls in rustdoc</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/47286">RELEASES.md for 1.24.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilized <a href="https://github.com/rust-lang/rust/pull/46501">allow_fail flag test feature</a>, added <a href="https://github.com/rust-lang/rust/pull/47413">error code for unstable feature errors</a>, test <a href="https://github.com/rust-lang/rust/pull/47250">rustdoc js</a> and switched <a href="https://github.com/rust-lang/rust/pull/47398">to pulldown as default markdown renderer</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> provided <a href="https://github.com/rust-lang/rust/pull/47171">suggestion when trying to use method on numeric literal</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/46461">type error method suggestions use whitelisted identity-like conversions</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47210">the doc-comment-decoration-trimming edge-case rustdoc ICE</a>.</li>
  <li><a href="https://github.com/keatinge">@keatinge</a> added <a href="https://github.com/rust-lang/rust/pull/47232">help message for incorrect pattern syntax</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/47039">missing src links for generic impls on trait pages in rustdoc</a>.</li>
  <li><a href="https://github.com/whentze">@whentze</a> fixed <a href="https://github.com/rust-lang/rust/pull/47357">docs for OsStr</a>.</li>
  <li><a href="https://github.com/overvenus">@overvenus</a> fixed <a href="https://github.com/rust-lang/rust/pull/47375">examples of Duration::subsec_millis and Duration::subsec_micros</a>.</li>
  <li><a href="https://github.com/alercah">@alercah</a> fixed <a href="https://github.com/rust-lang/rust/pull/47340">typo</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 16th of January 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>