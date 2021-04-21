+++
title = "This Week in Rust Docs 68"
date = "2017-08-13T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-68"
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
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/43641">lint casting signed integers smaller than <code class="highlighter-rouge">isize</code> to raw pointers</a>.</li>
  <li><a href="https://github.com/ruuda">@ruuda</a> pointed <a href="https://github.com/rust-lang/rust/pull/43631">“deref coercions” links to new book</a>.</li>
  <li><a href="https://github.com/nrc">@nrc</a> fixed <a href="https://github.com/rust-lang/rust/pull/43782">include! in doc tests</a>.</li>
  <li><a href="https://github.com/pengowen123">@pengowen123</a> fixed <a href="https://github.com/rust-lang/rust/pull/43813">unused_result lint triggering when a function returns <code class="highlighter-rouge">()</code>, <code class="highlighter-rouge">!</code> or an empty enum</a>.</li>
  <li><a href="https://github.com/Eijebong">@Eijebong</a> fixed <a href="https://github.com/rust-lang/rust/pull/43814">some typos</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/43819">doc examples for <code class="highlighter-rouge">include*</code> macros.</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> added <a href="https://github.com/rust-lang/rust/pull/43700">E0623 for structs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> wrote <a href="https://github.com/rust-lang/rust/pull/43790">the “passes” chapter of the rustdoc book</a>.</li>
  <li><a href="https://github.com/mattico">@mattico</a> added <a href="https://github.com/rust-lang/rust/pull/43339">lint for int to ptr cast</a>.</li>
  <li><a href="https://github.com/qnighy">@qnighy</a> added <a href="https://github.com/rust-lang/rust/pull/43426">hints when intercrate ambiguity causes overlap.</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, removed <a href="https://github.com/rust-lang/rust/pull/43829">suggest methods in certain cases</a> and added <a href="https://github.com/rust-lang/rust/pull/43803">missing links doc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> clarified <a href="https://github.com/rust-lang/rust/pull/43176">wording for E0122</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> exposed <a href="https://github.com/rust-lang/rust/pull/43348">all OS-specific modules in libstd doc</a> and type-checked <a href="https://github.com/rust-lang/rust/pull/43745"><code class="highlighter-rouge">break value;</code> even outside of <code class="highlighter-rouge">loop {}</code></a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/43484">at return type always when type mismatch against it</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/43627">release notes for 1.20</a>.</li>
  <li><a href="https://github.com/ruuda">@ruuda</a> detected <a href="https://github.com/rust-lang/rust/pull/43632">relative urls in tidy check</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> de-orphaned <a href="https://github.com/rust-lang/rust/pull/43709">extended information</a>, added <a href="https://github.com/rust-lang/rust/pull/43728"><code class="highlighter-rouge">#[must_use]</code> for functions</a> and made the <a href="https://github.com/rust-lang/rust/pull/43726">e05XX odyssey</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/43736">external impls to implementors js in rustdoc</a>, stopped <a href="https://github.com/rust-lang/rust/pull/43715">using URL shortener in docs</a> and fixed <a href="https://github.com/rust-lang/rust/pull/43760">broken CSS in search results in rustdoc</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> improved <a href="https://github.com/rust-lang/rust/pull/43724">std::ops docs</a>.</li>
  <li><a href="https://github.com/tchajed">@tchajed</a> updated <a href="https://github.com/rust-lang/rust/pull/43823">GitHub pull request documentation link</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> added <a href="https://github.com/rust-lang/rust/pull/43812">doc tests in rustdoc</a> and documented <a href="https://github.com/rust-lang/rust/pull/43792">the doc attribute</a>.</li>
  <li><a href="https://github.com/Eijebong">@Eijebong</a> fixed <a href="https://github.com/rust-lang/rust/pull/43794">some typos</a>.</li>
  <li><a href="https://github.com/natboehm">@natboehm</a> provided <a href="https://github.com/rust-lang/rust/pull/43721">more explanation for Deref in String docs</a>.</li>
  <li><a href="https://github.com/j-browne">@j-browne</a> fixed <a href="https://github.com/rust-lang/rust/pull/43793">broken links in Arc documentation</a>.</li>
  <li><a href="https://github.com/prisme60">@prisme60</a> fixed <a href="https://github.com/rust-lang/rust/pull/43783">typo corersponding -&gt; corresponding</a>.</li>
  <li><a href="https://github.com/mattico">@mattico</a> fixed <a href="https://github.com/rust-lang/rust/pull/43779">typo in unicode char definition</a>.</li>
  <li><a href="https://github.com/ubsan">@ubsan</a> fixed <a href="https://github.com/rust-lang/rust/pull/43773">a typo</a>.</li>
  <li><a href="https://github.com/pornel">@pornel</a> corrected <a href="https://github.com/rust-lang/rust/pull/43720">the extern constant syntax in hint</a>.</li>
  <li><a href="https://github.com/ivanbakel">@ivanbakel</a> fixed <a href="https://github.com/rust-lang/rust/pull/43582">mutable vars being marked used when they weren’t</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/43397">warn on unused field on union</a>, fixed <a href="https://github.com/rust-lang/rust/pull/43691">rustdoc error on ‘\0’</a>, added <a href="https://github.com/rust-lang/rust/pull/43558">union and const colors</a>, improved <a href="https://github.com/rust-lang/rust/pull/43747">headers linking</a>, improved <a href="https://github.com/rust-lang/rust/pull/43791">file docs</a>, improved <a href="https://github.com/rust-lang/rust/pull/43795">enum variants display in rustdoc</a>, improved <a href="https://github.com/rust-lang/rust/pull/43737">error message when duplicate names for type and trait method</a> and added <a href="https://github.com/rust-lang/rust/pull/43699">missing error code for private method</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 16th of August 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>