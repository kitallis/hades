+++
title = "This Week in Rust Docs 45"
date = "2017-02-26T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-45"
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
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/39835">release notes for 1.16</a>.</li>
  <li><a href="https://github.com/pmer">@pmer</a> used <a href="https://github.com/rust-lang/rust/pull/40102">“macOS” terminology consistently</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> published <a href="https://github.com/rust-lang/rust/pull/39986">docs for the proc_macro crate</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> replaced <a href="https://github.com/rust-lang/rust/pull/40056">./configure with config.toml in README.md and CONTRIBUTING.md</a> and added <a href="https://github.com/rust-lang/rust/pull/39738">notes about capacity effects to Vec::truncate()</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> distinguished <a href="https://github.com/rust-lang/rust/pull/39458">guesses from suggestions</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> used <a href="https://github.com/rust-lang/rust/pull/39441">help rather than span note for no method error; a slight rephrasing</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/39300">code in <code class="highlighter-rouge">rustc --explain</code></a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/40057">nightly-only experimental API display</a>, added <a href="https://github.com/rust-lang/rust/pull/40081">missing url in sync structs</a>, added <a href="https://github.com/rust-lang/rust/pull/39513">missing urls and small nits</a>, improved <a href="https://github.com/rust-lang/rust/pull/39814">invalid call on non-function error message</a>, fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/40033">missing urls and examples for Condvar docs</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a> and improved <a href="https://github.com/rust-lang/rust/pull/39944">associated constant rendering in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/mp4096">@mp4096</a> wrote <a href="https://github.com/rust-lang/rust/pull/39955">better explanation for return values for min, max functions for the Iterator trait</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> provided <a href="https://github.com/rust-lang/rust/pull/39953">suggestions for unknown macros imported with <code class="highlighter-rouge">use</code></a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> ported <a href="https://github.com/rust-lang/rust/pull/39855">the reference to mdbook</a>, created <a href="https://github.com/rust-lang/rust/pull/39866">the Unstable Book</a>, reenabled <a href="https://github.com/rust-lang/rust/pull/39976">the linkchecker for books</a> and updated <a href="https://github.com/rust-lang/rust/pull/39966">mdbook version</a>.</li>
  <li><a href="https://github.com/arthurprs">@arthurprs</a> fixed <a href="https://github.com/rust-lang/rust/pull/39937">spelling in hashmap comments</a>.</li>
  <li><a href="https://github.com/JDemler">@JDemler</a> added <a href="https://github.com/rust-lang/rust/pull/39845">Documentation for Custom Attributes and Error Reporting in Procedural Macros</a>.</li>
  <li><a href="https://github.com/jrmuizel">@jrmuizel</a> removed <a href="https://github.com/rust-lang/rust/pull/39304">obsolete documentation about drop-flags</a>.</li>
  <li><a href="https://github.com/mina86">@mina86</a> updated The Book: <a href="https://github.com/rust-lang/rust/pull/39777">binary prefixed are defined by IEC and not in SI</a>.</li>
  <li><a href="https://github.com/tclfs">@tclfs</a> fixed <a href="https://github.com/rust-lang/rust/pull/40078">a typo</a>.</li>
  <li><a href="https://github.com/cynicaldevil">@cynicaldevil</a> added <a href="https://github.com/rust-lang/rust/pull/40031">test for inclusive_range_syntax in compile-fail test suite</a>.</li>
  <li><a href="https://github.com/Rufflewind">@Rufflewind</a> added <a href="https://github.com/rust-lang/rust/pull/40069">Gankro’s table to nomicon/src/phantom-data.md</a> and resolved <a href="https://github.com/rust-lang/rust/pull/39748">ambiguities of Generics in The Book</a>.</li>
  <li><a href="https://github.com/danobi">@danobi</a> moved <a href="https://github.com/rust-lang/rust/pull/40086">COMPILER_TESTS.md out of the root directory</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> displayed <a href="https://github.com/rust-lang/rust/pull/39905">properly note/expected details</a>.</li>
  <li><a href="https://github.com/sgrif">@sgrif</a> fixed <a href="https://github.com/rust-lang/rust/pull/39940">indentation of error message</a>.</li>
  <li><a href="https://github.com/tomwhoiscontrary">@tomwhoiscontrary</a> corrected <a href="https://github.com/rust-lang/rust/pull/40071">another typo in procedural macros chapter of the Book</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> added <a href="https://github.com/rust-lang/rust/pull/39886">additional docs for Vec, String, and slice trait impls</a>.</li>
  <li><a href="https://github.com/ArtBears">@ArtBears</a> fixed <a href="https://github.com/rust-lang/rust/pull/39965">a typo in CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> set <a href="https://github.com/rust-lang/rust/pull/39859">rustdoc –test files’ path relative to the current directory</a>, improved <a href="https://github.com/rust-lang/rust/pull/39765">file not found for module error</a>, added <a href="https://github.com/rust-lang/rust/pull/40010">missing urls and examples into Barrier structs</a> and added <a href="https://github.com/rust-lang/rust/pull/40052">missing urls in MutexGuard docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 1st of March 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>