+++
title = "This Week in Rust Docs 96"
date = "2018-03-11T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-96"
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
  <li><a href="https://github.com/estebank">@estebank</a> reworded <a href="https://github.com/rust-lang/rust/pull/48138">E0044 and message for <code class="highlighter-rouge">!Send</code> types</a> and suggested <a href="https://github.com/rust-lang/rust/pull/48288">setting associated type when type argument is given instead</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/NovemberZulu">@NovemberZulu</a> rephrased <a href="https://github.com/rust-lang/rust/pull/48201">UnsafeCell doc</a>.</li>
  <li><a href="https://github.com/FraGag">@FraGag</a> improved <a href="https://github.com/rust-lang/rust/pull/48171">documentation of Clone and Copy implementors</a>.</li>
  <li><a href="https://github.com/jethrogb">@jethrogb</a> clarified <a href="https://github.com/rust-lang/rust/pull/48480">interfaction between File::set_len and file cursor</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/48374">RELEASES.md for 1.25.0</a>.</li>
  <li><a href="https://github.com/tinaun">@tinaun</a> removed <a href="https://github.com/rust-lang/rust/pull/48709">erroneous error message when checking impl trait params</a>.</li>
  <li><a href="https://github.com/ehuss">@ehuss</a> added <a href="https://github.com/rust-lang/rust/pull/48706">crate name to “main function not found” error message</a>.</li>
  <li><a href="https://github.com/focusaurus">@focusaurus</a> saved <a href="https://github.com/rust-lang/rust/pull/48631">state of top-level collapse toggle widget in rust docs</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> modified <a href="https://github.com/rust-lang/rust/pull/48914">E0389 error message</a>.</li>
  <li><a href="https://github.com/phansch">@phansch</a> added <a href="https://github.com/rust-lang/rust/pull/48361">more precise suggestion for non_upper_case_globals</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> exposed <a href="https://github.com/rust-lang/rust/pull/48759">#[target_feature] attributes as doc(cfg) flags in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48173">error codes for libsyntax_ext</a>, updated <a href="https://github.com/rust-lang/rust/pull/48559">rustc –explain sentence</a>, put <a href="https://github.com/rust-lang/rust/pull/48684">back ui json check</a>, added <a href="https://github.com/rust-lang/rust/pull/48596">warning for invalid start of code blocks in rustdoc</a>, removed <a href="https://github.com/rust-lang/rust/pull/48898">auto trait implementation section when empty</a>, added <a href="https://github.com/rust-lang/rust/pull/48877">missing urls</a> and fixed <a href="https://github.com/rust-lang/rust/pull/48831">blink when main theme is selected</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/zilbuz">@zilbuz</a> showed <a href="https://github.com/rust-lang/rust/pull/47574">the used type variable when issuing a “can’t use type parameters from outer function” error message</a>.</li>
  <li><a href="https://github.com/christianpoveda">@christianpoveda</a> made <a href="https://github.com/rust-lang/rust/pull/48474">new Cell docs</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> made <a href="https://github.com/rust-lang/rust/pull/48509">slight modification to the as_ref example of std::option::Option</a>.</li>
  <li><a href="https://github.com/flip1995">@flip1995</a> suggested <a href="https://github.com/rust-lang/rust/pull/48432">type for overflowing bin/hex-literals</a>.</li>
  <li><a href="https://github.com/lukaslueg">@lukaslueg</a> fixed <a href="https://github.com/rust-lang/rust/pull/48403">spelling s/casted/cast/</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> warned <a href="https://github.com/rust-lang/rust/pull/48326">about ignored generic bounds in <code class="highlighter-rouge">for</code></a>.</li>
  <li><a href="https://github.com/scottmcm">@scottmcm</a> improved <a href="https://github.com/rust-lang/rust/pull/48618">docs and associated SUCCESS/FAILURE for process::ExitCode</a>.</li>
  <li><a href="https://github.com/Songbird0">@Songbird0</a> modified <a href="https://github.com/rust-lang/rust/pull/48856">part of <code class="highlighter-rouge">line!</code> documentation</a>, modified <a href="https://github.com/rust-lang/rust/pull/48857">part of <code class="highlighter-rouge">column!</code> documentation</a> and added <a href="https://github.com/rust-lang/rust/pull/48738">a potential cause raising <code class="highlighter-rouge">ParseIntError</code></a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48511">resource-suffix option for rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/48507">new warning for CStr::from_ptr</a>, multiple <a href="https://github.com/rust-lang/rust/pull/48755">rustdoc fixes</a>, fix <a href="https://github.com/rust-lang/rust/pull/48789">sidebar horizontal scroll</a> and raw <a href="https://github.com/rust-lang/rust/pull/48546">string error note</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 13th of March 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>