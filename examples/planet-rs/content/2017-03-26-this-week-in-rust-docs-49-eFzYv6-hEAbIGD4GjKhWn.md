+++
title = "This Week in Rust Docs 49"
date = "2017-03-26T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-49"
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
  <li><a href="https://github.com/Dowwie">@Dowwie</a> updated <a href="https://github.com/rust-lang/rust/pull/39691">attributes.md : last sentence updated to reflect support for custom attributes</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39513">missing urls and small nits</a>, fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>, replaced <a href="https://github.com/rust-lang/rust/pull/40338">hoedown with pulldown in rustdoc</a> and replaced <a href="https://github.com/rust-lang/rust/pull/40338">hoedown with pull in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> updated <a href="https://github.com/rust-lang/rust/pull/40803">CONTRIBUTING.md regarding documentation contributions</a>.</li>
  <li><a href="https://github.com/s3rvac">@s3rvac</a> fixed <a href="https://github.com/rust-lang/rust/pull/40794">formatting in the docs for std::process::Command::envs()</a> and fixed <a href="https://github.com/rust-lang/rust/pull/40648">a typo in path.rs docs</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/40756">markdown links to pdqsort</a>, added <a href="https://github.com/rust-lang/rust/pull/40619">docs for sort_unstable to unstable book</a> and made <a href="https://github.com/rust-lang/rust/pull/40722">various fixes to wording consistency in the docs</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> fixed <a href="https://github.com/rust-lang/rust/pull/40567">documentation sorting</a>.</li>
  <li><a href="https://github.com/manuel-rhdt">@manuel-rhdt</a> fixed <a href="https://github.com/rust-lang/rust/pull/40715">doc error for ExactSizeIterator</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/40627">diagnostic for incorrect <code class="highlighter-rouge">pub (restriction)</code></a>.</li>
  <li><a href="https://github.com/SamWhited">@SamWhited</a> added <a href="https://github.com/rust-lang/rust/pull/40723">explanation for E0090 error message</a> and made <a href="https://github.com/rust-lang/rust/pull/40692">str docs consistently punctuated</a>.</li>
  <li><a href="https://github.com/Cldfire">@Cldfire</a> removed <a href="https://github.com/rust-lang/rust/pull/40725">duplicated styling in main.css</a>.</li>
  <li><a href="https://github.com/jswalden">@jswalden</a> fixed <a href="https://github.com/rust-lang/rust/pull/40621">a spelling error in HashMap documentation, and slightly reworded surrounding text for precision</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> extracted <a href="https://github.com/rust-lang/rust/pull/40332">book into a submodule</a>.</li>
  <li><a href="https://github.com/dwrensha">@dwrensha</a> fixed <a href="https://github.com/rust-lang/rust/pull/40576">innacuracy in mir TerminatorKind::SwitchInt docs</a>.</li>
  <li><a href="https://github.com/DaseinPhaos">@DaseinPhaos</a> fixed<a href="https://github.com/rust-lang/rust/pull/40667">typo in <code class="highlighter-rouge">ptr</code> doc</a>.</li>
  <li><a href="https://github.com/jdhorwitz">@jdhorwitz</a> cleaned <a href="https://github.com/rust-lang/rust/pull/40502">up visuals on error index</a> and improved <a href="https://github.com/rust-lang/rust/pull/40312">process docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/40689">whitespace around “=” in assoc items</a>, fixed <a href="https://github.com/rust-lang/rust/pull/40690">invalid linking in iter docs</a>, added <a href="https://github.com/rust-lang/rust/pull/40671">missing urls in Option enum</a> and fixed <a href="https://github.com/rust-lang/rust/pull/40587">invalid debug display for associated consts</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 29th of March 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>