+++
title = "This Week in Rust Docs 74"
date = "2017-09-24T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-74"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> made <a href="https://github.com/rust-lang/rust/pull/44613">rustdoc optimizations</a> and included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> added <a href="https://github.com/rust-lang/rust/pull/44124">E0623 for return types - both parameters are anonymous</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a>.</li>
  <li><a href="https://github.com/laumann">@laumann</a> added <a href="https://github.com/rust-lang/rust/pull/44297">suggestions for misspelled method names</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/44103">comparison operators to must-use lint (under <code class="highlighter-rouge">fn_must_use</code> feature)</a> and prevented <a href="https://github.com/rust-lang/rust/pull/44713">rustdoc to get confused by text “fn main” in a line comment</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/44625">how ChildStd{in,out,err} FDs are closed</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/44481">RELEASES.md for 1.21.0</a>.</li>
  <li><a href="https://github.com/thombles">@thombles</a> improved <a href="https://github.com/rust-lang/rust/pull/44786">diagnostics when attempting to match tuple enum variant with struct pattern</a>.</li>
  <li><a href="https://github.com/napen123">@napen123</a> added <a href="https://github.com/rust-lang/rust/pull/44794">doc example to HashMap::hasher</a>.</li>
  <li><a href="https://github.com/lucasem">@lucasem</a> made <a href="https://github.com/rust-lang/rust/pull/44797">docs improvement in std::sync::{PoisonError, TryLockError}</a>.</li>
  <li><a href="https://github.com/tirr-c">@tirr-c</a> made <a href="https://github.com/rust-lang/rust/pull/44735">a friendlier error message for closure argument type mismatch</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed [at parameter type on E0301](https://github.com/rust-lang/rust/pull/44782].</li>
  <li><a href="https://github.com/budziq">@budziq</a> corrected <a href="https://github.com/rust-lang/rust/pull/44664">the CONTRIBUTING.md “External Dependencies” section</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a> and fixed <a href="https://github.com/rust-lang/rust/pull/44789">warning position in rustdoc code blocks</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> hid <a href="https://github.com/rust-lang/rust/pull/44026">internal types/traits from std docs via new #[doc(masked)] attribute</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> extended <a href="https://github.com/rust-lang/rust/pull/44549">E0623 for earlybound and latebound for structs</a>.</li>
  <li><a href="https://github.com/bluss">@bluss</a> documented <a href="https://github.com/rust-lang/rust/pull/44651">thread builder panics for nul bytes in thread names</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> removed <a href="https://github.com/rust-lang/rust/pull/44215">suggestion of placing <code class="highlighter-rouge">use</code> statements into expanded code</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> expanded <a href="https://github.com/rust-lang/rust/pull/44648">size_of docs</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> fixed <a href="https://github.com/rust-lang/rust/pull/44622">incorrect <code class="highlighter-rouge">into_inner</code> link in docs</a>.</li>
  <li><a href="https://github.com/nikomatsakis">@nikomatsakis</a> reworked <a href="https://github.com/rust-lang/rust/pull/44505">the README.md for rustc and add other readmes</a>.</li>
  <li><a href="https://github.com/lucasem">@lucasem</a> improved <a href="https://github.com/rust-lang/rust/pull/44778">std::sync::RwLock docs</a>.</li>
  <li><a href="https://github.com/spastorino">@spastorino</a> linked <a href="https://github.com/rust-lang/rust/pull/44776">to Rust forge from CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> improved <a href="https://github.com/rust-lang/rust/pull/44759">english in create_dir_all docs</a>.</li>
  <li><a href="https://github.com/mattico">@mattico</a> fixed <a href="https://github.com/rust-lang/rust/pull/44726">librustc/README.md diagram</a>.</li>
  <li><a href="https://github.com/oconnor663">@oconnor663</a> fixed <a href="https://github.com/rust-lang/rust/pull/44712">an incorrect assertion in the doc example for <code class="highlighter-rouge">std::io::copy</code></a>.</li>
  <li><a href="https://github.com/mssun">@mssun</a> fixed <a href="https://github.com/rust-lang/rust/pull/44693">a typo in rustc help menu</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/43870">deref suggestion</a>, updated <a href="https://github.com/rust-lang/rust/pull/44397">codeblock color</a>, removed <a href="https://github.com/rust-lang/rust/pull/44350">small id false positive in rustdoc html diff</a>, added <a href="https://github.com/rust-lang/rust/pull/44554">pub visibility for methods as well</a>, added <a href="https://github.com/rust-lang/rust/pull/44773">missing links for Arc</a>, added <a href="https://github.com/rust-lang/rust/pull/44703">some missing links in io docs</a>, fixed <a href="https://github.com/rust-lang/rust/pull/44671">run button</a> and added <a href="https://github.com/rust-lang/rust/pull/44661">more links and put the link character to the left</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 27th of September 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>