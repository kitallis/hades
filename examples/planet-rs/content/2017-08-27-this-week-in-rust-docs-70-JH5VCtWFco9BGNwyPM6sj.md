+++
title = "This Week in Rust Docs 70"
date = "2017-08-27T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-70"
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
  <li><a href="https://github.com/qnighy">@qnighy</a> added <a href="https://github.com/rust-lang/rust/pull/43426">hints when intercrate ambiguity causes overlap</a>.</li>
  <li><a href="https://github.com/huntiep">@huntiep</a> improved <a href="https://github.com/rust-lang/rust/pull/43984">Try error messages</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/43849">new “Implementations on Foreign Types” section to traits in rustdoc</a> and hid <a href="https://github.com/rust-lang/rust/pull/44026">internal types/traits from std docs via new #[doc(masked)] attribute</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/44103">comparison operators to must-use lint (under <code class="highlighter-rouge">fn_must_use</code> feature)</a>.</li>
  <li><a href="https://github.com/ishitatsuyuki">@ishitatsuyuki</a> made <a href="https://github.com/rust-lang/rust/pull/42588">unused-extern-crate warn-by-default</a>.</li>
  <li><a href="https://github.com/llogiq">@llogiq</a> added <a href="https://github.com/rust-lang/rust/pull/44104">a lowercase suggestion to unknown_lints</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> extended <a href="https://github.com/rust-lang/rust/pull/44079">E0623 for LateBound and EarlyBound Regions</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/43886">clippy as a submodule</a> and suggested <a href="https://github.com/rust-lang/rust/pull/44059"><code class="highlighter-rouge">Ok(())</code> when encountering <code class="highlighter-rouge">Result::&lt;(), E&gt;::Ok()</code></a>.</li>
  <li><a href="https://github.com/mystor">@mystor</a> removed <a href="https://github.com/rust-lang/rust/pull/43918">highlight of # which does not start an attribute in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, compile <a href="https://github.com/rust-lang/rust/pull/43949">fail stable</a> and added <a href="https://github.com/rust-lang/rust/pull/43870">deref suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> improved <a href="https://github.com/rust-lang/rust/pull/43929">placement of <code class="highlighter-rouge">use</code> suggestions</a>.</li>
  <li><a href="https://github.com/ruuda">@ruuda</a> pointed <a href="https://github.com/rust-lang/rust/pull/43631">“deref coercions” links to new book</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> added <a href="https://github.com/rust-lang/rust/pull/43700">E0623 for structs</a>.</li>
  <li><a href="https://github.com/shanavas786">@shanavas786</a> fixed <a href="https://github.com/rust-lang/rust/pull/43996">typo in doc</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/43854">out missing if conditional</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> fixed <a href="https://github.com/rust-lang/rust/pull/44072">inconsistent doc headings</a>.</li>
  <li><a href="https://github.com/Jouan">@Jouan</a> added <a href="https://github.com/rust-lang/rust/pull/43979">links for impls</a>.</li>
  <li><a href="https://github.com/mattico">@mattico</a> clarified <a href="https://github.com/rust-lang/rust/pull/44043">windows build instructions in README</a>.</li>
  <li><a href="https://github.com/remexre">@remexre</a> mentionned <a href="https://github.com/rust-lang/rust/pull/44039">null_mut on the pointer primitive docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/43978">missing links in docs</a>, removed <a href="https://github.com/rust-lang/rust/pull/43977">outline when details have focus</a>, removed <a href="https://github.com/rust-lang/rust/pull/43966">duplicates in rustdoc</a>, add <a href="https://github.com/rust-lang/rust/pull/44090">missing link in string doc</a>, add <a href="https://github.com/rust-lang/rust/pull/44010">missing links for Read trait</a> and rollup <a href="https://github.com/rust-lang/rust/pull/44033">of 5 pull requests</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 30th of August 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>