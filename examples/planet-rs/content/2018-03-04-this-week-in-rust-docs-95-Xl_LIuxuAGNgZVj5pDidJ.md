+++
title = "This Week in Rust Docs 95"
date = "2018-03-04T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-95"
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
  <li><a href="https://github.com/zilbuz">@zilbuz</a> showed <a href="https://github.com/rust-lang/rust/pull/47574">the used type variable when issuing a “can’t use type parameters from outer function” error message</a>.</li>
  <li><a href="https://github.com/christianpoveda">@christianpoveda</a> made <a href="https://github.com/rust-lang/rust/pull/48474">new Cell docs</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> made <a href="https://github.com/rust-lang/rust/pull/48509">slight modification to the as_ref example of std::option::Option</a>.</li>
  <li><a href="https://github.com/flip1995">@flip1995</a> suggested <a href="https://github.com/rust-lang/rust/pull/48432">type for overflowing bin/hex-literals</a>.</li>
  <li><a href="https://github.com/jethrogb">@jethrogb</a> clarified <a href="https://github.com/rust-lang/rust/pull/48480">interfaction between File::set_len and file cursor</a>.</li>
  <li><a href="https://github.com/lukaslueg">@lukaslueg</a> fixed <a href="https://github.com/rust-lang/rust/pull/48403">spelling s/casted/cast/</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> warned <a href="https://github.com/rust-lang/rust/pull/48326">about ignored generic bounds in <code class="highlighter-rouge">for</code></a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/48374">RELEASES.md for 1.25.0</a>.</li>
  <li><a href="https://github.com/scottmcm">@scottmcm</a> improved <a href="https://github.com/rust-lang/rust/pull/48618">docs and associated SUCCESS/FAILURE for process::ExitCode</a> and produced <a href="https://github.com/rust-lang/rust/pull/48544">better size_hints from Flatten &amp; FlatMap</a>.</li>
  <li><a href="https://github.com/tinaun">@tinaun</a> removed <a href="https://github.com/rust-lang/rust/pull/48709">erroneous error message when checking impl trait params</a>.</li>
  <li><a href="https://github.com/ehuss">@ehuss</a> added <a href="https://github.com/rust-lang/rust/pull/48706">crate name to “main function not found” error message</a>.</li>
  <li><a href="https://github.com/focusaurus">@focusaurus</a> remember <a href="https://github.com/rust-lang/rust/pull/48631">state of top-level collapse toggle widget in rust docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48511">resource-suffix option for rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/48173">error codes for libsyntax_ext</a>, added <a href="https://github.com/rust-lang/rust/pull/48507">new warning for CStr::from_ptr</a>, updated <a href="https://github.com/rust-lang/rust/pull/48559">rustc –explain sentence</a>, put <a href="https://github.com/rust-lang/rust/pull/48684">back ui json check</a> and added <a href="https://github.com/rust-lang/rust/pull/48596">warning for invalid start of code blocks in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> provided <a href="https://github.com/rust-lang/rust/pull/48338">context for missing comma in match arm and if statement without block</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> added <a href="https://github.com/rust-lang/rust/pull/47894">foldable impl blocks in rustdoc</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/48283">readme for librustdoc</a>.</li>
  <li><a href="https://github.com/remexre">@remexre</a> fixed <a href="https://github.com/rust-lang/rust/pull/48529">docs for ASCII functions to no longer claim U+0021 is ‘@’</a>.</li>
  <li><a href="https://github.com/mark-i-m">@mark-i-m</a> splitted <a href="https://github.com/rust-lang/rust/pull/48446">E0404 to E0909; get rid of E0245</a> and started <a href="https://github.com/rust-lang/rust/pull/48479">moving to the rustc guide!</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/48328">“It is an error to…” wording for zero-duration behaviors</a>.</li>
  <li><a href="https://github.com/Centril">@Centril</a> documented <a href="https://github.com/rust-lang/rust/pull/48365">panics in Clone, PartialEq, PartialOrd, Ord for RefCell</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> removed <a href="https://github.com/rust-lang/rust/pull/48680">production TOCs for doc markdown files</a>.</li>
  <li><a href="https://github.com/teiesti">@teiesti</a> fixed <a href="https://github.com/rust-lang/rust/pull/48626">link to rustc guide in README.md</a>.</li>
  <li><a href="https://github.com/pthariensflame">@pthariensflame</a> made a <a href="https://github.com/rust-lang/rust/pull/48603">minor grammatical/style fix in docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added back <a href="https://github.com/rust-lang/rust/pull/48337">rustc explain</a>, added <a href="https://github.com/rust-lang/rust/pull/48381">rustdoc theme securities</a> and fixed <a href="https://github.com/rust-lang/rust/pull/48473">auto trait impl rustdoc ice</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 6th of March 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>