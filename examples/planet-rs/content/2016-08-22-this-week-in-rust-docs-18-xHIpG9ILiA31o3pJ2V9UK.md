+++
title = "This Week in Rust Docs 18"
date = "2016-08-22T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-18"
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

<h1 id="latest-news">Latest news</h1>

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
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> improved <a href="https://github.com/rust-lang/rust/pull/35810">documentation for <code class="highlighter-rouge">Fn*</code> traits</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35809"><code class="highlighter-rouge">BitAnd</code> example with something more evocative of bitwise AND</a>, demonstrated that <a href="https://github.com/rust-lang/rust/pull/35793"><code class="highlighter-rouge">RHS != Self</code> use cases for <code class="highlighter-rouge">Add</code> and <code class="highlighter-rouge">Sub</code></a>, added <a href="https://github.com/rust-lang/rust/pull/35863">evocative examples for <code class="highlighter-rouge">Shl</code> and <code class="highlighter-rouge">Shr</code></a>, replaced <a href="https://github.com/rust-lang/rust/pull/35861"><code class="highlighter-rouge">Rem</code> example with something more evocative</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35860"><code class="highlighter-rouge">Mul</code> example with something more evocative of multiplication</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35864"><code class="highlighter-rouge">Index</code> example with something more evocative of indexing</a>, made <a href="https://github.com/rust-lang/rust/pull/35876">more evocative examples for <code class="highlighter-rouge">Sub</code> and <code class="highlighter-rouge">SubAssign</code></a>, replaced <a href="https://github.com/rust-lang/rust/pull/35878"><code class="highlighter-rouge">println!</code> statements with <code class="highlighter-rouge">assert!</code>ions in <code class="highlighter-rouge">std::ptr</code> examples</a>, added <a href="https://github.com/rust-lang/rust/pull/35881">example for <code class="highlighter-rouge">Rc::would_unwrap</code></a>, added <a href="https://github.com/rust-lang/rust/pull/35880">links to interesting items in <code class="highlighter-rouge">std::ptr</code> documentation </a> and added <a href="https://github.com/rust-lang/rust/pull/35890">a panic example to std::from_utf8_unchecked</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> fixed <a href="https://github.com/rust-lang/rust/pull/35895">documentation in cell mod</a>.</li>
  <li><a href="https://github.com/CryZe">@CryZe</a> fixed <a href="https://github.com/rust-lang/rust/pull/35879">“Furthermore” Typo in String Docs</a>.</li>
  <li><a href="https://github.com/alevy">@alevy</a> fixed <a href="https://github.com/rust-lang/rust/pull/35889">a minor typo in CONTRIBUTING.md</a>.</li>
  <li><a href="https://github.com/munyari">@munyari</a> added <a href="https://github.com/rust-lang/rust/pull/35891">reference to <code class="highlighter-rouge">Self</code> in traits chapter (book)</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/35845">where <code class="highlighter-rouge">core::result::IntoIter</code> is created</a> and made <a href="https://github.com/rust-lang/rust/pull/35867">various refactorings in the rustdoc module</a>.</li>
  <li><a href="https://github.com/Stebalien">@Stebalien</a> clarified/fixed <a href="https://github.com/rust-lang/rust/pull/35862">formatting docs concerning fmt::Result/fmt::Error</a>.</li>
  <li><a href="https://github.com/jhod0">@jhod0</a> added <a href="https://github.com/rust-lang/rust/pull/34970">diagnostics for rustc_metadata</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for misplaced doc comments</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/35003">item types from some title pages from rustdoc</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">the reference isn’t normative</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/35786">Path and PathBuf docs</a>, added <a href="https://github.com/rust-lang/rust/pull/35824">new error code tests</a> and fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week (too), I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/trixnz">@trixnz</a>: <a href="https://github.com/rust-lang/rust/pull/35831">E0428</a></li>
  <li><a href="https://github.com/mlayne">@mlayne</a>: <a href="https://github.com/rust-lang/rust/pull/35812">E0232</a></li>
  <li><a href="https://github.com/pliniker">@pliniker</a>: <a href="https://github.com/rust-lang/rust/pull/35804">E0084</a></li>
  <li><a href="https://github.com/clementmiao">@clementmiao</a>: <a href="https://github.com/rust-lang/rust/pull/35780">E0396</a>, <a href="https://github.com/rust-lang/rust/pull/35778">E0395</a></li>
  <li><a href="https://github.com/KiChjang">@KiChjang</a>: <a href="https://github.com/rust-lang/rust/pull/35765">E0053</a></li>
  <li><a href="https://github.com/crypto-universe">@crypto-universe</a>: <a href="https://github.com/rust-lang/rust/pull/35756">E0407</a></li>
  <li><a href="https://github.com/DevShep">@DevShep</a>: <a href="https://github.com/rust-lang/rust/pull/35744">E0009</a></li>
  <li><a href="https://github.com/circuitfox">@circuitfox</a>: <a href="https://github.com/rust-lang/rust/pull/35739">E0403</a></li>
  <li><a href="https://github.com/pythoneer">@pythoneer</a>: <a href="https://github.com/rust-lang/rust/pull/35731">E0005</a></li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a>: <a href="https://github.com/rust-lang/rust/pull/35726">E0409</a>, <a href="https://github.com/rust-lang/rust/pull/35686">E0375</a></li>
  <li><a href="https://github.com/knight42">@knight42</a>: <a href="https://github.com/rust-lang/rust/pull/35722">E0394 and E0422</a></li>
  <li><a href="https://github.com/yossi-k">@yossi-k</a>: <a href="https://github.com/rust-lang/rust/pull/35672">E0322</a></li>
  <li><a href="https://github.com/canaltinova">@canaltinova</a>: <a href="https://github.com/rust-lang/rust/pull/35671">E0392</a></li>
  <li><a href="https://github.com/RockyTV">@RockyTV</a>: <a href="https://github.com/rust-lang/rust/pull/35670">E0365</a></li>
