+++
title = "This Week in Rust Docs 106"
date = "2018-05-21T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-106"
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
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> clarified <a href="https://github.com/rust-lang/rust/pull/49743">the difference between get_mut and into_mut for OccupiedEntry</a>.</li>
  <li><a href="https://github.com/mulkieran">@mulkieran</a> made <a href="https://github.com/rust-lang/rust/pull/50255">some documentation fixes for the Write trait</a>.</li>
  <li><a href="https://github.com/fkjogu">@fkjogu</a> documented <a href="https://github.com/rust-lang/rust/pull/50342">round-off error in <code class="highlighter-rouge">.mod_euc()</code>-method</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> added <a href="https://github.com/rust-lang/rust/pull/50143">deprecation lint for duplicated <code class="highlighter-rouge">macro_export</code>s</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> added <a href="https://github.com/rust-lang/rust/pull/50682">lint for multiple associated types</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> provided <a href="https://github.com/rust-lang/rust/pull/50919">more context for what the {f32,f64}::EPSILON values represent</a>.</li>
  <li><a href="https://github.com/davidtwco">@davidtwco</a> added <a href="https://github.com/rust-lang/rust/pull/50892">rustdoc documentation to compiler docs</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/50852">doc comment to hiding portions of code example</a>.</li>
  <li><a href="https://github.com/simartin">@simartin</a> improved <a href="https://github.com/rust-lang/rust/pull/50914">error diagnostic with missing commas after struct fields</a>.</li>
  <li><a href="https://github.com/d-e-s-o">@d-e-s-o</a> fixed <a href="https://github.com/rust-lang/rust/pull/50913">typo in cell.rs</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> used <a href="https://github.com/rust-lang/rust/pull/50875">“short form” doc(cfg) printing even when combined with other conditionals in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilized <a href="https://github.com/rust-lang/rust/pull/49546">short error format</a>, fixed <a href="https://github.com/rust-lang/rust/pull/50617">extern prelude failure in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/50846">E0665</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/ecstatic-morse">@ecstatic-morse</a> rewrote <a href="https://github.com/rust-lang/rust/pull/49767">docs for <code class="highlighter-rouge">std::ptr</code></a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> deprecated <a href="https://github.com/rust-lang/rust/pull/50669"><code class="highlighter-rouge">#![doc(passes, plugins, no_default_passes)]</code> in rustdoc</a> and replaced <a href="https://github.com/rust-lang/rust/pull/50541">most (e)println! statements with structured warnings/errors in rustdoc</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/50691">support for pub(restricted) in rustdoc</a>.</li>
  <li><a href="https://github.com/adevore">@adevore</a> added <a href="https://github.com/rust-lang/rust/pull/50624">example writing a &amp;str for fs::write</a>.</li>
  <li><a href="https://github.com/sanxiyn">@sanxiyn</a> updated <a href="https://github.com/rust-lang/rust/pull/50594">the man page with additional –print options</a>.</li>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> fixed <a href="https://github.com/rust-lang/rust/pull/50898">UnsafeCell doc typos and minor flow improvements</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> fixed <a href="https://github.com/rust-lang/rust/pull/50719">incorrect statement about return value for Iterator::zip</a>.</li>
  <li><a href="https://github.com/robinkrahl">@robinkrahl</a> reordered <a href="https://github.com/rust-lang/rust/pull/50858">description for snippets in rustdoc documentation</a>.</li>
  <li><a href="https://github.com/shamiao">@shamiao</a> fixed <a href="https://github.com/rust-lang/rust/pull/50797">a typo in signed-integer::from_str_radix()</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> fixed <a href="https://github.com/rust-lang/rust/pull/50728">rustdoc panic with <code class="highlighter-rouge">impl Trait</code> in type parameters</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> made <a href="https://github.com/rust-lang/rust/pull/50259">some rustdoc improvements</a>, added <a href="https://github.com/rust-lang/rust/pull/50632">minification process</a>, added <a href="https://github.com/rust-lang/rust/pull/50533">auto-impl for primitive type</a> and added <a href="https://github.com/rust-lang/rust/pull/50752">missing error codes in libsyntax-ext asm</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 22nd of May 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>