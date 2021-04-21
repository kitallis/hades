+++
title = "This Week in Rust Docs 58"
date = "2017-05-28T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-58"
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
  <li><a href="https://github.com/gamazeps">@gamazeps</a> expanded <a href="https://github.com/rust-lang/rust/pull/41981"><code class="highlighter-rouge">detach</code> documentation in <code class="highlighter-rouge">thread::JoinHande</code></a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/41953">releases notes for 1.18</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> introduced <a href="https://github.com/rust-lang/rust/pull/41968">‘ui-run’ test to compiletest</a>.</li>
  <li><a href="https://github.com/mjkillough">@mjkillough</a> documented <a href="https://github.com/rust-lang/rust/pull/42027">direct implementations on type aliases.</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/alex-ozdemir">@alex-ozdemir</a> made <a href="https://github.com/rust-lang/rust/pull/42076">clearer error message for Duplicate Definition</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> clarified <a href="https://github.com/rust-lang/rust/pull/42126">docs on implementing Into</a>.</li>
  <li><a href="https://github.com/pwoolcoc">@pwoolcoc</a> added <a href="https://github.com/rust-lang/rust/pull/42219"><code class="highlighter-rouge">allow_fail</code> test attribute</a>.</li>
  <li><a href="https://github.com/tommyip">@tommyip</a> explained <a href="https://github.com/rust-lang/rust/pull/42196">why a closure is <code class="highlighter-rouge">FnOnce</code> in closure errors</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> unified <a href="https://github.com/rust-lang/rust/pull/42260">the docs of PartialEq, PartialOrd and Ord</a> and clarified <a href="https://github.com/rust-lang/rust/pull/42252">the docs for align_of and its variants</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, updated <a href="https://github.com/rust-lang/rust/pull/42180">rustdoc man page</a> and added <a href="https://github.com/rust-lang/rust/pull/42264">new error codes</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/gamazeps">@gamazeps</a> added <a href="https://github.com/rust-lang/rust/pull/41980"><code class="highlighter-rouge">'static</code> and <code class="highlighter-rouge">Send</code> constraints explanations to <code class="highlighter-rouge">thread::spawn</code></a>.</li>
  <li><a href="https://github.com/neosilky">@neosilky</a> updated <a href="https://github.com/rust-lang/rust/pull/42131">to trait bounds CSS in rustdoc</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/42122">a few entries to the Unstable Book.</a>.</li>
  <li><a href="https://github.com/euclio">@euclio</a> removed <a href="https://github.com/rust-lang/rust/pull/42120">“much” from unicode diagnostic</a>.</li>
  <li><a href="https://github.com/nical">@nical</a> updated <a href="https://github.com/rust-lang/rust/pull/42137">to Rc and Arc documentation to favor the Rc::clone(&amp;ptr) syntax</a>.</li>
  <li><a href="https://github.com/citizen428">@citizen428</a> updated <a href="https://github.com/rust-lang/rust/pull/42236">documentation for indexing/slicing methods</a> and changed <a href="https://github.com/rust-lang/rust/pull/42150">error count messages</a>.</li>
  <li><a href="https://github.com/king6cong">@king6cong</a> reworded <a href="https://github.com/rust-lang/rust/pull/42241">doc</a>.</li>
  <li><a href="https://github.com/charliesome">@charliesome</a> fixed <a href="https://github.com/rust-lang/rust/pull/42216">‘associate type’ typo</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> documented <a href="https://github.com/rust-lang/rust/pull/42159">drop more</a>.</li>
  <li><a href="https://github.com/SamWhited">@SamWhited</a> fixed <a href="https://github.com/rust-lang/rust/pull/42195">broken link to nomicon in Unsize docs</a>.</li>
  <li><a href="https://github.com/projektir">@projektir</a> added <a href="https://github.com/rust-lang/rust/pull/42163">links to option::Option</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/42145">names of items in cross crate reexported modules in rustdoc</a>.</li>
  <li><a href="https://github.com/Wallacoloo">@Wallacoloo</a> mentionned <a href="https://github.com/rust-lang/rust/pull/42151">Vec::into_boxed_slice in documentation of [T]::into_vec</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41559">better error message when == operator is badly used</a>, made <a href="https://github.com/rust-lang/rust/pull/41700">–extend-css stable</a>, added <a href="https://github.com/rust-lang/rust/pull/42198">missing urls for OsStr docs</a> and added <a href="https://github.com/rust-lang/rust/pull/42152">missing links for CStr and CString</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 31st of May 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>