+++
title = "This Week in Rust Docs 47"
date = "2017-03-12T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-47"
+++
<p>Hello and welcome to <em>This Week in Rust Docs</em>!</p>

<p><em>This Week in Rust Docs</em> is openly developed <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs">on GitHub</a>.
If you find any errors in this week’s issue, <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs/pulls">please submit a PR</a>.</p>

<p>And of course, don’t forget to look at the docs:</p>

<ul>
  <li><a href="https://doc.rust-lang.org/">Stable</a></li>
  <li><a href="http://doc.rust-lang.org/beta/">Beta</a></li>
  <li><a href="http://doc.rust-lang.org/nightly/">Nightly</a></li>
</ul>

<p>This week’s edition was edited by <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>Nothing.</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/Dowwie">@Dowwie</a> updated <a href="https://github.com/rust-lang/rust/pull/39691">attributes.md - Last sentence updated to reflect support for custom attributes</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/40452">basic documentation/examples for six unstable features.</a>.</li>
  <li><a href="https://github.com/tschottdorf">@tschottdorf</a> improved <a href="https://github.com/rust-lang/rust/pull/40453">wording in the -{W,A,F,D} options</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39513">missing urls and small nits</a>, improved <a href="https://github.com/rust-lang/rust/pull/39814">invalid call on non-function error message</a>, fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a> and replaced <a href="https://github.com/rust-lang/rust/pull/40338">hoedown with pulldown in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> improved <a href="https://github.com/rust-lang/rust/pull/40265">the style of the sidebar in rustdoc output</a>.</li>
  <li><a href="https://github.com/jdhorwitz">@jdhorwitz</a> helped <a href="https://github.com/rust-lang/rust/pull/40226">people find String::as_bytes() for UTF-8</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> added <a href="https://github.com/rust-lang/rust/pull/40154">unstable book to the bookshelf</a> and extracted <a href="https://github.com/rust-lang/rust/pull/40222">nomicon to its own repo</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> cleaned up <a href="https://github.com/rust-lang/rust/pull/39713">“pattern doesn’t bind x” messages</a>, pointed to <a href="https://github.com/rust-lang/rust/pull/39202">enclosing block/fn on nested unsafe</a> and fixed <a href="https://github.com/rust-lang/rust/pull/40287">incorrect span label formatting</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> fixed <a href="https://github.com/rust-lang/rust/pull/40258">description of closure coercion feature</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/39835">release notes for 1.16</a>.</li>
  <li><a href="https://github.com/bjorn3">@bjorn3</a> improved <a href="https://github.com/rust-lang/rust/pull/40146">docs of rusty parts of typeck</a>.</li>
  <li><a href="https://github.com/Nashenas88">@Nashenas88</a> fixed <a href="https://github.com/rust-lang/rust/pull/40345">missing backtick typo</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> fixed <a href="https://github.com/rust-lang/rust/pull/40316">a typo in the docs</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> fixed <a href="https://github.com/rust-lang/rust/pull/40092">suggestion span error with a line containing multibyte characters</a>.</li>
  <li><a href="https://github.com/DirkyJerky">@DirkyJerky</a> clarified <a href="https://github.com/rust-lang/rust/pull/40423">docs in <code class="highlighter-rouge">VecDeque::resize</code></a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> distinguished <a href="https://github.com/rust-lang/rust/pull/40386">the ways <code class="highlighter-rouge">CStr::from_bytes_with_nul</code> can fail</a>, documented <a href="https://github.com/rust-lang/rust/pull/40335">why <code class="highlighter-rouge">str.to_{lower,upper}case</code> return <code class="highlighter-rouge">String</code></a> and clarified <a href="https://github.com/rust-lang/rust/pull/40333">handling of <code class="highlighter-rouge">src</code> in <code class="highlighter-rouge">ptr::write</code></a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> fixed <a href="https://github.com/rust-lang/rust/pull/40268">normalization error</a>.</li>
  <li><a href="https://github.com/crazymerlyn">@crazymerlyn</a> updated <a href="https://github.com/rust-lang/rust/pull/40326">link to COMPILER_TESTS.md in CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/joelgallant">@joelgallant</a> made some <a href="https://github.com/rust-lang/rust/pull/40321">README formatting in configure/make section</a>.</li>
  <li><a href="https://github.com/mmatyas">@mmatyas</a> fixed <a href="https://github.com/rust-lang/rust/pull/40292">text formatting in README</a>.</li>
  <li><a href="https://github.com/malbarbo">@malbarbo</a> removed <a href="https://github.com/rust-lang/rust/pull/40293">extra space in test description (of a mod test)</a>.</li>
  <li><a href="https://github.com/oconnor663">@oconnor663</a> clarified <a href="https://github.com/rust-lang/rust/pull/40283">docs for Args and ArgsOs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/40419">associated consts display</a>, added <a href="https://github.com/rust-lang/rust/pull/40299">missing example for Display::fmt</a>, cleaned up <a href="https://github.com/rust-lang/rust/pull/40278">rustdoc css</a> and added <a href="https://github.com/rust-lang/rust/pull/40327">missing urls in some macros doc</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 15th of March 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>