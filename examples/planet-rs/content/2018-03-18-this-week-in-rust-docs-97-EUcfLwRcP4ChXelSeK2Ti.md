+++
title = "This Week in Rust Docs 97"
date = "2018-03-18T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-97"
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

<p>Nothing interesting enough.</p>

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
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/FraGag">@FraGag</a> improved <a href="https://github.com/rust-lang/rust/pull/48171">documentation of Clone and Copy implementors</a>.</li>
  <li><a href="https://github.com/tinaun">@tinaun</a> removed <a href="https://github.com/rust-lang/rust/pull/48709">erroneous error message when checking impl trait params</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> modified <a href="https://github.com/rust-lang/rust/pull/48914">E0389 error message</a>.</li>
  <li><a href="https://github.com/phansch">@phansch</a> added <a href="https://github.com/rust-lang/rust/pull/48361">more precise suggestion for non_upper_case_globals</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> exposed <a href="https://github.com/rust-lang/rust/pull/48759">#[target_feature] attributes as doc(cfg) flags in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/49028">an “unstable features” chapter to the rustdoc book</a> and suppressed <a href="https://github.com/rust-lang/rust/pull/49064">the default allow(unused) under –display-warnings in rustdoctest</a>.</li>
  <li><a href="https://github.com/csmoe">@csmoe</a> improved <a href="https://github.com/rust-lang/rust/pull/49104">error message of inner attribute syntax</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> suggested <a href="https://github.com/rust-lang/rust/pull/48858"><code class="highlighter-rouge">!</code> instead of erroneous <code class="highlighter-rouge">not</code> on if/while block parse failure</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> added <a href="https://github.com/rust-lang/rust/pull/49105">an example of lossy decoding to str::Utf8Error docs</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> documented <a href="https://github.com/rust-lang/rust/pull/48932">when types have OS-dependent sizes</a>.</li>
  <li><a href="https://github.com/ysiraichi">@ysiraichi</a> suggested <a href="https://github.com/rust-lang/rust/pull/48834">removing <code class="highlighter-rouge">&amp;</code>s</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> improved <a href="https://github.com/rust-lang/rust/pull/48909">lint for type alias bounds</a>.</li>
  <li><a href="https://github.com/klnusbaum">@klnusbaum</a> provided <a href="https://github.com/rust-lang/rust/pull/48708">better borrow checker error message</a> and renamed <a href="https://github.com/rust-lang/rust/pull/49035">epoch to edition</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48173">error codes for libsyntax_ext</a>, added <a href="https://github.com/rust-lang/rust/pull/48596">warning for invalid start of code blocks in rustdoc</a>, and made <a href="https://github.com/rust-lang/rust/pull/49029">Atomic doc examples specific to each type</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> reworded <a href="https://github.com/rust-lang/rust/pull/48138">E0044 and message for <code class="highlighter-rouge">!Send</code> types</a>.</li>
  <li><a href="https://github.com/NovemberZulu">@NovemberZulu</a> rephrased <a href="https://github.com/rust-lang/rust/pull/48201">UnsafeCell doc</a>.</li>
  <li><a href="https://github.com/jethrogb">@jethrogb</a> clarified <a href="https://github.com/rust-lang/rust/pull/48480">interfaction between File::set_len and file cursor</a>.</li>
  <li><a href="https://github.com/ehuss">@ehuss</a> added <a href="https://github.com/rust-lang/rust/pull/48706">crate name to “main function not found” error message</a>.</li>
  <li><a href="https://github.com/focusaurus">@focusaurus</a> saved <a href="https://github.com/rust-lang/rust/pull/48631">state of top-level collapse toggle widget in rust docs</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> removed <a href="https://github.com/rust-lang/rust/pull/49042">unnecessary backtick in error message E0307 (invalid self type)</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> re-enabled <a href="https://github.com/rust-lang/rust/pull/49090">testing librustdoc</a> and tweaked <a href="https://github.com/rust-lang/rust/pull/48964">code fences in the rustdoc book</a>.</li>
  <li><a href="https://github.com/mark-i-m">@mark-i-m</a> moved <a href="https://github.com/rust-lang/rust/pull/48972">librustdoc readme to rustc guide</a> and moved <a href="https://github.com/rust-lang/rust/pull/48971">librustc_typeck READMEs to rustc guide</a>.</li>
  <li><a href="https://github.com/Songbird0">@Songbird0</a> added <a href="https://github.com/rust-lang/rust/pull/48961">example of use of assertions in rustdoc</a> and improved <a href="https://github.com/rust-lang/rust/pull/48853"><code class="highlighter-rouge">AddrParseError</code> documentation</a>.</li>
  <li><a href="https://github.com/jsgf">@jsgf</a> clarified <a href="https://github.com/rust-lang/rust/pull/48991">usage message for –remap-path-prefix</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> put <a href="https://github.com/rust-lang/rust/pull/48684">back ui json check</a>, removed <a href="https://github.com/rust-lang/rust/pull/48898">auto trait implementation section when empty</a>, added <a href="https://github.com/rust-lang/rust/pull/48877">missing urls</a>, fixed <a href="https://github.com/rust-lang/rust/pull/48831">blink when main theme is selected</a>, added <a href="https://github.com/rust-lang/rust/pull/48970">missing examples</a> and added <a href="https://github.com/rust-lang/rust/pull/48954">missing links</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 20th of March 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>