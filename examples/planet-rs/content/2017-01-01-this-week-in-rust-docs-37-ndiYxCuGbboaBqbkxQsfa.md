+++
title = "This Week in Rust Docs 37"
date = "2017-01-01T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-37"
+++
<p>Hello and welcome to <em>This Week in Rust Docs</em> and happy new year!</p>

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
  <li><a href="https://github.com/brson">@brson</a> removed <a href="https://github.com/rust-lang/rust/pull/37057">all “consider using an explicit lifetime parameter” suggestions</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/38605">solutions for <code class="highlighter-rouge">fn foo(&amp;foo: Foo)</code></a>, escaped <a href="https://github.com/rust-lang/rust/pull/38244">the deprecated and unstable reason text in rustdoc</a>, disambiguated <a href="https://github.com/rust-lang/rust/pull/38414">Implementors when the type name is not unique in rustdoc</a> and provide <a href="https://github.com/rust-lang/rust/pull/38168">disambiguated syntax for candidates in E0034</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/38581">behavior of <code class="highlighter-rouge">VecDeque::insert</code></a>, made <a href="https://github.com/rust-lang/rust/pull/38457">improvements to ‘include’ macro documentation</a> and clarified <a href="https://github.com/rust-lang/rust/pull/38310">zero-value behavior of <code class="highlighter-rouge">ctlz</code>/<code class="highlighter-rouge">cttz</code> intrinsics</a>.</li>
  <li><a href="https://github.com/bombless">@bombless</a> fixed <a href="https://github.com/rust-lang/rust/pull/38629">doc for <code class="highlighter-rouge">escape_debug</code></a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/rkruppe">@rkruppe</a> replaced <a href="https://github.com/rust-lang/rust/pull/38138">loop {} with abort() for panic in The Book</a>.</li>
  <li><a href="https://github.com/chris-morgan">@chris-morgan</a> fixed <a href="https://github.com/rust-lang/rust/pull/38569">rustdoc highlighting of <code class="highlighter-rouge">&amp;</code> and <code class="highlighter-rouge">*</code></a>.</li>
  <li><a href="https://github.com/linclark">@linclark</a> added <a href="https://github.com/rust-lang/rust/pull/38108">error explanation for E0328</a>.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> update <a href="https://github.com/rust-lang/rust/pull/38216">docs of slice get() and friends</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> documented <a href="https://github.com/rust-lang/rust/pull/38247">the optional extra arguments to assert_eq!() / assert_ne!()</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/38548">missing example for Thread struct</a>, added <a href="https://github.com/rust-lang/rust/pull/38362">Instant doc</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/38517">phrasing of MSYS2 dependencies in README.md</a> and documented <a href="https://github.com/rust-lang/rust/pull/38630">foreign variadic functions in TRPL and the reference.</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/38329">invalid HTML in stability notices in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38671">broken CSS for trait items in rustdoc</a>.</li>
  <li><a href="https://github.com/programble">@programble</a> added <a href="https://github.com/rust-lang/rust/pull/38711">links to methods on all slice iterator struct docs</a>.</li>
  <li><a href="https://github.com/jseyfried">@jseyfried</a> fixed <a href="https://github.com/rust-lang/rust/pull/38537">internal compiler error (ICE) in rustdoc</a>.</li>
  <li><a href="https://github.com/lucis-fluxum">@lucis-fluxum</a> fixed <a href="https://github.com/rust-lang/rust/pull/38693">typo in PartialOrd docs</a>.</li>
  <li><a href="https://github.com/agl">@agl</a> added <a href="https://github.com/rust-lang/rust/pull/38662">“an” before “i32”</a> and add <a href="https://github.com/rust-lang/rust/pull/38659">missing apostrophe.</a>.</li>
  <li><a href="https://github.com/kellerkindt">@kellerkindt</a> and suddenly <a href="https://github.com/rust-lang/rust/pull/38628">a german word :O</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/38491">builder docs</a>, added <a href="https://github.com/rust-lang/rust/pull/38587">missing urls in Arc docs</a>, add <a href="https://github.com/rust-lang/rust/pull/38611">missing urls in AtomicBool docs</a>, added <a href="https://github.com/rust-lang/rust/pull/38635">missing urls for AtomicPtr</a>, added <a href="https://github.com/rust-lang/rust/pull/38649">missing urls for atomic_int macros types</a> and added <a href="https://github.com/rust-lang/rust/pull/38674">missing urls for atomic fn docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 4th of January 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>