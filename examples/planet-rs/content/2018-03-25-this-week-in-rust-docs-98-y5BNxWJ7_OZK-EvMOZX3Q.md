+++
title = "This Week in Rust Docs 98"
date = "2018-03-25T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-98"
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

<p>Nothing interesting enough.</p>

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
  <li><a href="https://github.com/FraGag">@FraGag</a> improved <a href="https://github.com/rust-lang/rust/pull/48171">documentation of Clone and Copy implementors</a>.</li>
  <li><a href="https://github.com/tinaun">@tinaun</a> removed <a href="https://github.com/rust-lang/rust/pull/48709">erroneous error message when checking impl trait params</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> modified <a href="https://github.com/rust-lang/rust/pull/48914">E0389 error message</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> suppressed <a href="https://github.com/rust-lang/rust/pull/49064">the default allow(unused) under –display-warnings in rustdoctest</a> and moved <a href="https://github.com/rust-lang/rust/pull/49286">the “important traits” button to beside the type in rustdoc</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> documented <a href="https://github.com/rust-lang/rust/pull/48932">when types have OS-dependent sizes</a>.</li>
  <li><a href="https://github.com/klnusbaum">@klnusbaum</a> provided <a href="https://github.com/rust-lang/rust/pull/48708">better borrow checker error message</a>.</li>
  <li><a href="https://github.com/pthariensflame">@pthariensflame</a> made <a href="https://github.com/rust-lang/rust/pull/49351">a minor message/label formatting consistency fix</a>.</li>
  <li><a href="https://github.com/krk">@krk</a> added <a href="https://github.com/rust-lang/rust/pull/49338">submodule fetch instructions</a>.</li>
  <li><a href="https://github.com/chisophugis">@chisophugis</a> fixed <a href="https://github.com/rust-lang/rust/pull/49353">confusing doc for <code class="highlighter-rouge">scan</code></a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> added <a href="https://github.com/rust-lang/rust/pull/49304">support for universal_impl_trait in rustdoc</a>.</li>
  <li><a href="https://github.com/PramodBisht">@PramodBisht</a> fixed <a href="https://github.com/rust-lang/rust/pull/48946">doc comments present after a particular syntax error cause an unhelpful error message to be output</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/49258">suggestion of <code class="highlighter-rouge">!</code> for erroneous identifier <code class="highlighter-rouge">not</code></a> and pointed <a href="https://github.com/rust-lang/rust/pull/49197">to value in “value assigned is never read” lint</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48173">error codes for libsyntax_ext</a>, fixed <a href="https://github.com/rust-lang/rust/pull/49333">impl assoc constant link not working</a>, removed <a href="https://github.com/rust-lang/rust/pull/49335">unneeded trait implementations titles</a>, added <a href="https://github.com/rust-lang/rust/pull/48999">repeat method on slice</a> and proposed <a href="https://github.com/rust-lang/rust/pull/49223">a variant if it is an enum for E0599</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> exposed <a href="https://github.com/rust-lang/rust/pull/48759">#[target_feature] attributes as doc(cfg) flags in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/49028">an “unstable features” chapter to the rustdoc book</a> and whitelisted <a href="https://github.com/rust-lang/rust/pull/49225">every target feature for rustdoc</a>.</li>
  <li><a href="https://github.com/csmoe">@csmoe</a> improved <a href="https://github.com/rust-lang/rust/pull/49104">error message of inner attribute syntax</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> added <a href="https://github.com/rust-lang/rust/pull/49105">an example of lossy decoding to str::Utf8Error docs</a> and fixed <a href="https://github.com/rust-lang/rust/pull/49161">incorrect copy-paste for new <code class="highlighter-rouge">X?</code> in formatting strings</a>.</li>
  <li><a href="https://github.com/ysiraichi">@ysiraichi</a> suggested <a href="https://github.com/rust-lang/rust/pull/48834">removing <code class="highlighter-rouge">&amp;</code>s</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> improved <a href="https://github.com/rust-lang/rust/pull/48909">lint for type alias bounds</a>.</li>
  <li><a href="https://github.com/klnusbaum">@klnusbaum</a> renamed <a href="https://github.com/rust-lang/rust/pull/49035">epoch to edition</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/49318">books for next release</a>.</li>
  <li><a href="https://github.com/ordovicia">@ordovicia</a> improved <a href="https://github.com/rust-lang/rust/pull/49268">diagnostics for ‘..’ pattern fragment not in the last position</a>.</li>
  <li><a href="https://github.com/Centril">@Centril</a> documented <a href="https://github.com/rust-lang/rust/pull/49229">format_args! / Arguments&lt;’a&gt; behavior wrt. Display and Debug</a>.</li>
  <li><a href="https://github.com/davidtwco">@davidtwco</a> hosted <a href="https://github.com/rust-lang/rust/pull/49193">compiler documentation</a>.</li>
  <li><a href="https://github.com/sanxiyn">@sanxiyn</a> documented <a href="https://github.com/rust-lang/rust/pull/49169">only-X test header</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> reduced <a href="https://github.com/rust-lang/rust/pull/49160">the diagnostic spam when multiple fields are missing in pattern</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/48374">RELEASES.md for 1.25.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48596">warning for invalid start of code blocks in rustdoc</a>, and made <a href="https://github.com/rust-lang/rust/pull/49029">Atomic doc examples specific to each type</a>, fixed <a href="https://github.com/rust-lang/rust/pull/49312">IE11 search</a>, fixed <a href="https://github.com/rust-lang/rust/pull/49189">automatic urls with backticks</a> and fixed <a href="https://github.com/rust-lang/rust/pull/49152">events handling in rustdoc</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 27th of March 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>