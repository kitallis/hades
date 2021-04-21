+++
title = "This Week in Rust Docs 85"
date = "2017-12-10T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-85"
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
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>, showed <a href="https://github.com/rust-lang/rust/pull/46350">closure signature on type errors</a> and resolved <a href="https://github.com/rust-lang/rust/pull/46608">type on return type suggestion</a>.</li>
  <li><a href="https://github.com/canndrew">@canndrew</a> added <a href="https://github.com/rust-lang/rust/pull/46232">docs for never primitive</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/46461">type error method suggestions use whitelisted identity-like conversions</a> and added <a href="https://github.com/rust-lang/rust/pull/46248">one-time diagnostics for private enum variants glob reexport</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/46327">RELEASES.md for 1.23.0</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> turned <a href="https://github.com/rust-lang/rust/pull/46567">errors from loading external docs into a proper lint</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/46359">display of hidden types in rustdoc</a> and stabilized <a href="https://github.com/rust-lang/rust/pull/46501">allow_fail flag test feature</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> displayed <a href="https://github.com/rust-lang/rust/pull/45953"><code class="highlighter-rouge">\t</code> in diagnostics code as four spaces</a>.</li>
  <li><a href="https://github.com/JRegimbal">@JRegimbal</a> changed <a href="https://github.com/rust-lang/rust/pull/45898">“Types/modules” title of search tab to be more accurate</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> clarified <a href="https://github.com/rust-lang/rust/pull/46136">what <code class="highlighter-rouge">-D warnings</code> or <code class="highlighter-rouge">-F warnings</code> does</a>.</li>
  <li><a href="https://github.com/nak3">@nak3</a> fixed <a href="https://github.com/rust-lang/rust/pull/46463">invalid docs path for compiler plugins</a> and fixed <a href="https://github.com/rust-lang/rust/pull/46465">invalid link to lint_plugin_test.rs</a>.</li>
  <li><a href="https://github.com/tromey">@tromey</a> fixed <a href="https://github.com/rust-lang/rust/pull/46432">documentation for DecodeUtf16Error</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> mentionned <a href="https://github.com/rust-lang/rust/pull/46431">the name of ? in Result’s docs</a>.</li>
  <li><a href="https://github.com/vramana">@vramana</a> fixed <a href="https://github.com/rust-lang/rust/pull/46572">bad error message for cannot_reborrow_already_uniquely_borrowed</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> included <a href="https://github.com/rust-lang/rust/pull/46603"><code class="highlighter-rouge">impl [u8]</code> in the docs</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> specified <a href="https://github.com/rust-lang/rust/pull/46416">that macro <code class="highlighter-rouge">cfg!</code> evaluating at compile-time</a>.</li>
  <li><a href="https://github.com/AgustinCB">@AgustinCB</a> modified <a href="https://github.com/rust-lang/rust/pull/46497">message for keyword as identifier name</a>.</li>
  <li><a href="https://github.com/notriddle">@notriddle</a> renamed <a href="https://github.com/rust-lang/rust/pull/46187">C-like enum to Field-less enum</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> documented <a href="https://github.com/rust-lang/rust/pull/46483">behavior of <code class="highlighter-rouge">ptr::swap</code> with overlapping regions of memory</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> added <a href="https://github.com/rust-lang/rust/pull/46512">compile_error macro examples</a>.</li>
  <li><a href="https://github.com/timotree3">@timotree3</a> updated <a href="https://github.com/rust-lang/rust/pull/46495">old link</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/46247">warnings for markdown doc generation</a>, speedup <a href="https://github.com/rust-lang/rust/pull/46221">search loading when search url is received in rust docs</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46433">deduplication of items in rust docs search</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46454">search results overlap</a>, moved <a href="https://github.com/rust-lang/rust/pull/46444">colors to main.css</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46611">switched types in type mismatch error</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46586">doc important trait display on mobile</a>, greatly improved <a href="https://github.com/rust-lang/rust/pull/46526">sidebar when width &lt; 700px</a> and improved <a href="https://github.com/rust-lang/rust/pull/46502">search style</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 12th of December 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>