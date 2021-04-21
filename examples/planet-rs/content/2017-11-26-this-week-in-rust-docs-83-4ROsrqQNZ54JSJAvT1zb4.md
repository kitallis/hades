+++
title = "This Week in Rust Docs 83"
date = "2017-11-26T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-83"
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
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45776">code on diagnostics when underlined</a>, accounted <a href="https://github.com/rust-lang/rust/pull/45997">for missing keyword in fn/struct definition</a>, displayed <a href="https://github.com/rust-lang/rust/pull/45953"><code class="highlighter-rouge">\t</code> in diagnostics code as four spaces</a>, highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, suggest3e <a href="https://github.com/rust-lang/rust/pull/46249">using slice when encountering <code class="highlighter-rouge">let x = ""[..];</code></a> and used <a href="https://github.com/rust-lang/rust/pull/46256">suggestions instead of notes ref mismatches</a>.</li>
  <li><a href="https://github.com/JRegimbal">@JRegimbal</a> changed <a href="https://github.com/rust-lang/rust/pull/45898">“Types/modules” title of search tab to be more accurate</a>.</li>
  <li><a href="https://github.com/colinmarsh19">@colinmarsh19</a> added <a href="https://github.com/rust-lang/rust/pull/46258">a note “please remove this semicolon”</a>.</li>
  <li><a href="https://github.com/canndrew">@canndrew</a> added <a href="https://github.com/rust-lang/rust/pull/46232">docs for never primitive</a>.</li>
  <li><a href="https://github.com/ExpHP">@ExpHP</a> made <a href="https://github.com/rust-lang/rust/pull/46260">doc stubs for builtin macros reflect existing support for trailing commas</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> clarified <a href="https://github.com/rust-lang/rust/pull/46136">what <code class="highlighter-rouge">-D warnings</code> or <code class="highlighter-rouge">-F warnings</code> does</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/46219">documentation for slice swap/copy/clone operations</a>.</li>
  <li><a href="https://github.com/lucasem">@lucasem</a> fixed <a href="https://github.com/rust-lang/rust/pull/46234">typo in core::marker</a>.</li>
  <li><a href="https://github.com/davidalber">@davidalber</a> added <a href="https://github.com/rust-lang/rust/pull/46201"><code class="highlighter-rouge">eprint*!</code> to the list of macros in the <code class="highlighter-rouge">format!</code> family</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/46247">warnings for markdown doc generation</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46175">global doc search</a>, speedup <a href="https://github.com/rust-lang/rust/pull/46221">search loading when search url is received</a> and removed <a href="https://github.com/rust-lang/rust/pull/46224">invalid doc link</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a> and showed in docs <a href="https://github.com/rust-lang/rust/pull/45039">whether the return type of a function impls Iterator/Read/Write</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/45454">Release notes for 1.22.0</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> used <a href="https://github.com/rust-lang/rust/pull/46024">the proper term when using non-existing variant</a>, used <a href="https://github.com/rust-lang/rust/pull/45946">multiline text for crate conflict diagnostics</a> and removed <a href="https://github.com/rust-lang/rust/pull/45947">dereference suggestion on tuple argument</a>.</li>
  <li><a href="https://github.com/fhartwig">@fhartwig</a> made <a href="https://github.com/rust-lang/rust/pull/45645">rustdoc not include self-by-value methods from Deref target</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/46035">structured suggestions for various “use” suggestions</a>, included <a href="https://github.com/rust-lang/rust/pull/46052">rendered diagnostic in json</a> and checked <a href="https://github.com/rust-lang/rust/pull/46116">//~ERROR comments in ui tests</a>.</li>
  <li><a href="https://github.com/vitiral">@vitiral</a> added <a href="https://github.com/rust-lang/rust/pull/46088">doc for doing <code class="highlighter-rouge">Read</code> from <code class="highlighter-rouge">&amp;str</code></a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/45998">broken CSS for book redirect pages</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> amended <a href="https://github.com/rust-lang/rust/pull/46190">RELEASES for 1.22.1</a>.</li>
  <li><a href="https://github.com/aqrln">@aqrln</a> fixed <a href="https://github.com/rust-lang/rust/pull/46141">a typo in ToSocketAddrs documentation</a>.</li>
  <li><a href="https://github.com/martinlindhe">@martinlindhe</a> fixed <a href="https://github.com/rust-lang/rust/pull/46157">some typos</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/46081">path search</a> and displayed <a href="https://github.com/rust-lang/rust/pull/46134">negative traits implementation</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 28th of November 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>