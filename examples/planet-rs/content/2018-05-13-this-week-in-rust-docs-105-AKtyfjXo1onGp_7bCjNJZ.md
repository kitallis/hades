+++
title = "This Week in Rust Docs 105"
date = "2018-05-13T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-105"
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

<p>Some improvements on JS size are being worked on. More to come soon!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
  <li><a href="https://github.com/rust-lang-nursery/reference/projects/1">Document all features in the Rust reference</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/labels/T-doc">T-doc</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/sgrif">@sgrif</a> removed <a href="https://github.com/rust-lang/rust/pull/48407">the “leak check” in favor of “universes”</a>.</li>
  <li><a href="https://github.com/ecstatic-morse">@ecstatic-morse</a> rewrote <a href="https://github.com/rust-lang/rust/pull/49767">docs for <code class="highlighter-rouge">std::ptr</code></a>.</li>
  <li><a href="https://github.com/da-x">@da-x</a> included <a href="https://github.com/rust-lang/rust/pull/49898">scope names in diagnostics</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> clarified <a href="https://github.com/rust-lang/rust/pull/49743">the difference between get_mut and into_mut for OccupiedEntry</a>.</li>
  <li><a href="https://github.com/mulkieran">@mulkieran</a> made <a href="https://github.com/rust-lang/rust/pull/50255">some documentation fixes for the Write trait</a>.</li>
  <li><a href="https://github.com/fkjogu">@fkjogu</a> documented <a href="https://github.com/rust-lang/rust/pull/50342">round-off error in <code class="highlighter-rouge">.mod_euc()</code>-method</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> added <a href="https://github.com/rust-lang/rust/pull/50143">deprecation lint for duplicated <code class="highlighter-rouge">macro_export</code>s</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> deprecated <a href="https://github.com/rust-lang/rust/pull/50669"><code class="highlighter-rouge">#![doc(passes, plugins, no_default_passes)]</code> in rustdoc</a> and replaced <a href="https://github.com/rust-lang/rust/pull/50541">most (e)println! statements with structured warnings/errors in rustdoc</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> added <a href="https://github.com/rust-lang/rust/pull/50682">lint for multiple associated types</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/50691">support for pub(restricted) in rustdoc</a>.</li>
  <li><a href="https://github.com/adevore">@adevore</a> added <a href="https://github.com/rust-lang/rust/pull/50624">example writing a &amp;str for fs::write</a>.</li>
  <li><a href="https://github.com/sanxiyn">@sanxiyn</a> updated <a href="https://github.com/rust-lang/rust/pull/50594">the man page with additional –print options</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilized <a href="https://github.com/rust-lang/rust/pull/49546">short error format</a>, made <a href="https://github.com/rust-lang/rust/pull/50259">some rustdoc improvements</a>, fixed <a href="https://github.com/rust-lang/rust/pull/50617">extern prelude failure in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/50632">minification process</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/ibabushkin">@ibabushkin</a> refactored <a href="https://github.com/rust-lang/rust/pull/49711">auto trait handling in librustdoc to be accessible from librustc</a>.</li>
  <li><a href="https://github.com/rizakrko">@rizakrko</a> added <a href="https://github.com/rust-lang/rust/pull/50161">missing implementation hint</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> mentionned <a href="https://github.com/rust-lang/rust/pull/49988"><code class="highlighter-rouge">Result&lt;!, E&gt;</code> in never docs</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> removed <a href="https://github.com/rust-lang/rust/pull/50476">crazy suggestion for unreachable braced pub-use</a>.</li>
  <li><a href="https://github.com/leodasvacas">@leodasvacas</a> improve <a href="https://github.com/rust-lang/rust/pull/50536">error reporting in Copy derive</a>.</li>
  <li><a href="https://github.com/Screwtapello">@Screwtapello</a> updated <a href="https://github.com/rust-lang/rust/pull/50602">canonicalize docs</a>.</li>
  <li><a href="https://github.com/glandium">@glandium</a> restored <a href="https://github.com/rust-lang/rust/pull/50591">RawVec::reserve* documentation</a> and cleaned up <a href="https://github.com/rust-lang/rust/pull/50527">a <code class="highlighter-rouge">use</code> in a raw_vec test</a>.</li>
  <li><a href="https://github.com/ExpHP">@ExpHP</a> moved <a href="https://github.com/rust-lang/rust/pull/50588">“See also” disambiguation links for primitive types to top</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/50572">in the docs that <code class="highlighter-rouge">mul_add</code> is not always faster</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> added <a href="https://github.com/rust-lang/rust/pull/50511">some explanations for #[must_use]</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/49523">RELEASES.md for 1.26.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> prevented <a href="https://github.com/rust-lang/rust/pull/50305">infinite recursion of modules in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/50432">rustdoc pathes search</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 15th of May 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>