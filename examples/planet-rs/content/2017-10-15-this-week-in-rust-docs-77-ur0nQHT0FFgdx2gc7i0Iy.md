+++
title = "This Week in Rust Docs 77"
date = "2017-10-15T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-77"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> made <a href="https://github.com/rust-lang/rust/pull/44613">rustdoc optimizations</a>, included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a> and showed <a href="https://github.com/rust-lang/rust/pull/45039">in docs whether the return type of a function impls Iterator/Read/Write</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/44970">syntax when finding method on array</a>, warned <a href="https://github.com/rust-lang/rust/pull/44881">when assigning a block that doesn’t have an implicit return</a> and referred <a href="https://github.com/rust-lang/rust/pull/44642">to types using the local identifier</a>.</li>
  <li><a href="https://github.com/sunjay">@sunjay</a> documented <a href="https://github.com/rust-lang/rust/pull/45098">the process for when rustfmt/rls break</a>.</li>
  <li><a href="https://github.com/jacwah">@jacwah</a> mentionned <a href="https://github.com/rust-lang/rust/pull/45082">Clone and refs in –explain E0382</a>.</li>
  <li><a href="https://github.com/Nemo157">@Nemo157</a> rendered <a href="https://github.com/rust-lang/rust/pull/44994">cfg(feature) requirements in documentation</a>.</li>
  <li><a href="https://github.com/goffrie">@goffrie</a> provided <a href="https://github.com/rust-lang/rust/pull/45123">the full span of method calls to <code class="highlighter-rouge">check_argument_types</code></a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> made <a href="https://github.com/rust-lang/rust/pull/45181">a list of all lang items in unstable book</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expanded <a href="https://github.com/rust-lang/rust/pull/45227">docs/examples for TCP <code class="highlighter-rouge">set_nonblocking</code> methods</a>.</li>
  <li><a href="https://github.com/0xAX">@0xAX</a> fixed <a href="https://github.com/rust-lang/rust/pull/45264">a typo in src/bootstrap/README.md</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> fixed <a href="https://github.com/rust-lang/rust/pull/45217">out of date unstable book entries for <code class="highlighter-rouge">alloc_*</code> features</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a>, improved <a href="https://github.com/rust-lang/rust/pull/45187">sidebar rendering and added methods list</a>, save <a href="https://github.com/rust-lang/rust/pull/45281">selected search tab</a>, saved <a href="https://github.com/rust-lang/rust/pull/45288">the highlighted item when switching tab</a>, removed <a href="https://github.com/rust-lang/rust/pull/45280">terribly useless and problematic margin when searching on mobile</a>, fixed <a href="https://github.com/rust-lang/rust/pull/45289">arrow display</a>, hid <a href="https://github.com/rust-lang/rust/pull/45290">help when search bar is focused</a> and limited <a href="https://github.com/rust-lang/rust/pull/45212">the sidebar height</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> documented <a href="https://github.com/rust-lang/rust/pull/44969">trait impls when the type appears in the trait’s generics</a> and let <a href="https://github.com/rust-lang/rust/pull/44989">rustdoc print the crate version into docs</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> improved <a href="https://github.com/rust-lang/rust/pull/44867">doc-test: in Markdown tests, Use all of <code class="highlighter-rouge">&lt;h1&gt;</code> to <code class="highlighter-rouge">&lt;h6&gt;</code> as the test name</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> improved <a href="https://github.com/rust-lang/rust/pull/44855">docs for CStr, CString, OsStr, OsString</a>.</li>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> don’t <a href="https://github.com/rust-lang/rust/pull/44962">encourage people to ignore threading errors in the docs</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> upgraded <a href="https://github.com/rust-lang/rust/pull/45172">some comments to doc comments</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> fixed <a href="https://github.com/rust-lang/rust/pull/45171">a mistake in release notes for 1.21.0</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> made <a href="https://github.com/rust-lang/rust/pull/45069">a better error message for missing tuple pattern in args</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> increased <a href="https://github.com/rust-lang/rust/pull/45245">padding between consecutive impls in rustdoc</a>.</li>
  <li><a href="https://github.com/Gankro">@Gankro</a> clarified <a href="https://github.com/rust-lang/rust/pull/45253">how needs_drop is conservative</a>.</li>
  <li><a href="https://github.com/laumann">@laumann</a> added <a href="https://github.com/rust-lang/rust/pull/45173">suggestions for misspelled labels</a>.</li>
  <li><a href="https://github.com/Badel2">@Badel2</a> made <a href="https://github.com/rust-lang/rust/pull/45178">a better error message for comma after base struct</a>.</li>
  <li><a href="https://github.com/jean-lourenco">@jean-lourenco</a> improved <a href="https://github.com/rust-lang/rust/pull/45122">compile error output when using arguments instead of types</a>.</li>
  <li><a href="https://github.com/johnthagen">@johnthagen</a> fixed <a href="https://github.com/rust-lang/rust/pull/45116">typos</a> and clarified <a href="https://github.com/rust-lang/rust/pull/45136">RAM usage during build in README</a>.</li>
  <li><a href="https://github.com/tinaun">@tinaun</a> documented <a href="https://github.com/rust-lang/rust/pull/45166">a few more unstable feature gates</a>.</li>
  <li><a href="https://github.com/camsteffen">@camsteffen</a> fixed <a href="https://github.com/rust-lang/rust/pull/45105">rustc documentation typo</a>.</li>
  <li><a href="https://github.com/rkruppe">@rkruppe</a> fixed <a href="https://github.com/rust-lang/rust/pull/45089">typo in codegen test</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/45055">tabs for search for better information access</a>, usize <a href="https://github.com/rust-lang/rust/pull/45133">index message for vec</a> and made <a href="https://github.com/rust-lang/rust/pull/45240">improvements in the mobile sidebar</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 18th of October 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>