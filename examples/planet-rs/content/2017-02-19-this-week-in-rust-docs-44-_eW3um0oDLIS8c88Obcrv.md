+++
title = "This Week in Rust Docs 44"
date = "2017-02-19T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-44"
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

<p>The <code class="highlighter-rouge">rustdoc --test</code> output has been updated and merged! Now it looks like this:</p>

<div class="highlighter-rouge"><div class="highlight"><pre class="highlight"><code>&gt; rustdoc --test some_file.rs
the/path/some_file.rs - Foo::foo (line 43) ... ok
the/path/some_file.rs - Foo::bar (line 79) ... ok
</code></pre></div></div>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

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
  <li><a href="https://github.com/mp4096">@mp4096</a> wrote <a href="https://github.com/rust-lang/rust/pull/39955">better explanation for return values for min, max functions for the Iterator trait</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> provided <a href="https://github.com/rust-lang/rust/pull/39953">suggestions for unknown macros imported with <code class="highlighter-rouge">use</code></a> and added <a href="https://github.com/rust-lang/rust/pull/39738">notes about capacity effects to Vec::truncate()</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> ported <a href="https://github.com/rust-lang/rust/pull/39855">the reference to mdbook</a> and create <a href="https://github.com/rust-lang/rust/pull/39866">the Unstable Book</a>.</li>
  <li><a href="https://github.com/arthurprs">@arthurprs</a> fix <a href="https://github.com/rust-lang/rust/pull/39937">spelling in hashmap comments</a>.</li>
  <li><a href="https://github.com/JDemler">@JDemler</a> added <a href="https://github.com/rust-lang/rust/pull/39845">Documentation for Custom Attributes and Error Reporting in Procedural Macros</a>.</li>
  <li><a href="https://github.com/jrmuizel">@jrmuizel</a> removed <a href="https://github.com/rust-lang/rust/pull/39304">obsolete documentation about drop-flags</a>.</li>
  <li><a href="https://github.com/mina86">@mina86</a> updated The Book: <a href="https://github.com/rust-lang/rust/pull/39777">binary prefixed are defined by IEC and not in SI</a>.</li>
  <li><a href="https://github.com/Dowwie">@Dowwie</a> updated <a href="https://github.com/rust-lang/rust/pull/39691">attributes.md - Last sentence updated to reflect support for custom attributes</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/39944">associated constant rendering in rustdoc</a>, set <a href="https://github.com/rust-lang/rust/pull/39859">rustdoc –test files’ path relative to the current directory</a>, improved <a href="https://github.com/rust-lang/rust/pull/39765">file not found for module error</a>, fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a> and improved <a href="https://github.com/rust-lang/rust/pull/39814">invalid call on non-function error message</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> improved <a href="https://github.com/rust-lang/rust/pull/39760">grammar on field init docs</a> and removed <a href="https://github.com/rust-lang/rust/pull/39758">duplicated “parameter” in E0089 text</a>.</li>
  <li><a href="https://github.com/JordiPolo">@JordiPolo</a> substituted <a href="https://github.com/rust-lang/rust/pull/39756">try! for ? in the Result documentation</a>.</li>
  <li><a href="https://github.com/jimmycuadra">@jimmycuadra</a> included <a href="https://github.com/rust-lang/rust/pull/39740">a stability span only if needed in rustdoc</a>.</li>
  <li><a href="https://github.com/notriddle">@notriddle</a> added <a href="https://github.com/rust-lang/rust/pull/39697">the item type to the tooltip</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> showed <a href="https://github.com/rust-lang/rust/pull/39654">attributes on all item types in rustdoc</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/39862">wording in LocalKey documentation</a>.</li>
  <li><a href="https://github.com/king6cong">@king6cong</a> updated <a href="https://github.com/rust-lang/rust/pull/39844">sys/mod doc and mod import order adjust</a>, made <a href="https://github.com/rust-lang/rust/pull/39839">doc consistent with var name</a> and fixed <a href="https://github.com/rust-lang/rust/pull/39784">a typo</a>.</li>
  <li><a href="https://github.com/Stebalien">@Stebalien</a> fixed <a href="https://github.com/rust-lang/rust/pull/39904">String::split_off documentation</a>.</li>
  <li><a href="https://github.com/CBenoit">@CBenoit</a> corrected <a href="https://github.com/rust-lang/rust/pull/39847">a typo in procedural macros chapter of the Book</a>.</li>
  <li><a href="https://github.com/WRONGWAY4YOU">@WRONGWAY4YOU</a> fixed <a href="https://github.com/rust-lang/rust/pull/39846">typo</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> fixed <a href="https://github.com/rust-lang/rust/pull/39836">types in to_owned doctest</a>.</li>
  <li><a href="https://github.com/DaseinPhaos">@DaseinPhaos</a> updated <a href="https://github.com/rust-lang/rust/pull/39840">procedural-macros.md</a>.</li>
  <li><a href="https://github.com/mina86">@mina86</a> removed <a href="https://github.com/rust-lang/rust/pull/39775">GNU extensions in the example when they were unnecessary in The Book</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> ported <a href="https://github.com/rust-lang/rust/pull/39633">books to mdbook</a>.</li>
  <li><a href="https://github.com/ahmedcharles">@ahmedcharles</a> fixed <a href="https://github.com/rust-lang/rust/pull/39778">some typos in the core::fmt docs.</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39743">tested item in the rustdoc –test output</a>, added <a href="https://github.com/rust-lang/rust/pull/39928">missing urls for env functions</a> and added <a href="https://github.com/rust-lang/rust/pull/39788">filename when running rustdoc –test on a markdown file</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 22nd of February 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>