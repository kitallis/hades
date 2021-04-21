+++
title = "This Week in Rust Docs 71"
date = "2017-09-03T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-71"
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
  <li><a href="https://github.com/qnighy">@qnighy</a> added <a href="https://github.com/rust-lang/rust/pull/43426">hints when intercrate ambiguity causes overlap</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/43849">new “Implementations on Foreign Types” section to traits in rustdoc</a>, hid <a href="https://github.com/rust-lang/rust/pull/44026">internal types/traits from std docs via new #[doc(masked)] attribute</a> and expanded <a href="https://github.com/rust-lang/rust/pull/44194">on using rustup custom toolchains in CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> extended <a href="https://github.com/rust-lang/rust/pull/44079">E0623 for LateBound and EarlyBound Regions</a> and added <a href="https://github.com/rust-lang/rust/pull/44124">E0623 for return types - both parameters are anonymous</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> added <a href="https://github.com/rust-lang/rust/pull/44274">tests rustdoc</a>.</li>
  <li><a href="https://github.com/MarkMcCaskey">@MarkMcCaskey</a> updated <a href="https://github.com/rust-lang/rust/pull/44206">unimplemented! docs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilised <a href="https://github.com/rust-lang/rust/pull/43949">compile_fail</a>, added <a href="https://github.com/rust-lang/rust/pull/43870">deref suggestion</a>, fixed <a href="https://github.com/rust-lang/rust/pull/44254">rendering of const keyword for functions</a> and added <a href="https://github.com/rust-lang/rust/pull/44165">display of cfg in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/ishitatsuyuki">@ishitatsuyuki</a> made <a href="https://github.com/rust-lang/rust/pull/42588">unused-extern-crate warn-by-default</a>.</li>
  <li><a href="https://github.com/llogiq">@llogiq</a> added <a href="https://github.com/rust-lang/rust/pull/44104">a lowercase suggestion to unknown_lints</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/43886">clippy as a submodule</a>, `](https://github.com/rust-lang/rust/pull/44059).</li>
  <li><a href="https://github.com/mystor">@mystor</a> removed <a href="https://github.com/rust-lang/rust/pull/43918">highlight of # which does not start an attribute in rustdoc</a>.</li>
  <li><a href="https://github.com/nrc">@nrc</a> improved <a href="https://github.com/rust-lang/rust/pull/44238">the Pulldown/hoedown warnings</a>.</li>
  <li><a href="https://github.com/mattico">@mattico</a> fixed <a href="https://github.com/rust-lang/rust/pull/44172">link in unstable book entry for Generators</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expandws <a href="https://github.com/rust-lang/rust/pull/44209">docs of multi-address behavior of some UDP/TCP APIs</a>, fixed <a href="https://github.com/rust-lang/rust/pull/44205">typo in doc <code class="highlighter-rouge">ToSocketAddrs</code> example</a> and rewrote <a href="https://github.com/rust-lang/rust/pull/44117"><code class="highlighter-rouge">std::net::ToSocketAddrs</code> doc examples</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> fixed <a href="https://github.com/rust-lang/rust/pull/44231">release notes on associated constants</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> fixed <a href="https://github.com/rust-lang/rust/pull/44230">typo in 1.20.0 release notes</a>.</li>
  <li><a href="https://github.com/daboross">@daboross</a> clarified <a href="https://github.com/rust-lang/rust/pull/44114">that VecDeque::swap can panic</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, updated <a href="https://github.com/rust-lang/rust/pull/44256">html-diff-rs version</a>, fixed <a href="https://github.com/rust-lang/rust/pull/44192">invalid display of enum sub-fields docs</a> and fixed <a href="https://github.com/rust-lang/rust/pull/44135">invalid linker position</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 6th of September 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>