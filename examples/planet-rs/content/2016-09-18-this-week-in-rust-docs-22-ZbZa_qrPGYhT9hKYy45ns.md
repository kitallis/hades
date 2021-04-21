+++
title = "This Week in Rust Docs 22"
date = "2016-09-18T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-22"
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

<p>The <a href="https://github.com/rust-lang/rust/pull/36334">“new” run button</a> has been merged and is now used in <a href="https://doc.rust-lang.org/nightly/std/">nightly docs</a>. Give it a try!</p>

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
  <li><a href="https://github.com/christopherdumas">@christopherdumas</a> updated <a href="https://github.com/rust-lang/rust/pull/36404"><code class="highlighter-rouge">includes!</code> documentation</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> links between format_args! macro and std::fmt::Arguments struct](https://github.com/rust-lang/rust/pull/36523).</li>
  <li><a href="https://github.com/EdorianDark">@EdorianDark</a> inserted <a href="https://github.com/rust-lang/rust/pull/36248">examples with universal function call syntax</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/36390">basic doc examples for <code class="highlighter-rouge">std::panic::{set_hook, take_hook}</code></a> and added <a href="https://github.com/rust-lang/rust/pull/36390">basic doc example for <code class="highlighter-rouge">std::panic::set_hook</code></a>.</li>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> added <a href="https://github.com/rust-lang/rust/pull/35880">links to interesting items in <code class="highlighter-rouge">std::ptr</code> documentation</a>, <a href="https://github.com/rust-lang/rust/pull/35759">refactored range examples</a> and added <a href="https://github.com/rust-lang/rust/pull/36364"><code class="highlighter-rouge">default</code> docstrings for <code class="highlighter-rouge">String</code>, <code class="highlighter-rouge">AtomicBool</code>, and <code class="highlighter-rouge">Generics</code></a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">reference isn’t normative</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>, added <a href="https://github.com/rust-lang/rust/pull/36102">metadata diagnostics</a>, fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week (too), I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/jfirebaugh">@jfirebaugh</a>: <a href="https://github.com/rust-lang/rust/pull/36389">E0297</a></li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a>: <a href="https://github.com/rust-lang/rust/pull/36383">E0049</a></li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a>: <a href="https://github.com/rust-lang/rust/pull/36354">E0537, E0535 and E0536</a></li>
</ul>

<p>Other contributions:</p>

<ul>
  <li><a href="https://github.com/kmcallister">@kmcallister</a> tweaked <a href="https://github.com/rust-lang/rust/pull/36357">std::mem docs</a>, tweaked <a href="https://github.com/rust-lang/rust/pull/36402">array docs</a> and tweaked <a href="https://github.com/rust-lang/rust/pull/36424">std::marker docs</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> fixed <a href="https://github.com/rust-lang/rust/pull/36521">language in documentation comment</a>, added <a href="https://github.com/rust-lang/rust/pull/36519">example in AsMut trait documentation</a>.</li>
  <li><a href="https://github.com/kylog">@kylog</a> fixed <a href="https://github.com/rust-lang/rust/pull/36380">a typo in The Book</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> improved <a href="https://github.com/rust-lang/rust/pull/36480">wording</a>.</li>
  <li><a href="https://github.com/athulappadan">@athulappadan</a> added <a href="https://github.com/rust-lang/rust/pull/36396">Documentation about what Default does for each type</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> removed <a href="https://github.com/rust-lang/rust/pull/36293">the <code class="highlighter-rouge">docblock-short</code> collapse</a>.</li>
  <li><a href="https://github.com/kylog">@kylog</a> fixed <a href="https://github.com/rust-lang/rust/pull/36380">a typo in The Book</a>.</li>
  <li><a href="https://github.com/dangcheng">@dangcheng</a> fixed <a href="https://github.com/rust-lang/rust/pull/36374">a mistake (File::open -&gt; File::create) in The Book</a>.</li>
  <li><a href="https://github.com/c4rlo">@c4rlo</a> fixed <a href="https://github.com/rust-lang/rust/pull/36204">a “\” in a table heading to be “/”</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36363">urls when they were needed</a> and set <a href="https://github.com/rust-lang/rust/pull/36334">run button transparent instead of invisible</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 21st of September 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>