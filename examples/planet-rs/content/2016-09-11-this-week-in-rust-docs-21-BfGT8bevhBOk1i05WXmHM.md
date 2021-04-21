+++
title = "This Week in Rust Docs 21"
date = "2016-09-11T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-21"
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

<p>In the last meetup, we discussed about the possibilty to “expand” examples in the doc in order to show the full code. The goal is to prevent issues from copy/paste where <code class="highlighter-rouge">try</code>/<code class="highlighter-rouge">?</code> are used.</p>

<p>Another thing was that it should be more obvious that the code examples could actually be run online. To this issue, a PR has already be opened <a href="https://github.com/rust-lang/rust/pull/36334">here</a>.</p>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/35233">Error code list which need to be updated to new format</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/athulappadan">@athulappadan</a> added <a href="https://github.com/rust-lang/rust/pull/36396">Documentation about what Default does for each type</a>.</li>
  <li><a href="https://github.com/EdorianDark">@EdorianDark</a> inserted <a href="https://github.com/rust-lang/rust/pull/36248">examples with universal function call syntax</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> removed <a href="https://github.com/rust-lang/rust/pull/36293">the <code class="highlighter-rouge">docblock-short</code> collapse</a>.</li>
  <li><a href="https://github.com/kylog">@kylog</a> fixed <a href="https://github.com/rust-lang/rust/pull/36380">a typo in The Book</a>.</li>
  <li><a href="https://github.com/dangcheng">@dangcheng</a> fixed <a href="https://github.com/rust-lang/rust/pull/36374">a mistake (File::open -&gt; File::create) in The Book</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/36390">basic doc example for <code class="highlighter-rouge">std::panic::set_hook</code></a>.</li>
  <li><a href="https://github.com/c4rlo">@c4rlo</a> fixed <a href="https://github.com/rust-lang/rust/pull/36204">a “\” in a table heading to be “/”</a>.</li>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> added <a href="https://github.com/rust-lang/rust/pull/35880">links to interesting items in <code class="highlighter-rouge">std::ptr</code> documentation</a> and <a href="https://github.com/rust-lang/rust/pull/35759">refactored range examples</a>, added <a href="https://github.com/rust-lang/rust/pull/36364"><code class="highlighter-rouge">default</code> docstrings for <code class="highlighter-rouge">String</code>, <code class="highlighter-rouge">AtomicBool</code>, and <code class="highlighter-rouge">Generics</code></a> and updated <a href="https://github.com/rust-lang/rust/pull/36366">docstrings for <code class="highlighter-rouge">mem::update</code> and <code class="highlighter-rouge">mem::swap</code></a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">reference isn’t normative</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36102">metadata diagnostics</a>, added <a href="https://github.com/rust-lang/rust/pull/36363">urls when they were needed</a>, fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a> and set <a href="https://github.com/rust-lang/rust/pull/36334">run button transparent instead of invisible</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week (too), I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/Cobrand">@Cobrand</a>: <a href="https://github.com/rust-lang/rust/pull/36267">E0559</a></li>
  <li><a href="https://github.com/abhiQmar">@abhiQmar</a>: <a href="https://github.com/rust-lang/rust/pull/36223">E0558</a></li>
  <li><a href="https://github.com/razielgn">@razielgn</a>: <a href="https://github.com/rust-lang/rust/pull/36212">E0493</a></li>
  <li><a href="https://github.com/EugeneGonzalez">@EugeneGonzalez</a>: <a href="https://github.com/rust-lang/rust/pull/36210">E0529</a></li>
</ul>

<p>Other contributions:</p>

<ul>
  <li><a href="https://github.com/JDemler">@JDemler</a> fixed <a href="https://github.com/rust-lang/rust/pull/36326">typo in nomicon</a>.</li>
  <li><a href="https://github.com/fanzier">@fanzier</a> fixed <a href="https://github.com/rust-lang/rust/pull/36165">typo in PartialOrd docs</a>.</li>
  <li><a href="https://github.com/johnthagen">@johnthagen</a> updated <a href="https://github.com/rust-lang/rust/pull/36225">nightly docs supported Windows versions to match Getting Started page</a>.</li>
  <li><a href="https://github.com/Sawyer47">@Sawyer47</a> removed <a href="https://github.com/rust-lang/rust/pull/36266">incorrect methods inherited through Deref by filtering them</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/36314">doc coercion</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/35845">where <code class="highlighter-rouge">core::result::IntoIter</code> is created</a> and added <a href="https://github.com/rust-lang/rust/pull/36311">doc example for <code class="highlighter-rouge">std::time::Instant::elapsed</code></a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/36078">associated consts in search results</a>.</li>
  <li><a href="https://github.com/Cobrand">@Cobrand</a> added <a href="https://github.com/rust-lang/rust/pull/36241">mention of <code class="highlighter-rouge">make tidy</code></a> and added <a href="https://github.com/rust-lang/rust/pull/36241">mention of <code class="highlighter-rouge">make tidy</code> into contributing.md file</a>.</li>
  <li><a href="https://github.com/apasel422">@apasel422</a> cleaned up <a href="https://github.com/rust-lang/rust/pull/36263">thread-local storage docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36243">missing urls</a>, added <a href="https://github.com/rust-lang/rust/pull/36141">new error code tests</a>, added missing urls <a href="https://github.com/rust-lang/rust/pull/36298">here</a> and <a href="https://github.com/rust-lang/rust/pull/36243">here</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 14th of September 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>