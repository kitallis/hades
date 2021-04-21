+++
title = "This Week in Rust Docs 64"
date = "2017-07-16T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-64"
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

<p><a href="https://github.com/steveklabnik">@steveklabnik</a> ended the first version of <a href="https://github.com/steveklabnik/rustdoc">the rewrite of rustdoc</a> using RLS. It’s far from done but don’t hesitate to give it a try!</p>

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
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/42503">releases notes for 1.19</a>.</li>
  <li><a href="https://github.com/rthomas">@rthomas</a> updated <a href="https://github.com/rust-lang/rust/pull/42837">docs on Error struct</a>.</li>
  <li><a href="https://github.com/Emilgardis">@Emilgardis</a> fixed <a href="https://github.com/rust-lang/rust/pull/42270">wrong report on closure args mismatch when a ref is expected but not found</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> reordered <a href="https://github.com/rust-lang/rust/pull/43251">span suggestions to appear below main labels</a>.</li>
  <li><a href="https://github.com/Others">@Others</a> improved <a href="https://github.com/rust-lang/rust/pull/43256">panic docs for Instant::duration_since</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> clarified <a href="https://github.com/rust-lang/rust/pull/43176">wording for E0122</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, added <a href="https://github.com/rust-lang/rust/pull/43009">errors when doc comments are added where they’re unused</a>, added <a href="https://github.com/rust-lang/rust/pull/43220">TryFrom implementation for bool, f32 and f64</a> and added <a href="https://github.com/rust-lang/rust/pull/43173">a message when a variable name collides with a function name</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/42670">hint about the return code of panic!</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> refactored <a href="https://github.com/rust-lang/rust/pull/42897">pretty printing slightly</a> and tested <a href="https://github.com/rust-lang/rust/pull/43152">src/doc once more</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/43240">the books.</a> and added <a href="https://github.com/rust-lang/rust/pull/43187">the Code of Conduct to the repository</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> fixed <a href="https://github.com/rust-lang/rust/pull/43229">minor typo in std::path documentation.</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> documented <a href="https://github.com/rust-lang/rust/pull/42926">what happens on failure in path ext is_file is_dir</a>.</li>
  <li><a href="https://github.com/jgallag88">@jgallag88</a> added <a href="https://github.com/rust-lang/rust/pull/43136">warning to BufWriter documentation</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> cleaned up <a href="https://github.com/rust-lang/rust/pull/43006">some code</a>, added <a href="https://github.com/rust-lang/rust/pull/43130">spacing between trait functions in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/43145">a failure in case nothing to run was found</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 19th of July 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>