+++
title = "This Week in Rust Docs 80"
date = "2017-11-05T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-80"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a>, showed <a href="https://github.com/rust-lang/rust/pull/45039">in docs whether the return type of a function impls Iterator/Read/Write</a>, added <a href="https://github.com/rust-lang/rust/pull/45764">#[allow(unused)] to every doctest in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/45767">talk about #![doc(test(no_crate_inject))] and #![doc(test(attr(…)))] in rustdoc</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/45454">Release notes for 1.22.0</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45776">code on diagnostics when underlined</a> and highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> updated <a href="https://github.com/rust-lang/rust/pull/45778">reference link in doc’s 404</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> fixed <a href="https://github.com/rust-lang/rust/pull/45756">typos in README.md</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> started <a href="https://github.com/rust-lang/rust/pull/45692">shipping the Cargo book</a>.</li>
  <li><a href="https://github.com/sdroege">@sdroege</a> updated <a href="https://github.com/rust-lang/rust/pull/45714">the std::thread docs and clarified that panics can nowadays be caught</a>.</li>
  <li><a href="https://github.com/fhartwig">@fhartwig</a> made <a href="https://github.com/rust-lang/rust/pull/45645">rustdoc not include self-by-value methods from Deref target</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/45620">duplicated impls with generics in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/45582">missing links and examples</a>, added <a href="https://github.com/rust-lang/rust/pull/45470">missing docs for MetadataExt</a>, added <a href="https://github.com/rust-lang/rust/pull/45766">more elements in the sidebar</a>, added <a href="https://github.com/rust-lang/rust/pull/45673">search over generic types in docs</a> and added <a href="https://github.com/rust-lang/rust/pull/45631">missing links and examples for FileExt</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/joshleeb">@joshleeb</a> fixed <a href="https://github.com/rust-lang/rust/pull/45603">duplicated display of error E0502</a> and improved <a href="https://github.com/rust-lang/rust/pull/45630">display of error E0308</a>.</li>
  <li><a href="https://github.com/leodasvacas">@leodasvacas</a> documented <a href="https://github.com/rust-lang/rust/pull/45579">that call expressions also represent ADT constructors</a>.</li>
  <li><a href="https://github.com/tirr-c">@tirr-c</a> displayed <a href="https://github.com/rust-lang/rust/pull/45711">spans correctly when there are zero-width or wide characters</a>.</li>
  <li><a href="https://github.com/Ljzn">@Ljzn</a> fixed <a href="https://github.com/rust-lang/rust/pull/45718">typo</a> and fixed <a href="https://github.com/rust-lang/rust/pull/45681">typo</a>.</li>
  <li><a href="https://github.com/LaurentMazare">@LaurentMazare</a> added <a href="https://github.com/rust-lang/rust/pull/45639">a nicer error message for missing in for loop, fixes #40782</a>.</li>
  <li><a href="https://github.com/Cldfire">@Cldfire</a> suggested <a href="https://github.com/rust-lang/rust/pull/45660">renaming import if names clash</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> fixed <a href="https://github.com/rust-lang/rust/pull/45664">incorrect error type in Read::byte docs</a>.</li>
  <li><a href="https://github.com/carols10cents">@carols10cents</a> updated <a href="https://github.com/rust-lang/rust/pull/45554">the book for a fix to the print button</a>.</li>
  <li><a href="https://github.com/Technius">@Technius</a> improved <a href="https://github.com/rust-lang/rust/pull/45295">std::process module docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/45187">sidebar rendering and added methods list</a>, fixed <a href="https://github.com/rust-lang/rust/pull/45450">title heading overlap in rust doc</a>, search <a href="https://github.com/rust-lang/rust/pull/45617">fixes</a> and added <a href="https://github.com/rust-lang/rust/pull/45746">tests for methods listing in rust docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 7th of November 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>