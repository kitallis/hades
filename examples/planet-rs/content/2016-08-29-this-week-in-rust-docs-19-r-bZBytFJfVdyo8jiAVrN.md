+++
title = "This Week in Rust Docs 19"
date = "2016-08-29T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-19"
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

<p>This week’s edition was edited by: <a href="https://github.com/GuillaumeGomez">GuillaumeGomez</a>.</p>

<h1 id="latest-news-getting-a-bit-old-but-still-gold">Latest news (getting a bit old but still gold!)</h1>

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

<p>There are currently around 50 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on github!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> improved <a href="https://github.com/rust-lang/rust/pull/35810">documentation for <code class="highlighter-rouge">Fn*</code> traits</a>, demonstrated that <a href="https://github.com/rust-lang/rust/pull/35793"><code class="highlighter-rouge">RHS != Self</code> use cases for <code class="highlighter-rouge">Add</code> and <code class="highlighter-rouge">Sub</code></a>, added <a href="https://github.com/rust-lang/rust/pull/35863">evocative examples for <code class="highlighter-rouge">Shl</code> and <code class="highlighter-rouge">Shr</code></a>, added <a href="https://github.com/rust-lang/rust/pull/35880">links to interesting items in <code class="highlighter-rouge">std::ptr</code> documentation </a> and added <a href="https://github.com/rust-lang/rust/pull/35890">a panic example to std::from_utf8_unchecked</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> fixed <a href="https://github.com/rust-lang/rust/pull/35895">documentation in cell mod</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/35845">where <code class="highlighter-rouge">core::result::IntoIter</code> is created</a>.</li>
  <li><a href="https://github.com/Stebalien">@Stebalien</a> clarified/fixed <a href="https://github.com/rust-lang/rust/pull/35862">formatting docs concerning fmt::Result/fmt::Error</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">the reference isn’t normative</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/36078">associated consts in search results in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36083">missing urls into convert module</a>, improved <a href="https://github.com/rust-lang/rust/pull/35786">Path and PathBuf docs</a> and fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week (too), I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/crypto-universe">@crypto-universe</a>: <a href="https://github.com/rust-lang/rust/pull/35835">E0426</a>, <a href="https://github.com/rust-lang/rust/pull/35835">E0426</a></li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a>: <a href="https://github.com/rust-lang/rust/pull/36044">E0450</a></li>
  <li><a href="https://github.com/0xmohit">@0xmohit</a>: <a href="https://github.com/rust-lang/rust/pull/35989">E0453</a>, <a href="https://github.com/rust-lang/rust/pull/35985">E0277</a>, <a href="https://github.com/rust-lang/rust/pull/35961">E0445 and E0454</a></li>
  <li><a href="https://github.com/creativcoder">@creativcoder</a>: <a href="https://github.com/rust-lang/rust/pull/35939">E0195</a></li>
  <li><a href="https://github.com/shyaamsundhar">@shyaamsundhar</a>: <a href="https://github.com/rust-lang/rust/pull/35858">E0435, E0437 and E0438</a></li>
  <li><a href="https://github.com/kyrias">@kyrias</a>: <a href="https://github.com/rust-lang/rust/pull/35841">E0424</a></li>
</ul>

<p>Others contributions:</p>

<ul>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> replaced <a href="https://github.com/rust-lang/rust/pull/35936"><code class="highlighter-rouge">Div</code> example with something more evocative of division</a>, added <a href="https://github.com/rust-lang/rust/pull/35881">example for <code class="highlighter-rouge">Rc::would_unwrap</code></a>, replaced <a href="https://github.com/rust-lang/rust/pull/35878"><code class="highlighter-rouge">println!</code> statements with <code class="highlighter-rouge">assert!</code>ions in <code class="highlighter-rouge">std::ptr</code> examples</a>, added <a href="https://github.com/rust-lang/rust/pull/35876">more evocative examples for <code class="highlighter-rouge">Sub</code> and <code class="highlighter-rouge">SubAssign</code></a>, replaced <a href="https://github.com/rust-lang/rust/pull/35864"><code class="highlighter-rouge">Index</code> example with something more evocative of indexing</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35861"><code class="highlighter-rouge">Rem</code> example with something more evocative</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35860"><code class="highlighter-rouge">Mul</code> example with something more evocative of multiplication</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35809"><code class="highlighter-rouge">BitAnd</code> example with something more evocative of bitwise AND</a></li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for misplaced doc comments</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> made <a href="https://github.com/rust-lang/rust/pull/35867">various refactorings in the rustdoc module</a>.</li>
  <li><a href="https://github.com/CryZe">@CryZe</a> fixed <a href="https://github.com/rust-lang/rust/pull/35879">“Furthermore” Typo in String Docs</a>.</li>
  <li><a href="https://github.com/alevy">@alevy</a> fixed <a href="https://github.com/rust-lang/rust/pull/35889">a minor typo in CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/munyari">@munyari</a> added <a href="https://github.com/rust-lang/rust/pull/35891">reference to <code class="highlighter-rouge">Self</code> in traits chapter in The Book</a>.</li>
  <li><a href="https://github.com/kyrias">@kyrias</a> improved <a href="https://github.com/rust-lang/rust/pull/35980">E0094 underline</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> added <a href="https://github.com/rust-lang/rust/pull/35948">trailing commas</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added new error code tests<a href="https://github.com/rust-lang/rust/pull/36003">here</a> and <a href="https://github.com/rust-lang/rust/pull/35920">here</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 31st of August 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>