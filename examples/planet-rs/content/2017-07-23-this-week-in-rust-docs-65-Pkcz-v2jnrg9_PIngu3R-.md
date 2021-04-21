+++
title = "This Week in Rust Docs 65"
date = "2017-07-23T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-65"
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
  <li><a href="https://github.com/Emilgardis">@Emilgardis</a> fixed <a href="https://github.com/rust-lang/rust/pull/42270">wrong report on closure args mismatch when a ref is expected but not found</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> clarified <a href="https://github.com/rust-lang/rust/pull/43176">wording for E0122</a>.</li>
  <li><a href="https://github.com/afshinm">@afshinm</a> added <a href="https://github.com/rust-lang/rust/pull/43363"><code class="highlighter-rouge">+</code> sign to static lifetime help message</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> adjusted <a href="https://github.com/rust-lang/rust/pull/43386">new suggestions to the suggestion guidelines</a>.</li>
  <li><a href="https://github.com/waywardmonkeys">@waywardmonkeys</a> fixed <a href="https://github.com/rust-lang/rust/pull/43428">some doc comment typos</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/43413">generic example of std::ops::Sub in doc comments</a>.</li>
  <li><a href="https://github.com/xliiv">@xliiv</a> added <a href="https://github.com/rust-lang/rust/pull/43423">simple docs example for struct Cell</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> clarified <a href="https://github.com/rust-lang/rust/pull/43374">that sort_unstable is deterministic</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made <a href="https://github.com/rust-lang/rust/pull/43409">into_iter example more concise</a>.</li>
  <li><a href="https://github.com/cuviper">@cuviper</a> corrected <a href="https://github.com/rust-lang/rust/pull/43401">the spelling of “homogeneous”</a>.</li>
  <li><a href="https://github.com/s3rvac">@s3rvac</a> added <a href="https://github.com/rust-lang/rust/pull/43379">a missing verb to the description of std::process::ExitStatus::success()</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> exposed <a href="https://github.com/rust-lang/rust/pull/43348">all OS-specific modules in libstd doc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, added <a href="https://github.com/rust-lang/rust/pull/43009">lint for when doc comments are added where they’re unused</a> and removed <a href="https://github.com/rust-lang/rust/pull/43397">warn on unused field on union</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/rthomas">@rthomas</a> updated <a href="https://github.com/rust-lang/rust/pull/42837">docs on Error struct</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> reordered <a href="https://github.com/rust-lang/rust/pull/43251">span suggestions to appear below main labels</a>.</li>
  <li><a href="https://github.com/Others">@Others</a> improved <a href="https://github.com/rust-lang/rust/pull/43256">panic docs for Instant::duration_since</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> provided <a href="https://github.com/rust-lang/rust/pull/43416">an actual equivalent to filter_map doc</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> added <a href="https://github.com/rust-lang/rust/pull/43343">an extra note to <code class="highlighter-rouge">late_bound_lifetime_arguments</code> error/lint</a>.</li>
  <li><a href="https://github.com/perryprog">@perryprog</a> made <a href="https://github.com/rust-lang/rust/pull/43323">a less verbose output for unused arguments</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> made <a href="https://github.com/rust-lang/rust/pull/42973">JSON error byte position start at top of file</a>, made <a href="https://github.com/rust-lang/rust/pull/43260">explanatory error on <code class="highlighter-rouge">--print target-spec-json</code> without unstable options</a> and suggested <a href="https://github.com/rust-lang/rust/pull/43178">one-argument enum variant to fix type mismatch when applicable</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/43368">release notes for 1.19.0</a>.</li>
  <li><a href="https://github.com/vbrandl">@vbrandl</a> documented <a href="https://github.com/rust-lang/rust/pull/43252">default values for primitive types</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 26th of July 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>