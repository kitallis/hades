+++
title = "This Week in Rust Docs 35"
date = "2016-12-18T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-35"
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
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/38443">the API examples for <code class="highlighter-rouge">std::fs::File</code></a>, documented <a href="https://github.com/rust-lang/rust/pull/38397">platform-specific differences for <code class="highlighter-rouge">std::process::exit</code></a>, improved <a href="https://github.com/rust-lang/rust/pull/38334">documentation for <code class="highlighter-rouge">core::hash::BuildHasherDefault</code></a> and improved <a href="https://github.com/rust-lang/rust/pull/38208"><code class="highlighter-rouge">BTreeSet</code> documentation</a>.</li>
  <li><a href="https://github.com/bluecereal">@bluecereal</a> updated <a href="https://github.com/rust-lang/rust/pull/38436">if-let.md</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> wrote <a href="https://github.com/rust-lang/rust/pull/38427">the 1.14 release notes</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/38329">invalid HTML in stability notices in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38330">short summaries in search results in rustdoc</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> used <a href="https://github.com/rust-lang/rust/pull/38328">struct name as span instead of entire block</a>, disambiguated <a href="https://github.com/rust-lang/rust/pull/38414">Implementors when the type name is not unique in rustdoc</a>, showed <a href="https://github.com/rust-lang/rust/pull/37493">span for trait that doesn’t implement Copy</a>, escaped <a href="https://github.com/rust-lang/rust/pull/38244">the deprecated and unstable reason text in rustdoc</a>, pointed out <a href="https://github.com/rust-lang/rust/pull/38150">the known type when field doesn’t satisfy bound</a> and provided <a href="https://github.com/rust-lang/rust/pull/38168">disambiguated syntax for candidates in E0034</a>.</li>
  <li><a href="https://github.com/matklad">@matklad</a> advertised <a href="https://github.com/rust-lang/rust/pull/38297">Vec in LinkedList docs</a>.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> updated <a href="https://github.com/rust-lang/rust/pull/38216">docs of slice get() and friends</a>.</li>
  <li><a href="https://github.com/sourcefrog">@sourcefrog</a> explained <a href="https://github.com/rust-lang/rust/pull/38158">the meaning of Result iters and link to factory functions</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/38395">a formatting nit in rustdoc</a>.</li>
  <li><a href="https://github.com/wezm">@wezm</a> simplified <a href="https://github.com/rust-lang/rust/pull/38013">notes on testing and concurrency</a>.</li>
  <li><a href="https://github.com/jonhoo">@jonhoo</a> expanded <a href="https://github.com/rust-lang/rust/pull/38315">E0309 explanation with motivating example</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> documented <a href="https://github.com/rust-lang/rust/pull/38247">the optional extra arguments to assert_eq!() / assert_ne!()</a>.</li>
  <li><a href="https://github.com/rkruppe">@rkruppe</a> used <a href="https://github.com/rust-lang/rust/pull/38138">abort() over loop {} for panic in The Book</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/38433">thread docs</a>, improved <a href="https://github.com/rust-lang/rust/pull/38236">unix socket doc</a>, improved <a href="https://github.com/rust-lang/rust/pull/38346">duration doc</a> and improved <a href="https://github.com/rust-lang/rust/pull/38362">instant doc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/KiChjang">@KiChjang</a> displayed <a href="https://github.com/rust-lang/rust/pull/38057">better error messages for E0282</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/38264">broken src links from reexported items from macros in rustdoc</a>.</li>
  <li><a href="https://github.com/Cobrand">@Cobrand</a> improved <a href="https://github.com/rust-lang/rust/pull/37941">and fixed mpsc documentation</a>.</li>
  <li><a href="https://github.com/sourcefrog">@sourcefrog</a> avoided <a href="https://github.com/rust-lang/rust/pull/38164">using locally installed Source Code Pro font in rustdoc</a>.</li>
  <li><a href="https://github.com/michael-zapata">@michael-zapata</a> harmonised <a href="https://github.com/rust-lang/rust/pull/38179">rustdoc error messages</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/38122">The Book for rustup</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/38067">more examples to UpdSocket</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 21st of December 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>