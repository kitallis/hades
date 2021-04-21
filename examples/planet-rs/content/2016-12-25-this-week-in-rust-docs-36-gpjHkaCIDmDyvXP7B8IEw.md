+++
title = "This Week in Rust Docs 36"
date = "2016-12-25T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-36"
+++
<p>Hello and welcome to <em>This Week in Rust Docs</em>!</p>

<p><em>This Week in Rust Docs</em> is openly developed <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs">on GitHub</a>.
If you find any errors in this week’s issue, <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs/pulls">please submit a PR</a>.</p>

<p>And of course, don’t forget to look at the docs:</p>

<ul>
  <li><a href="https://doc.rust-lang.org/">Stable</a></li>
  <li><a href="http://doc.rust-lang.org/beta/">Beta</a></li>
  <li><a href="http://doc.rust-lang.org/nightly/">Nightly</a></li>
</ul>

<p>This week’s edition was edited by <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>The way rustdoc is creating urls is problematic for the moment. A good summary of this issue can be found <a href="https://github.com/rust-lang/rust/issues/36417">here</a>. A few members of the Rust Doc team are preparing an RFC in order to improve this. If you want to get involved, feel free to speak about it with <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a> (imperio on IRC).</p>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/38581">behavior of <code class="highlighter-rouge">VecDeque::insert</code></a>, clarified <a href="https://github.com/rust-lang/rust/pull/38517">phrasing of MSYS2 dependencies in README.md</a>, made <a href="https://github.com/rust-lang/rust/pull/38457">improvements to ‘include’ macro documentation</a> and clarified <a href="https://github.com/rust-lang/rust/pull/38310">zero-value behavior of <code class="highlighter-rouge">ctlz</code>/<code class="highlighter-rouge">cttz</code> intrinsics</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/38491">builder docs</a>, added <a href="https://github.com/rust-lang/rust/pull/38587">missing urls in Arc docs</a>, added <a href="https://github.com/rust-lang/rust/pull/38548">missing example for Thread struct</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>, improved <a href="https://github.com/rust-lang/rust/pull/38362">instant doc</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> escaped <a href="https://github.com/rust-lang/rust/pull/38244">the deprecated and unstable reason text in rustdoc</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/rkruppe">@rkruppe</a> used <a href="https://github.com/rust-lang/rust/pull/38138">abort() over loop {} for panic in The Book</a>.</li>
  <li><a href="https://github.com/chris-morgan">@chris-morgan</a> fixed <a href="https://github.com/rust-lang/rust/pull/38569">rustdoc highlighting of <code class="highlighter-rouge">&amp;</code> and <code class="highlighter-rouge">*</code></a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/38329">invalid HTML in stability notices in rustdoc</a>.</li>
  <li><a href="https://github.com/linclark">@linclark</a> added <a href="https://github.com/rust-lang/rust/pull/38108">error explanation for E0328.</a>.</li>
  <li><a href="https://github.com/chriskrycho">@chriskrycho</a> documented <a href="https://github.com/rust-lang/rust/pull/37928">RFC 1623: static lifetime elision.</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> documented <a href="https://github.com/rust-lang/rust/pull/38247">the optional extra arguments to assert_eq!() / assert_ne!()</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/38443">the API examples for <code class="highlighter-rouge">std::fs::File</code></a>, documented <a href="https://github.com/rust-lang/rust/pull/38397">platform-specific differences for <code class="highlighter-rouge">std::process::exit</code></a>, improved <a href="https://github.com/rust-lang/rust/pull/38334">documentation for <code class="highlighter-rouge">core::hash::BuildHasherDefault</code></a>, improved <a href="https://github.com/rust-lang/rust/pull/38208"><code class="highlighter-rouge">BTreeSet</code> documentation</a> and implement <a href="https://github.com/rust-lang/rust/pull/38006"><code class="highlighter-rouge">fmt::Debug</code> for all structures in libstd.</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> wrote <a href="https://github.com/rust-lang/rust/pull/38427">the 1.14 release notes</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/38330">short summaries in search results in rustdoc</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed out <a href="https://github.com/rust-lang/rust/pull/38150">the known type when field doesn’t satisfy bound</a> and explained <a href="https://github.com/rust-lang/rust/pull/38505">why/when <code class="highlighter-rouge">.lines()</code> returns an error</a>.</li>
  <li><a href="https://github.com/matklad">@matklad</a> advertised <a href="https://github.com/rust-lang/rust/pull/38297">Vec in LinkedList docs</a>.</li>
  <li><a href="https://github.com/sourcefrog">@sourcefrog</a> explained <a href="https://github.com/rust-lang/rust/pull/38158">the meaning of Result iters and link to factory functions</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/38395">a formatting nit in rustdoc</a>.</li>
  <li><a href="https://github.com/wezm">@wezm</a> simplified <a href="https://github.com/rust-lang/rust/pull/38013">notes on testing and concurrency</a>.</li>
  <li><a href="https://github.com/jonhoo">@jonhoo</a> expanded <a href="https://github.com/rust-lang/rust/pull/38315">E0309 explanation with motivating example</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/38433">thread docs</a>, improved <a href="https://github.com/rust-lang/rust/pull/38236">unix socket doc</a>, improved <a href="https://github.com/rust-lang/rust/pull/38346">duration doc</a>, added <a href="https://github.com/rust-lang/rust/pull/38572">JoinHandle missing examples</a>, added <a href="https://github.com/rust-lang/rust/pull/38513">missing examples in some thread functions</a> and added <a href="https://github.com/rust-lang/rust/pull/38099">cast suggestions</a>.</li>
  <li><a href="https://github.com/DirkyJerky">@DirkyJerky</a> created <a href="https://github.com/rust-lang/rust/pull/38554">hyperlink to correct documentation</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> properly calculated <a href="https://github.com/rust-lang/rust/pull/38497">line length for where clauses</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> removed <a href="https://github.com/rust-lang/rust/pull/38480">@import normalize.css</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> made <a href="https://github.com/rust-lang/rust/pull/38432">a minor fix in the merge_sort comments</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 28th of December 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>