</ul>

<p>Others contributions:</p>

<ul>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> replaced <a href="https://github.com/rust-lang/rust/pull/35830"><code class="highlighter-rouge">Neg</code> example with something more evocative of negation</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35827"><code class="highlighter-rouge">Not</code> example with something more evocative</a>, replaced <a href="https://github.com/rust-lang/rust/pull/35806"><code class="highlighter-rouge">AddAssign</code> example with something more evocative of addition</a>, demonstrated <a href="https://github.com/rust-lang/rust/pull/35800"><code class="highlighter-rouge">RHS != Self</code> use cases for <code class="highlighter-rouge">Mul</code> and <code class="highlighter-rouge">Div</code></a>, replaced <a href="https://github.com/rust-lang/rust/pull/35709"><code class="highlighter-rouge">Add</code> example with something more evocative of addition</a>.</li>
  <li><a href="https://github.com/terrynsun">@terrynsun</a> updated <a href="https://github.com/rust-lang/rust/pull/35660">E0207 label to report parameter type</a>.</li>
  <li><a href="https://github.com/CryZe">@CryZe</a> improved <a href="https://github.com/rust-lang/rust/pull/35663"><code class="highlighter-rouge">No stdlib</code> and related documentation</a>.</li>
  <li><a href="https://github.com/crypto-universe">@crypto-universe</a> updated <a href="https://github.com/rust-lang/rust/pull/35770">test for E0221</a>.</li>
  <li><a href="https://github.com/ErikUggeldahl">@ErikUggeldahl</a> fixed <a href="https://github.com/rust-lang/rust/pull/35781">very minor spelling typo in The Book</a>.</li>
  <li><a href="https://github.com/cantino">@cantino</a> fixed <a href="https://github.com/rust-lang/rust/pull/35794">minor typo</a>.</li>
  <li><a href="https://github.com/wdv4758h">@wdv4758h</a> fixed <a href="https://github.com/rust-lang/rust/pull/35818">incorrect label messages for missing unsafe blocks (E0133)</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/35775">a few doc examples for <code class="highlighter-rouge">std::ffi::OsStr</code></a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/35655">a couple of issues with search results in rustdoc</a>.</li>
  <li><a href="https://github.com/jonathandturner">@jonathandturner</a> fixed <a href="https://github.com/rust-lang/rust/pull/35839">wording in error messages</a> and moved <a href="https://github.com/rust-lang/rust/pull/35732">‘doesn’t live long enough’ errors to labels</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/35680">new error code tests</a> and fixed <a href="https://github.com/rust-lang/rust/pull/35749">issue #11004</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 24th of August 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>