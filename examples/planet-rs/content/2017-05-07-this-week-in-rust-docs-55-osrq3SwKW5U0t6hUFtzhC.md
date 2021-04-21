+++
title = "This Week in Rust Docs 55"
date = "2017-05-07T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-55"
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

<p>The jquery dependency <a href="https://github.com/rust-lang/rust/pull/41307">is being removed</a> from the rustdoc javascript. When navigating in the docs, please check if everything’s working as expected!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/labels/T-doc">T-doc</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/40857">at fields that make the type recursive</a>, emitted <a href="https://github.com/rust-lang/rust/pull/41434">diagnostic when using <code class="highlighter-rouge">const</code> storing <code class="highlighter-rouge">Fn</code> as pattern</a>, made <a href="https://github.com/rust-lang/rust/pull/41489">unsatisfied trait bounds note multiline</a>, used <a href="https://github.com/rust-lang/rust/pull/41520">diagnostics for trace_macro instead of println</a> and removed <a href="https://github.com/rust-lang/rust/pull/41760">duplicated errors for closure type mismatch</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/icefoxen">@icefoxen</a> made <a href="https://github.com/rust-lang/rust/pull/40719">a tiny update to rustdoc CSS</a>.</li>
  <li><a href="https://github.com/abonander">@abonander</a> documented <a href="https://github.com/rust-lang/rust/pull/41476">the <code class="highlighter-rouge">proc_macro</code> feature in the Unstable Book</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/41612">generic example of std::ops::Add in doc comments</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/41548">release notes for 1.17</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> improved <a href="https://github.com/rust-lang/rust/pull/41536">docs on <code class="highlighter-rouge">Arc&lt;T&gt;</code> and Send/Sync</a> and added <a href="https://github.com/rust-lang/rust/pull/41531">more ways to create a PathBuf to docs</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> allowed <a href="https://github.com/rust-lang/rust/pull/41785"># to appear in rustdoc code output.</a> and made a <a href="https://github.com/rust-lang/rust/pull/41791">minor cleanup of UX guidelines.</a>.</li>
  <li><a href="https://github.com/Idolf">@Idolf</a> supported <a href="https://github.com/rust-lang/rust/pull/41747">#![deny(missing_docs)] together with #[proc_macro_derive]</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> reduced <a href="https://github.com/rust-lang/rust/pull/41384">HTML output in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/41559">better error message when == operator is badly used</a>, added <a href="https://github.com/rust-lang/rust/pull/41772">help message if a FnOnce is moved</a> and made <a href="https://github.com/rust-lang/rust/pull/41700">–extend-css stable</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> cleaned <a href="https://github.com/rust-lang/rust/pull/41488">up callable type mismatch errors</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> updated <a href="https://github.com/rust-lang/rust/pull/41217">docs of ‘fence’</a>.</li>
  <li><a href="https://github.com/alexeyzab">@alexeyzab</a> fixed <a href="https://github.com/rust-lang/rust/pull/41547">error message for mismatched types</a>.</li>
  <li><a href="https://github.com/z1mvader">@z1mvader</a> rewrote <a href="https://github.com/rust-lang/rust/pull/41543">the thread struct docs</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/41784">links between <code class="highlighter-rouge">slice::{copy,clone}_from_slice</code> in docs</a>, fixed <a href="https://github.com/rust-lang/rust/pull/41688">incorrect hex value in doc comment example</a>, simplified <a href="https://github.com/rust-lang/rust/pull/41749">types in <code class="highlighter-rouge">std::option</code> doc comment example</a> and made <a href="https://github.com/rust-lang/rust/pull/41720">improvements to <code class="highlighter-rouge">std::time::Duration</code> doc examples</a>.</li>
  <li><a href="https://github.com/acdenisSK">@acdenisSK</a> fixed <a href="https://github.com/rust-lang/rust/pull/41786">“an” usage</a>.</li>
  <li><a href="https://github.com/rap2hpoutre">@rap2hpoutre</a> added <a href="https://github.com/rust-lang/rust/pull/41768">an example to std::thread::Result type</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> updated <a href="https://github.com/rust-lang/rust/pull/41721">ChildStdin/ChildStdout docs to be clearer</a>.</li>
  <li><a href="https://github.com/bholley">@bholley</a> documented <a href="https://github.com/rust-lang/rust/pull/41730">the reasoning for the Acquire/Release handshake when dropping Arcs</a>.</li>
  <li><a href="https://github.com/cuviper">@cuviper</a> fixed <a href="https://github.com/rust-lang/rust/pull/41613">links in RELEASES.md for 1.10.0 through 1.12.0</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> removed <a href="https://github.com/rust-lang/rust/pull/41705">ANTLR grammar</a> and unignored <a href="https://github.com/rust-lang/rust/pull/41629">tests which work fine now</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> minimized <a href="https://github.com/rust-lang/rust/pull/40851">single span suggestions into a label</a>.</li>
  <li><a href="https://github.com/hsivonen">@hsivonen</a> explained <a href="https://github.com/rust-lang/rust/pull/41602">why zero-length slices require a non-null pointer</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/41307">jquery dependency</a> and add <a href="https://github.com/rust-lang/rust/pull/41678">option to display warnings in rustdoc</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 10th of May 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>