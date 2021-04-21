+++
title = "This Week in Rust Docs 92"
date = "2018-02-11T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-92"
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

<p>The roadmap for 2018 is still in discussion. More information soon!</p>

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
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/47843"><code class="highlighter-rouge">-Zteach</code> documentation</a>, reworded <a href="https://github.com/rust-lang/rust/pull/48138">E0044 and message for <code class="highlighter-rouge">!Send</code> types</a> and detected <a href="https://github.com/rust-lang/rust/pull/47763">possibly non-Rust closure syntax during parse</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> added <a href="https://github.com/rust-lang/rust/pull/47894">foldable impl blocks in rustdoc</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> fixed <a href="https://github.com/rust-lang/rust/pull/47731">how paths are printed by error messages during bootstrap</a>.</li>
  <li><a href="https://github.com/Aaron1011">@Aaron1011</a> generated <a href="https://github.com/rust-lang/rust/pull/47833">documentation for auto-trait impls</a>.</li>
  <li><a href="https://github.com/PramodBisht">@PramodBisht</a> changed <a href="https://github.com/rust-lang/rust/pull/47806">color of struct link from #ff794d to #2dbfb8 for Rust docs</a>.</li>
  <li><a href="https://github.com/matthiaskrgr">@matthiaskrgr</a> fixed <a href="https://github.com/rust-lang/rust/pull/48133">typo: endianess to endianness (this also changes function names!)</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> showed <a href="https://github.com/rust-lang/rust/pull/48033">better warning for trying to cast non-u8 scalar to char</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/47613">filtering options to <code class="highlighter-rouge">rustc_on_unimplemented</code></a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> documented <a href="https://github.com/rust-lang/rust/pull/46962">std::os::raw</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/47496">documentation from doc(include) to analysis data</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/47753">rust book</a>.</li>
  <li><a href="https://github.com/varkor">@varkor</a> documented <a href="https://github.com/rust-lang/rust/pull/47547">the behaviour of infinite iterators on potentially-computable methods</a>, created <a href="https://github.com/rust-lang/rust/pull/47854">a directory for –out-dir if it does not already exist</a> and warned <a href="https://github.com/rust-lang/rust/pull/47203">when rustc output conflicts with existing directories</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> fixed <a href="https://github.com/rust-lang/rust/pull/47959">rustdoc ICE on macros defined within functions</a> and bailed <a href="https://github.com/rust-lang/rust/pull/48064">early for linky things in intra-docs-links</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/47958">shared file handler behavior of File::try_clone</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> declined <a href="https://github.com/rust-lang/rust/pull/47896">to lint technically-unnecessary parens in function or method arguments inside of nested macros</a>, corrected <a href="https://github.com/rust-lang/rust/pull/47922">unused field pattern suggestions</a> and corrected <a href="https://github.com/rust-lang/rust/pull/48028">E0619 span re method call receivers whose type must be known</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/47286">RELEASES.md for 1.24.0</a>.</li>
  <li><a href="https://github.com/matthiaskrgr">@matthiaskrgr</a> fixed <a href="https://github.com/rust-lang/rust/pull/48107">typo: substract -&gt; subtract</a>, fixed <a href="https://github.com/rust-lang/rust/pull/48120">typos in src/{bootstrap,ci,etc,lib{backtrace,core,fmt_macros}}</a> and fixed <a href="https://github.com/rust-lang/rust/pull/48031">typos in config.toml.example</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> hid <a href="https://github.com/rust-lang/rust/pull/48051"><code class="highlighter-rouge">-&gt; ()</code> in cross crate inlined Fn bounds</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> warned <a href="https://github.com/rust-lang/rust/pull/48020">about more ignored bounds in type aliases</a>.</li>
  <li><a href="https://github.com/Badel2">@Badel2</a> documented <a href="https://github.com/rust-lang/rust/pull/48026">that associated constants prevent a trait from being made into an object</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> fixed <a href="https://github.com/rust-lang/rust/pull/48003">info about generic impls in AsMut docs</a>.</li>
  <li><a href="https://github.com/jaystrictor">@jaystrictor</a> removed <a href="https://github.com/rust-lang/rust/pull/47999">‘the this’ in doc comments</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/47761">tests for themes</a>, fixed <a href="https://github.com/rust-lang/rust/pull/47810">themes rendering issues on mobile</a>, fixed <a href="https://github.com/rust-lang/rust/pull/47862">const evaluation ICE in rustdoc</a> and hide <a href="https://github.com/rust-lang/rust/pull/48080">theme button under menu in mobile mode and fix top margin issue …</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 13th of February 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>