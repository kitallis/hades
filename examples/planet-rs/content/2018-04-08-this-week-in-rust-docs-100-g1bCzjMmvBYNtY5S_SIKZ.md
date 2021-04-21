+++
title = "This Week in Rust Docs 100"
date = "2018-04-08T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-100"
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

<p>🎉 It’s the hundredth post! 🎉</p>

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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> moved <a href="https://github.com/rust-lang/rust/pull/49286">the “important traits” button to beside the type in rustdoc</a>.</li>
  <li><a href="https://github.com/klnusbaum">@klnusbaum</a> provided <a href="https://github.com/rust-lang/rust/pull/48708">better borrow checker error message</a>.</li>
  <li><a href="https://github.com/PramodBisht">@PramodBisht</a> fixed <a href="https://github.com/rust-lang/rust/pull/48946">doc comments present after a particular syntax error cause an unhelpful error message to be output</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/49258">suggestion of <code class="highlighter-rouge">!</code> for erroneous identifier <code class="highlighter-rouge">not</code></a> and pointed <a href="https://github.com/rust-lang/rust/pull/49197">to value in “value assigned is never read” lint</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> soft-deprecated <a href="https://github.com/rust-lang/rust/pull/49536">the description() method of the Error trait</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/49523">RELEASES.md for 1.26.0</a>.</li>
  <li><a href="https://github.com/sgrif">@sgrif</a> removed <a href="https://github.com/rust-lang/rust/pull/48407">the “leak check” in favor of “universes”</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/49465">docs for the test crate with the std docs</a>.</li>
  <li><a href="https://github.com/ecstatic-morse">@ecstatic-morse</a> rewrote <a href="https://github.com/rust-lang/rust/pull/49767">docs for <code class="highlighter-rouge">std::ptr</code></a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> modified <a href="https://github.com/rust-lang/rust/pull/48914">compile-fail/E0389 error message</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> added <a href="https://github.com/rust-lang/rust/pull/49707">“the Rustc book”</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48173">error codes for libsyntax_ext</a>, added <a href="https://github.com/rust-lang/rust/pull/48999">repeat method on slice</a>, stabilized <a href="https://github.com/rust-lang/rust/pull/49546">short error format</a>, added <a href="https://github.com/rust-lang/rust/pull/49504">page to list all crate’s items</a>, added <a href="https://github.com/rust-lang/rust/pull/49542">warning if a resolution failed</a> and added <a href="https://github.com/rust-lang/rust/pull/49757">specific never search</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/FraGag">@FraGag</a> improved <a href="https://github.com/rust-lang/rust/pull/48171">documentation of Clone and Copy implementors</a>.</li>
  <li><a href="https://github.com/tinaun">@tinaun</a> removed <a href="https://github.com/rust-lang/rust/pull/48709">erroneous error message when checking impl trait params</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> suppressed <a href="https://github.com/rust-lang/rust/pull/49064">the default allow(unused) under –display-warnings in rustdoctest</a> and added <a href="https://github.com/rust-lang/rust/pull/49451">an –edition flag to compile docs/doctests with a certain edition in rustdoc</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/49418">network byte order conversions for integer / IP address conversions</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> added <a href="https://github.com/rust-lang/rust/pull/49532">test for rustdoc ignore test</a> and fixed <a href="https://github.com/rust-lang/rust/pull/49478">escaped backslash in windows file not found message</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/49623">mdbook</a> and re-wrote <a href="https://github.com/rust-lang/rust/pull/49628">the documentation index</a>.</li>
  <li><a href="https://github.com/rolfvandekrol">@rolfvandekrol</a> fixed <a href="https://github.com/rust-lang/rust/pull/49599">typo</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> added <a href="https://github.com/rust-lang/rust/pull/49594">some performance guidance to std::fs and std::io docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/49335">unneeded trait implementations titles</a>, added <a href="https://github.com/rust-lang/rust/pull/49512">support for variant and types fields for intra links</a>, added <a href="https://github.com/rust-lang/rust/pull/49516">missing anchor for union type fields</a>, fixed <a href="https://github.com/rust-lang/rust/pull/49515">targetted value background</a>, fixed <a href="https://github.com/rust-lang/rust/pull/49510">anchor position on fields</a>, fixed <a href="https://github.com/rust-lang/rust/pull/49652">anchors issue when everything is collapsed</a> and fixed <a href="https://github.com/rust-lang/rust/pull/49603">url for intra link provided method</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 10th of April 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>