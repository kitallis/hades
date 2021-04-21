+++
title = "This Week in Rust Docs 46"
date = "2017-03-05T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-46"
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
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> improved <a href="https://github.com/rust-lang/rust/pull/40265">the style of the sidebar in rustdoc output</a>.</li>
  <li><a href="https://github.com/jdhorwitz">@jdhorwitz</a> helped <a href="https://github.com/rust-lang/rust/pull/40226">people find String::as_bytes() for UTF-8</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> added <a href="https://github.com/rust-lang/rust/pull/40154">unstable book to the bookshelf</a> and extracted <a href="https://github.com/rust-lang/rust/pull/40222">nomicon to its own repo</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> cleaned <a href="https://github.com/rust-lang/rust/pull/39713">up “pattern doesn’t bind x” messages</a> and pointed to <a href="https://github.com/rust-lang/rust/pull/39202">enclosing block/fn on nested unsafe</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> fixed <a href="https://github.com/rust-lang/rust/pull/40258">description of closure coercion feature</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/39835">release notes for 1.16</a>.</li>
  <li><a href="https://github.com/Dowwie">@Dowwie</a> updated <a href="https://github.com/rust-lang/rust/pull/39691">attributes.md - Last sentence updated to reflect support for custom attributes</a>.</li>
  <li><a href="https://github.com/APTy">@APTy</a> added <a href="https://github.com/rust-lang/rust/pull/39007">docs and tests for join_multicast_{v4,v6}</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39513">missing urls and small nits</a>, improved <a href="https://github.com/rust-lang/rust/pull/39814">invalid call on non-function error message</a>, fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a> and added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/pmer">@pmer</a> used <a href="https://github.com/rust-lang/rust/pull/40102">“macOS” terminology consistently</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> replaced <a href="https://github.com/rust-lang/rust/pull/40056">./configure with config.toml in README.md and CONTRIBUTING.md</a> and adde <a href="https://github.com/rust-lang/rust/pull/39738">notes about capacity effects to Vec::truncate()</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> moved <a href="https://github.com/rust-lang/rust/pull/40213">the reference into a submodule</a>, updated <a href="https://github.com/rust-lang/rust/pull/40151">mdbook version</a> and sorted <a href="https://github.com/rust-lang/rust/pull/40153">unstable book alphabetically</a>.</li>
  <li><a href="https://github.com/iKevinY">@iKevinY</a> fixed <a href="https://github.com/rust-lang/rust/pull/40170">link in <code class="highlighter-rouge">if let</code> docs</a>.</li>
  <li><a href="https://github.com/letmaik">@letmaik</a> fixed <a href="https://github.com/rust-lang/rust/pull/40194">wrong word used in book page “const and static”</a>.</li>
  <li><a href="https://github.com/d-e-s-o">@d-e-s-o</a> fixed <a href="https://github.com/rust-lang/rust/pull/40175">inconsistency in error output in guessing-game.md</a>.</li>
  <li><a href="https://github.com/MajorBreakfast">@MajorBreakfast</a> added <a href="https://github.com/rust-lang/rust/pull/40169">“the” in the docs</a>, improved <a href="https://github.com/rust-lang/rust/pull/40144">unit-like structs code sample in the docs</a>, changed <a href="https://github.com/rust-lang/rust/pull/40142">“pointers” to “references” in structs docs</a>, made <a href="https://github.com/rust-lang/rust/pull/40131">lifetime elision docs clearer</a> and used <a href="https://github.com/rust-lang/rust/pull/40115">present perfect instead of simple past in the loop docs</a>.</li>
  <li><a href="https://github.com/robinst">@robinst</a> added <a href="https://github.com/rust-lang/rust/pull/40122">an example for how to provide stdin using std::process::Command</a>.</li>
  <li><a href="https://github.com/king6cong">@king6cong</a> fixed <a href="https://github.com/rust-lang/rust/pull/40121">typo</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/40057">nightly-only experimental API display</a>, added <a href="https://github.com/rust-lang/rust/pull/40081">missing url in sync structs</a>, added <a href="https://github.com/rust-lang/rust/pull/40033">missing urls and examples for Condvar docs</a>, improved <a href="https://github.com/rust-lang/rust/pull/39944">associated constant rendering in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/40126">missing docs and examples for fmt::Write</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 8th of March 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>