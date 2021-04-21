+++
title = "This Week in Rust Docs 87"
date = "2018-01-07T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-87"
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

<p>Happy new year! The last year has been very intense in the Rust documentation world:</p>

<ul>
  <li>We improved the documentation search (both display and search itself).</li>
  <li>We improved the documentation display (it now fully works on mobile devices!).</li>
  <li>We added more information into the documentation (the crate version, if the return type is implements any of Iterator/Read/Write trait, and a lot more…).</li>
  <li>We added the possibility to include external files in documentation.</li>
  <li>We (finally!) started the migration of our markdown renderer from hoedown to pulldown.</li>
  <li>We added examples to <strong>every</strong> function/methods/types to allow readers to understand more quickly.</li>
  <li>We added a lint (<code class="highlighter-rouge">missing_docs</code>) to allow you to never miss an item not being documented.</li>
  <li>And a lot more of awesome changes!</li>
</ul>

<p>Maybe a blog post will be wrote to sum all this up, we’ll see.</p>

<p>Now time to catch up the two missing weeks!</p>

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
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, suggested <a href="https://github.com/rust-lang/rust/pull/47247">casting on numeric type error</a>, provided <a href="https://github.com/rust-lang/rust/pull/47171">suggestion when trying to use method on numeric literal</a> and added <a href="https://github.com/rust-lang/rust/pull/47144">a custom error when moving arg outside of its closure</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/46461">type error method suggestions use whitelisted identity-like conversions</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47210">the doc-comment-decoration-trimming edge-case rustdoc ICE</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/keatinge">@keatinge</a> added <a href="https://github.com/rust-lang/rust/pull/47232">help message for incorrect pattern syntax</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> documented <a href="https://github.com/rust-lang/rust/pull/46962">std::os::raw</a>, better <a href="https://github.com/rust-lang/rust/pull/47120">Debug impl for io::Error</a>.</li>
  <li><a href="https://github.com/hellow554">@hellow554</a> added <a href="https://github.com/rust-lang/rust/pull/46938">kbd style tag to main.css in rustdoc</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vramana">@vramana</a> improved <a href="https://github.com/rust-lang/rust/pull/47020">error messages in the case of partial and collateral moves for NLL</a> and improved <a href="https://github.com/rust-lang/rust/pull/47093">move related error messages</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/47039">missing src links for generic impls on trait pages in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilized <a href="https://github.com/rust-lang/rust/pull/46501">allow_fail flag test feature</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/46327">RELEASES.md for 1.23.0</a>.</li>
  <li><a href="https://github.com/MaloJaffre">@MaloJaffre</a> added <a href="https://github.com/rust-lang/rust/pull/46278">compiler docs testing to CI</a>.</li>
  <li><a href="https://github.com/tspiteri">@tspiteri</a> improved <a href="https://github.com/rust-lang/rust/pull/46947">None condition doc for <code class="highlighter-rouge">checked_div</code> and <code class="highlighter-rouge">checked_rem</code> in docs</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> wrote <a href="https://github.com/rust-lang/rust/pull/47217">examples for {BTree,Hash}Set::{get,replace,take}</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> added <a href="https://github.com/rust-lang/rust/pull/47064">a tidy check for missing or too many trailing newlines</a>.</li>
  <li><a href="https://github.com/SergioBenitez">@SergioBenitez</a> clarified <a href="https://github.com/rust-lang/rust/pull/47216">appending behavior of ‘io::Read::read_to_string()’</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> minor <a href="https://github.com/rust-lang/rust/pull/46987">rewrite of env::current_exe docs; clarified symlinks</a> and documented <a href="https://github.com/rust-lang/rust/pull/47145">when LineWriter flushes; document errors for into_inner</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> corrected <a href="https://github.com/rust-lang/rust/pull/47030">a few stability attributes</a>.</li>
  <li><a href="https://github.com/dzamlo">@dzamlo</a> fixed <a href="https://github.com/rust-lang/rust/pull/47198">an error in std::process documentation</a>.</li>
  <li><a href="https://github.com/varkor">@varkor</a> fixed <a href="https://github.com/rust-lang/rust/pull/47079">doc typo for is_ascii_graphic</a>.</li>
  <li><a href="https://github.com/Sogomn">@Sogomn</a> defocused <a href="https://github.com/rust-lang/rust/pull/47134">search bar in rustdoc pages</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> reworded <a href="https://github.com/rust-lang/rust/pull/47124">reason for move note</a> and reworded <a href="https://github.com/rust-lang/rust/pull/47098">trying to operate in immutable fields</a>.</li>
  <li><a href="https://github.com/mark-i-m">@mark-i-m</a> fixed <a href="https://github.com/rust-lang/rust/pull/47107">typo</a>.</li>
  <li><a href="https://github.com/fschutt">@fschutt</a> improved <a href="https://github.com/rust-lang/rust/pull/47052">error messages for linking failure</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/46359">display of hidden types in rustdoc</a>, made <a href="https://github.com/rust-lang/rust/pull/46700">doc search more relevant</a> and fixed <a href="https://github.com/rust-lang/rust/pull/47202">search bar defocus</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 9th of January 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>