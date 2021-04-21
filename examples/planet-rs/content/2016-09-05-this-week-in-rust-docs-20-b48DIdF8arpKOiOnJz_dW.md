+++
title = "This Week in Rust Docs 20"
date = "2016-09-05T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-20"
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

<p>This week’s edition was edited by <a href="https://github.com/matthew-piziak">Matthew Piziak</a>.</p>

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

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> added <a href="https://github.com/rust-lang/rust/pull/35880">links to interesting items in <code class="highlighter-rouge">std::ptr</code> documentation</a> and <a href="https://github.com/rust-lang/rust/pull/35759">refactored range examples</a></li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/35845">where <code class="highlighter-rouge">core::result::IntoIter</code> is created</a></li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">reference isn’t normative</a></li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/36078">associated consts in search results</a></li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a> and <a href="https://github.com/rust-lang/rust/pull/36243">added missing urls</a></li>
  <li><a href="Cobrand">@Cobrand</a> added <a href="https://github.com/rust-lang/rust/pull/36241">mention of <code class="highlighter-rouge">make tidy</code></a></li>
  <li><a href="https://github.com/c4rlo">@c4rlo</a> fixed <a href="https://github.com/rust-lang/rust/pull/36204">a “\” in a table heading to be “/”</a></li>
  <li><a href="https://github.com/apasel422">@apasel422</a> cleaned up <a href="https://github.com/rust-lang/rust/pull/36263">thread-local storage docs</a></li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week (too), I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/KiChjang">@KiChjang</a>: <a href="https://github.com/rust-lang/rust/pull/35480">E0379</a></li>
  <li><a href="https://github.com/EugeneGonzalez">@EugeneGonzalez</a>: <a href="https://github.com/rust-lang/rust/pull/36205">E0528</a> and <a href="https://github.com/rust-lang/rust/pull/35773">E0259</a></li>
  <li><a href="https://github.com/abhiQmar">@abhiQmar</a>: <a href="https://github.com/rust-lang/rust/pull/36050">E0076</a> and <a href="https://github.com/rust-lang/rust/pull/36223">E0558</a></li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a>: <a href="https://github.com/rust-lang/rust/pull/36054">E0451</a> and <a href="https://github.com/rust-lang/rust/pull/36147">E0265</a></li>
  <li><a href="https://github.com/birryree">@birryree</a>: <a href="https://github.com/rust-lang/rust/pull/36148">E0194</a></li>
  <li><a href="https://github.com/paulfanelli">@paulfanelli</a>: <a href="https://github.com/rust-lang/rust/pull/36060">E0463</a></li>
  <li><a href="https://github.com/acrrd">@acrrd</a>: <a href="https://github.com/rust-lang/rust/pull/36079">E0318</a></li>
  <li><a href="https://github.com/0xmohit">@0xmohit</a>: <a href="https://github.com/rust-lang/rust/pull/36100">E0260</a>, <a href="https://github.com/rust-lang/rust/pull/36135">E0520</a></li>
  <li><a href="https://github.com/zjhmale">@zjhmale</a>: <a href="https://github.com/rust-lang/rust/pull/36114">E0393</a></li>
  <li><a href="https://github.com/gavinb">@gavinb</a>: <a href="https://github.com/rust-lang/rust/pull/36125">E0164, E0615, and E0184</a></li>
  <li><a href="https://github.com/athulappadan">@athulappadan</a>: <a href="https://github.com/rust-lang/rust/pull/36136">E0034</a></li>
</ul>

<p>Other contributions:</p>

<ul>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> improved <a href="https://github.com/rust-lang/rust/pull/35810">documentation for <code class="highlighter-rouge">Fn*</code> traits</a>, added <a href="https://github.com/rust-lang/rust/pull/35863">evocative examples for <code class="highlighter-rouge">Shl</code> and <code class="highlighter-rouge">Shr</code></a>, added <a href="https://github.com/rust-lang/rust/pull/35926">evocative examples for <code class="highlighter-rouge">BitOr</code> and <code class="highlighter-rouge">BitXor</code></a>, replaced <a href="https://github.com/rust-lang/rust/pull/35927"><code class="highlighter-rouge">BitAndAssign</code> example with something more evocative</a>, improved <a href="https://github.com/rust-lang/rust/pull/35993"><code class="highlighter-rouge">BitAnd</code> trait documentation</a>, added <a href="https://github.com/rust-lang/rust/pull/35997">a simple example for <code class="highlighter-rouge">thread::current()</code></a>, implemented <a href="https://github.com/rust-lang/rust/pull/35758">accumulate vector and assert for <code class="highlighter-rouge">RangeFrom</code> and <code class="highlighter-rouge">RangeInclusive</code> examples</a>, showed <a href="https://github.com/rust-lang/rust/pull/35771">how iterating over <code class="highlighter-rouge">RangeTo</code> and <code class="highlighter-rouge">RangeToInclusive</code> fails</a>, and <a href="https://github.com/rust-lang/rust/pull/35793">demonstrated <code class="highlighter-rouge">RHS != Self</code> use cases for <code class="highlighter-rouge">Add</code> and <code class="highlighter-rouge">Sub</code></a></li>
  <li><a href="https://github.com/F001">@F001</a> fixed <a href="https://github.com/rust-lang/rust/pull/35895">documentation in the <code class="highlighter-rouge">cell</code> module</a></li>
  <li><a href="https://github.com/regexident">@regexident</a> updated a <a href="https://github.com/rust-lang/rust/pull/35962">code sample in chapter on syntax extensions</a></li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> noticed that <a href="https://github.com/rust-lang/rust/pull/36130">rustbook chapters/sections should be ordered lists</a></li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made the <a href="https://github.com/rust-lang/rust/pull/36134"><code class="highlighter-rouge">TcpListener</code> example more simple</a></li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> updated <a href="https://github.com/rust-lang/rust/pull/36152">the man pages</a></li>
  <li><a href="https://github.com/fanzier">@fanzier</a> fixed a <a href="https://github.com/rust-lang/rust/pull/36165">typo in <code class="highlighter-rouge">PartialOrd </code> docs</a></li>
  <li><a href="https://github.com/wdv4758h">@wdv4758h</a> changed <a href="https://github.com/rust-lang/rust/pull/36169"><code class="highlighter-rouge">rustc::plugin</code> to <code class="highlighter-rouge">rustc_plugin</code> in a doc comment</a></li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> explained <a href="https://github.com/rust-lang/rust/pull/35418">why <code class="highlighter-rouge">Box</code>/<code class="highlighter-rouge">Rc</code>/<code class="highlighter-rouge">Arc</code> methods do not take <code class="highlighter-rouge">self</code></a></li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/35786"><code class="highlighter-rouge">Path</code> and <code class="highlighter-rouge">PathBuf</code> docs</a></li>
  <li><a href="https://github.com/durka">@durka</a> indicated <a href="https://github.com/rust-lang/rust/pull/36231">where to copy <code class="highlighter-rouge">config.toml.example</code></a></li>
  <li><a href="https://github.com/skade">@skade</a> documented <a href="https://github.com/rust-lang/rust/pull/36099"><code class="highlighter-rouge">try!</code>’s error conversion behavior</a></li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 7th of September 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>