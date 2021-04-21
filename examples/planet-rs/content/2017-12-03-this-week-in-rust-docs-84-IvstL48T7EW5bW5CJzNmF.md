+++
title = "This Week in Rust Docs 84"
date = "2017-12-03T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-84"
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
  <li><a href="https://github.com/estebank">@estebank</a> displayed <a href="https://github.com/rust-lang/rust/pull/45953"><code class="highlighter-rouge">\t</code> in diagnostics code as four spaces</a>, highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a> and showed <a href="https://github.com/rust-lang/rust/pull/46350">closure signature on type errors</a>.</li>
  <li><a href="https://github.com/JRegimbal">@JRegimbal</a> changed <a href="https://github.com/rust-lang/rust/pull/45898">“Types/modules” title of search tab to be more accurate</a>.</li>
  <li><a href="https://github.com/canndrew">@canndrew</a> added <a href="https://github.com/rust-lang/rust/pull/46232">docs for never primitive</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> clarified <a href="https://github.com/rust-lang/rust/pull/46136">what <code class="highlighter-rouge">-D warnings</code> or <code class="highlighter-rouge">-F warnings</code> does</a>.</li>
  <li><a href="https://github.com/nak3">@nak3</a> fixed <a href="https://github.com/rust-lang/rust/pull/46463">invalid docs path for compiler plugins</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/46461">type error method suggestions use whitelisted identity-like conversions</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46156">the size of bool</a>.</li>
  <li><a href="https://github.com/tromey">@tromey</a> fixed <a href="https://github.com/rust-lang/rust/pull/46432">documentation for DecodeUtf16Error</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> mentionned <a href="https://github.com/rust-lang/rust/pull/46431">the name of ? in Result’s docs</a>.</li>
  <li><a href="https://github.com/archer884">@archer884</a> changed <a href="https://github.com/rust-lang/rust/pull/46341">const error message to use ‘literal’</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/46327">RELEASES.md for 1.23.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/46247">warnings for markdown doc generation</a>, speedup <a href="https://github.com/rust-lang/rust/pull/46221">search loading when search url is received in rust docs</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46433">deduplication of items in rust docs search</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46454">search results overlap</a>, moved <a href="https://github.com/rust-lang/rust/pull/46444">colors to main.css</a> and removed <a href="https://github.com/rust-lang/rust/pull/46359">display of hidden types in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> accounted <a href="https://github.com/rust-lang/rust/pull/45997">for missing keyword in fn/struct definition</a>, suggested <a href="https://github.com/rust-lang/rust/pull/46249">using slice when encountering <code class="highlighter-rouge">let x = ""[..];</code></a>, used <a href="https://github.com/rust-lang/rust/pull/46256">suggestions instead of notes ref mismatches</a>, pointed <a href="https://github.com/rust-lang/rust/pull/46381">to next token when it is in the expected line</a>, highlighted <a href="https://github.com/rust-lang/rust/pull/46349"><code class="highlighter-rouge">&amp;</code> when type matches on type mismatch error</a> and shortened <a href="https://github.com/rust-lang/rust/pull/46282">output of E0391</a>.</li>
  <li><a href="https://github.com/colinmarsh19">@colinmarsh19</a> added <a href="https://github.com/rust-lang/rust/pull/46258">a note “please remove this semicolon”</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/46219">documentation for slice swap/copy/clone operations</a>.</li>
  <li><a href="https://github.com/lucasem">@lucasem</a> fixed <a href="https://github.com/rust-lang/rust/pull/46234">typo in core::marker</a>.</li>
  <li><a href="https://github.com/davidalber">@davidalber</a> added <a href="https://github.com/rust-lang/rust/pull/46201"><code class="highlighter-rouge">eprint*!</code> to the list of macros in the <code class="highlighter-rouge">format!</code> family</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/46384">issues with cross-crate inlined associated items in rustdoc</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> documented <a href="https://github.com/rust-lang/rust/pull/46285">non-obvious behavior of fmt::UpperHex &amp; co for negative integers</a> and expanded <a href="https://github.com/rust-lang/rust/pull/46240">docs of &lt;$Int&gt;::from_str_radix, based on that of char::to_digit</a>.</li>
  <li><a href="https://github.com/chrisduerr">@chrisduerr</a> fixed <a href="https://github.com/rust-lang/rust/pull/46387">rustdoc item summaries that are headers</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/46175">global doc search</a>, removed <a href="https://github.com/rust-lang/rust/pull/46224">invalid doc link</a>, inverted <a href="https://github.com/rust-lang/rust/pull/46392">colors in important traits tooltip</a> and fixed <a href="https://github.com/rust-lang/rust/pull/46326">invalid HTML escape</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 5th of December 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>