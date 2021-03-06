+++
title = "This Week in Rust Docs 57"
date = "2017-05-21T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-57"
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

<p>After a long debate, it has been decided to keep hoedown testing/rendering by default in rustdoc. However, you can test pulldown by running rustdoc with <code class="highlighter-rouge">-Z unstable-options enable-commonmark</code>.</p>

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
  <li><a href="https://github.com/F001">@F001</a> fixed <a href="https://github.com/rust-lang/rust/pull/41848">comma after struct update syntax</a>.</li>
  <li><a href="https://github.com/gamazeps">@gamazeps</a> added <a href="https://github.com/rust-lang/rust/pull/41980"><code class="highlighter-rouge">'static</code> and <code class="highlighter-rouge">Send</code> constraints explanations to <code class="highlighter-rouge">thread::spawn</code></a> and expanded <a href="https://github.com/rust-lang/rust/pull/41981"><code class="highlighter-rouge">detach</code> documentation in <code class="highlighter-rouge">thread::JoinHande</code></a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/41953">releases notes for 1.18</a>.</li>
  <li><a href="https://github.com/neosilky">@neosilky</a> updated <a href="https://github.com/rust-lang/rust/pull/42131">to trait bounds CSS in rustdoc</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> introduced <a href="https://github.com/rust-lang/rust/pull/41968">‘ui-run’ test to compiletest</a>.</li>
  <li><a href="https://github.com/mjkillough">@mjkillough</a> documented <a href="https://github.com/rust-lang/rust/pull/42027">direct implementations on type aliases.</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/42122">a few entries to the Unstable Book.</a>.</li>
  <li><a href="https://github.com/euclio">@euclio</a> removed <a href="https://github.com/rust-lang/rust/pull/42120">“much” from unicode diagnostic</a>.</li>
  <li><a href="https://github.com/alex-ozdemir">@alex-ozdemir</a> made <a href="https://github.com/rust-lang/rust/pull/42076">clearer error message for Duplicate Definition</a>.</li>
  <li><a href="https://github.com/Idolf">@Idolf</a> supported <a href="https://github.com/rust-lang/rust/pull/41747">#![deny(missing_docs)] together with #[proc_macro_derive]</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41559">better error message when == operator is badly used</a>, made <a href="https://github.com/rust-lang/rust/pull/41700">–extend-css stable</a> and added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> made <a href="https://github.com/rust-lang/rust/pull/41489">unsatisfied trait bounds note multiline</a>.</li>
  <li><a href="https://github.com/abonander">@abonander</a> documented <a href="https://github.com/rust-lang/rust/pull/41476">the <code class="highlighter-rouge">proc_macro</code> feature in the Unstable Book</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> added <a href="https://github.com/rust-lang/rust/pull/41907">lint for unused macros</a>.</li>
  <li><a href="https://github.com/gamazeps">@gamazeps</a> explained <a href="https://github.com/rust-lang/rust/pull/41982">why <code class="highlighter-rouge">thread::yield_now</code> could be used</a>, improved <a href="https://github.com/rust-lang/rust/pull/41994"><code class="highlighter-rouge">thread::Builder</code>’s doc</a> and added <a href="https://github.com/rust-lang/rust/pull/41995">links to the <code class="highlighter-rouge">thread::LocalKey</code> doc</a>.</li>
  <li><a href="https://github.com/dhardy">@dhardy</a> added <a href="https://github.com/rust-lang/rust/pull/41857">loop_break_value documentation for The Book</a>.</li>
  <li><a href="https://github.com/excaliburHisSheath">@excaliburHisSheath</a> improved <a href="https://github.com/rust-lang/rust/pull/41870">docs in os::windows::ffi and os::windows::fs</a>.</li>
  <li><a href="https://github.com/froydnj">@froydnj</a> fixed <a href="https://github.com/rust-lang/rust/pull/41859">confusion about parts required for float formatting</a>.</li>
  <li><a href="https://github.com/cuviper">@cuviper</a> gave <a href="https://github.com/rust-lang/rust/pull/42092">a nicer error for non-Unicode arguments to rustc and rustdoc</a>.</li>
  <li><a href="https://github.com/maccoda">@maccoda</a> improved <a href="https://github.com/rust-lang/rust/pull/42091">std::env docs</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/42096">implementors list in Rustdoc’s javascript</a>, corrected <a href="https://github.com/rust-lang/rust/pull/42111">some stability versions</a> and displayed <a href="https://github.com/rust-lang/rust/pull/42001"><code class="highlighter-rouge">extern "C" fn</code> instead of <code class="highlighter-rouge">extern fn</code> in rustdoc</a>.</li>
  <li><a href="https://github.com/citizen428">@citizen428</a> added <a href="https://github.com/rust-lang/rust/pull/42024">documentation for <code class="highlighter-rouge">ExitStatus</code></a>.</li>
  <li><a href="https://github.com/seeekr">@seeekr</a> fixed <a href="https://github.com/rust-lang/rust/pull/42079">typo in libstd/sync/mpsc/mod.rs docs</a>.</li>
  <li><a href="https://github.com/pravic">@pravic</a> fixed <a href="https://github.com/rust-lang/rust/pull/42080">regression introduced by jQuery removal</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> improved <a href="https://github.com/rust-lang/rust/pull/42070">std::env docs</a>.</li>
  <li><a href="https://github.com/faso">@faso</a> fixed <a href="https://github.com/rust-lang/rust/pull/42028">a typo in contributing.md</a>.</li>
  <li><a href="https://github.com/rap2hpoutre">@rap2hpoutre</a> improved <a href="https://github.com/rust-lang/rust/pull/42011">collapse toggle render (css)</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41772">help message if a FnOnce is moved</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 24th of May 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>