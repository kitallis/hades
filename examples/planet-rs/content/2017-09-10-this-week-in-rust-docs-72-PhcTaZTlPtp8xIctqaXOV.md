+++
title = "This Week in Rust Docs 72"
date = "2017-09-10T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-72"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> hid <a href="https://github.com/rust-lang/rust/pull/44026">internal types/traits from std docs via new #[doc(masked)] attribute</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> extended <a href="https://github.com/rust-lang/rust/pull/44079">E0623 for LateBound and EarlyBound Regions</a> and added <a href="https://github.com/rust-lang/rust/pull/44124">E0623 for return types - both parameters are anonymous</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a> and updated <a href="https://github.com/rust-lang/rust/pull/44430">mdbook</a>.</li>
  <li><a href="https://github.com/tommyip">@tommyip</a> added <a href="https://github.com/rust-lang/rust/pull/44453">doc example to String::as_mut_str</a> and added <a href="https://github.com/rust-lang/rust/pull/44449">doc example to String::as_str</a>.</li>
  <li><a href="https://github.com/smt923">@smt923</a> added <a href="https://github.com/rust-lang/rust/pull/44472">short doc examples for str::from_utf8_mut</a>.</li>
  <li><a href="https://github.com/toidiu">@toidiu</a> updated <a href="https://github.com/rust-lang/rust/pull/44467">documentation to demonstrate mutability</a>.</li>
  <li><a href="https://github.com/napen123">@napen123</a> added <a href="https://github.com/rust-lang/rust/pull/44457">doc examples for str::as_bytes_mut</a>.</li>
  <li><a href="https://github.com/frehberg">@frehberg</a> extended <a href="https://github.com/rust-lang/rust/pull/44378">UdpSocket API doc</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> moved <a href="https://github.com/rust-lang/rust/pull/44413">the man directory to a subdirectory</a>.</li>
  <li><a href="https://github.com/joshlf">@joshlf</a> documented <a href="https://github.com/rust-lang/rust/pull/44396">std::thread::LocalKey limitation with initializers</a>.</li>
  <li><a href="https://github.com/laumann">@laumann</a> added <a href="https://github.com/rust-lang/rust/pull/44297">suggestions for misspelled method names</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilised <a href="https://github.com/rust-lang/rust/pull/43949">compile_fail</a>, added <a href="https://github.com/rust-lang/rust/pull/43870">deref suggestion</a>, fixed <a href="https://github.com/rust-lang/rust/pull/44254">rendering of const keyword for functions</a>, added <a href="https://github.com/rust-lang/rust/pull/44165">display of cfg in rustdoc</a>, codeblock <a href="https://github.com/rust-lang/rust/pull/44397">color</a>, reduced <a href="https://github.com/rust-lang/rust/pull/44347">false positives number in rustdoc html diff</a> and removed <a href="https://github.com/rust-lang/rust/pull/44350">small id false positive in rustdoc html diff</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/qnighy">@qnighy</a> added <a href="https://github.com/rust-lang/rust/pull/43426">hints when intercrate ambiguity causes overlap</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/43849">new “Implementations on Foreign Types” section to traits in rustdoc</a> and expanded <a href="https://github.com/rust-lang/rust/pull/44194">on using rustup custom toolchains in CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> added <a href="https://github.com/rust-lang/rust/pull/44274">rustdoc tests</a>.</li>
  <li><a href="https://github.com/MarkMcCaskey">@MarkMcCaskey</a> updated <a href="https://github.com/rust-lang/rust/pull/44206">unimplemented! docs</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> removed <a href="https://github.com/rust-lang/rust/pull/44268">invalid doctest from bootstrap.py</a>.</li>
  <li><a href="https://github.com/WiSaGaN">@WiSaGaN</a> fixed <a href="https://github.com/rust-lang/rust/pull/44330">link typo in 1.20.0 release notes</a>.</li>
  <li><a href="https://github.com/jakllsch">@jakllsch</a> included <a href="https://github.com/rust-lang/rust/pull/44321">docs in extended distribution only if enabled</a>.</li>
  <li><a href="https://github.com/lu-zero">@lu-zero</a> removed <a href="https://github.com/rust-lang/rust/pull/44328">the incorrect documentation for from_str</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> removed <a href="https://github.com/rust-lang/rust/pull/44313">dead test functions from rustbook</a>.</li>
  <li><a href="https://github.com/kallisti5">@kallisti5</a> gave <a href="https://github.com/rust-lang/rust/pull/44315">an example to get UNIX_EPOCH in seconds in <code class="highlighter-rouge">std::time</code></a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 13th of September 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>