+++
title = "This Week in Rust Docs 67"
date = "2017-08-06T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-67"
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

<p>The <a href="https://github.com/steveklabnik/rustdoc">rewrite of rustdoc</a> is still under heavy development. Don’t hesitate to give it a try!</p>

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
  <li><a href="https://github.com/RalfJung">@RalfJung</a> clarified <a href="https://github.com/rust-lang/rust/pull/43176">wording for E0122</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> exposed <a href="https://github.com/rust-lang/rust/pull/43348">all OS-specific modules in libstd doc</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/43641">lint casting signed integers smaller than <code class="highlighter-rouge">isize</code> to raw pointers</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/43484">at return type always when type mismatch against it</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/43627">release notes for 1.20</a>.</li>
  <li><a href="https://github.com/ruuda">@ruuda</a> detected <a href="https://github.com/rust-lang/rust/pull/43632">relative urls in tidy check</a> and pointed <a href="https://github.com/rust-lang/rust/pull/43631">“deref coercions” links to new book</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, removed <a href="https://github.com/rust-lang/rust/pull/43397">warn on unused field on union</a>, fixed <a href="https://github.com/rust-lang/rust/pull/43691">rustdoc error on ‘\0’</a> and added <a href="https://github.com/rust-lang/rust/pull/43558">union and const colors</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/xliiv">@xliiv</a> added <a href="https://github.com/rust-lang/rust/pull/43423">simple docs example for struct Cell</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> printed <a href="https://github.com/rust-lang/rust/pull/43515">associated types in traits “implementors” section in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/43529">documentation for function pointers as a primitive</a>, added <a href="https://github.com/rust-lang/rust/pull/43509">[src] links to associated functions inside an impl block in rustdoc</a>, shrank <a href="https://github.com/rust-lang/rust/pull/43602">headings in non-top-level docblocks in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/43560">docs for references as a primitive</a>.</li>
  <li><a href="https://github.com/edaniels">@edaniels</a> fixed <a href="https://github.com/rust-lang/rust/pull/43689">typo in coerce_forced_unit docstring</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/43652">string slice doc</a>, fixed and improved <a href="https://github.com/rust-lang/rust/pull/43619">thread docs</a> and improved <a href="https://github.com/rust-lang/rust/pull/43585">docs &amp; doc examples for HashSet</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> uplifted <a href="https://github.com/rust-lang/rust/pull/43640">some comments to Doc comments</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> improved <a href="https://github.com/rust-lang/rust/pull/43442">field does not exist error: note fields if Levenshtein suggestion fails</a> and added <a href="https://github.com/rust-lang/rust/pull/43519">a couple more error explanations for posterity</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> updated <a href="https://github.com/rust-lang/rust/pull/43618">nomicon</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made <a href="https://github.com/rust-lang/rust/pull/43409">into_iter doc example more concise</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> documented <a href="https://github.com/rust-lang/rust/pull/43563">the <code class="highlighter-rouge">from_str_radix</code> panic</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> throw <a href="https://github.com/rust-lang/rust/pull/43009">errors when doc comments are added where they’re unused</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 9th of August 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>