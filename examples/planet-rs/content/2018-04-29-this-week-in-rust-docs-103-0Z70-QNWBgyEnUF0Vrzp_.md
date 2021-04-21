+++
title = "This Week in Rust Docs 103"
date = "2018-04-29T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-103"
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

<p>We added <a href="https://github.com/rust-lang/rust/pull/49954">settings into generated documentation</a> so you can have your own setup. We’ll add more options soon to make rust documentation browsing more personal and comfortable for everyone!</p>

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
  <li><a href="https://github.com/sgrif">@sgrif</a> removed <a href="https://github.com/rust-lang/rust/pull/48407">the “leak check” in favor of “universes”</a>.</li>
  <li><a href="https://github.com/ecstatic-morse">@ecstatic-morse</a> rewrote <a href="https://github.com/rust-lang/rust/pull/49767">docs for <code class="highlighter-rouge">std::ptr</code></a>.</li>
  <li><a href="https://github.com/da-x">@da-x</a> included <a href="https://github.com/rust-lang/rust/pull/49898">scope names in diagnostics</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> clarified <a href="https://github.com/rust-lang/rust/pull/49743">the difference between get_mut and into_mut for OccupiedEntry</a>.</li>
  <li><a href="https://github.com/ibabushkin">@ibabushkin</a> refactored <a href="https://github.com/rust-lang/rust/pull/49711">auto trait handling in librustdoc to be accessible from librustc</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> added <a href="https://github.com/rust-lang/rust/pull/49611">edition-gated keywords</a> and used <a href="https://github.com/rust-lang/rust/pull/50204">enum for approximate suggestions</a>.</li>
  <li><a href="https://github.com/z4v1er">@z4v1er</a> fixed <a href="https://github.com/rust-lang/rust/pull/50217">typo</a>.</li>
  <li><a href="https://github.com/rizakrko">@rizakrko</a> added <a href="https://github.com/rust-lang/rust/pull/50161">missing implementation hint</a>.</li>
  <li><a href="https://github.com/mulkieran">@mulkieran</a> made <a href="https://github.com/rust-lang/rust/pull/50255">some documentation fixes for the Write trait</a>.</li>
  <li><a href="https://github.com/kornelski">@kornelski</a> buried <a href="https://github.com/rust-lang/rust/pull/50163">Error::description()</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilized <a href="https://github.com/rust-lang/rust/pull/49546">short error format</a>, prevented <a href="https://github.com/rust-lang/rust/pull/50305">infinite recursion of modules in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/50302">query search order check</a> and made <a href="https://github.com/rust-lang/rust/pull/50259">some rustdoc improvements</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/PramodBisht">@PramodBisht</a> fixed <a href="https://github.com/rust-lang/rust/pull/48946">doc comments present after a particular syntax error cause an unhelpful error message to be output</a>.</li>
  <li><a href="https://github.com/ecstatic-morse">@ecstatic-morse</a> added <a href="https://github.com/rust-lang/rust/pull/49829">doc links to <code class="highlighter-rouge">std::os</code> extension traits</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> added <a href="https://github.com/rust-lang/rust/pull/49707">“the Rustc book”</a>.</li>
  <li><a href="https://github.com/andjo403">@andjo403</a> made <a href="https://github.com/rust-lang/rust/pull/50134">rustdoc tests follow the jobserver limit of threads</a>.</li>
  <li><a href="https://github.com/dmizuk">@dmizuk</a> marked <a href="https://github.com/rust-lang/rust/pull/49858"><code class="highlighter-rouge">ptr::Unique</code> with <code class="highlighter-rouge">#[doc(hidden)]</code></a>.</li>
  <li><a href="https://github.com/ralfbiedert">@ralfbiedert</a> added <a href="https://github.com/rust-lang/rust/pull/50219">missing <code class="highlighter-rouge">.</code> in docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/48999">repeat method on slice</a>, added <a href="https://github.com/rust-lang/rust/pull/49954">rustdoc settings menu</a>, fixed <a href="https://github.com/rust-lang/rust/pull/50118">search bar bug</a>, fixed <a href="https://github.com/rust-lang/rust/pull/50284">search load page failure</a>, added <a href="https://github.com/rust-lang/rust/pull/50229">setting to go to item if there is only one result</a>, added <a href="https://github.com/rust-lang/rust/pull/50231">more doc aliases</a> and made <a href="https://github.com/rust-lang/rust/pull/50214">js improvements</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 1st of May 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>