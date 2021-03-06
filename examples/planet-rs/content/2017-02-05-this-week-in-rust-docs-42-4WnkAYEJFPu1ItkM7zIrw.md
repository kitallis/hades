+++
title = "This Week in Rust Docs 42"
date = "2017-02-05T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-42"
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
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> improved <a href="https://github.com/rust-lang/rust/pull/39554">error message when two-arg assert_eq! receives a trailing comma</a> and used <a href="https://github.com/rust-lang/rust/pull/39441">help rather than span note for no method error; a slight rephrasing</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> gave <a href="https://github.com/rust-lang/rust/pull/39444">a better error message for unknown derive messages</a>.</li>
  <li><a href="https://github.com/APTy">@APTy</a> added <a href="https://github.com/rust-lang/rust/pull/39007">docs and tests for join_multicast_{v4,v6}</a>.</li>
  <li><a href="https://github.com/cengizIO">@cengizIO</a> improved <a href="https://github.com/rust-lang/rust/pull/39361">error message for uninferrable types #38812</a>.</li>
  <li><a href="https://github.com/JordiPolo">@JordiPolo</a> changed <a href="https://github.com/rust-lang/rust/pull/39545">deprecation warning to indicate custom derive support was removed</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> fixed <a href="https://github.com/rust-lang/rust/pull/39312">full path being output with <code class="highlighter-rouge">rustdoc -h</code></a>.</li>
  <li><a href="https://github.com/rspeer">@rspeer</a> fixed <a href="https://github.com/rust-lang/rust/pull/39174">a misleading statement in <code class="highlighter-rouge">Iterator.nth()</code></a>.</li>
  <li><a href="https://github.com/bluecereal">@bluecereal</a> updated <a href="https://github.com/rust-lang/rust/pull/38436">if-let.md</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> distinguished <a href="https://github.com/rust-lang/rust/pull/39458">guesses from suggestions</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/39300">code in <code class="highlighter-rouge">rustc --explain</code></a>, pointed <a href="https://github.com/rust-lang/rust/pull/39202">to enclosing block/fn on nested unsafe</a> and detected <a href="https://github.com/rust-lang/rust/pull/39231">missing <code class="highlighter-rouge">;</code> on methods with return type <code class="highlighter-rouge">()</code></a>.</li>
  <li><a href="https://github.com/phungleson">@phungleson</a> fixed <a href="https://github.com/rust-lang/rust/pull/39459">short hand struct doc</a> and removed <a href="https://github.com/rust-lang/rust/pull/39443">suggestions to use things which weren’t found either</a>.</li>
  <li><a href="https://github.com/chriskrycho">@chriskrycho</a> documented <a href="https://github.com/rust-lang/rust/pull/37928">RFC 1623: static lifetime elision.</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> removed <a href="https://github.com/rust-lang/rust/pull/39374">lie about #[cfg] from reference</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/39002">Debug implementations for libcollection structs</a> and added <a href="https://github.com/rust-lang/rust/pull/39513">missing urls and small nits</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/Wilfred">@Wilfred</a> made a <a href="https://github.com/rust-lang/rust/pull/39389">minor grammar fix ‘can not’ -&gt; ‘cannot’</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> added <a href="https://github.com/rust-lang/rust/pull/39116">clearer error message using <code class="highlighter-rouge">&amp;str + &amp;str</code></a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added a <a href="https://github.com/rust-lang/rust/pull/38103">note for individual lint name in messages set via lint group attribute</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> clarified <a href="https://github.com/rust-lang/rust/pull/39299">the lack of mutability inside an Rc in std:rc</a>.</li>
  <li><a href="https://github.com/alexcrichton">@alexcrichton</a> suppressed <a href="https://github.com/rust-lang/rust/pull/39354">warnings/errors with rustdoc –test</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/39517">relnotes for 1.15.1</a>.</li>
  <li><a href="https://github.com/phungleson">@phungleson</a> made <a href="https://github.com/rust-lang/rust/pull/39486">a tiny doc wording change</a>.</li>
  <li><a href="https://github.com/apasel422">@apasel422</a> updated <a href="https://github.com/rust-lang/rust/pull/39196">nomicon to describe <code class="highlighter-rouge">#[may_dangle]</code></a>.</li>
  <li><a href="https://github.com/tspiteri">@tspiteri</a> marked <a href="https://github.com/rust-lang/rust/pull/39416">FFI functions with unsafety icon in rustdoc</a>.</li>
  <li><a href="https://github.com/Freyskeyd">@Freyskeyd</a> improved <a href="https://github.com/rust-lang/rust/pull/38823">doc cfg(test) and tests directory</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made <a href="https://github.com/rust-lang/rust/pull/39405">minor doc Option improvements</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>, added <a href="https://github.com/rust-lang/rust/pull/39506">missing urls in HashMap</a> and added <a href="https://github.com/rust-lang/rust/pull/39407">missing url in convert module</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 8th of February 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>