+++
title = "This Week in Rust Docs 43"
date = "2017-02-12T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-43"
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

<p>The <code class="highlighter-rouge">rustdoc --test</code> output has been updated. Now it looks like this (or at least it’ll when <a href="https://github.com/rust-lang/rust/pull/39743">#39743</a> will get merged):</p>

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
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> added <a href="https://github.com/rust-lang/rust/pull/39738">notes about capacity effects to Vec::truncate()</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> documented <a href="https://github.com/rust-lang/rust/pull/39757">the return value of zero-size/zero-heap pointer types</a>.</li>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> improved <a href="https://github.com/rust-lang/rust/pull/39760">grammar on field init docs</a> and removed <a href="https://github.com/rust-lang/rust/pull/39758">duplicated “parameter” in E0089 text</a>.</li>
  <li><a href="https://github.com/Rufflewind">@Rufflewind</a> resolved <a href="https://github.com/rust-lang/rust/pull/39748">ambiguities in the Generics in the Book</a>.</li>
  <li><a href="https://github.com/JordiPolo">@JordiPolo</a> substituted <a href="https://github.com/rust-lang/rust/pull/39756">try! for ? in the Result documentation</a>.</li>
  <li><a href="https://github.com/jimmycuadra">@jimmycuadra</a> included <a href="https://github.com/rust-lang/rust/pull/39740">a stability span only if needed in rustdoc</a>.</li>
  <li><a href="https://github.com/Dowwie">@Dowwie</a> updated <a href="https://github.com/rust-lang/rust/pull/39691">attributes.md - Last sentence updated to reflect support for custom attributes</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> cleaned up <a href="https://github.com/rust-lang/rust/pull/39713">“pattern doesn’t bind x” messages</a>.</li>
  <li><a href="https://github.com/notriddle">@notriddle</a> added <a href="https://github.com/rust-lang/rust/pull/39697">the item type to the tooltip</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> showed <a href="https://github.com/rust-lang/rust/pull/39654">attributes on all item types in rustdoc</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> used <a href="https://github.com/rust-lang/rust/pull/39441">help rather than span note for no method error; a slight rephrasing</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> distinguished <a href="https://github.com/rust-lang/rust/pull/39458">guesses from suggestions</a>.</li>
  <li><a href="https://github.com/jrmuizel">@jrmuizel</a> removed <a href="https://github.com/rust-lang/rust/pull/39304">obsolete documentation about drop-flags</a>.</li>
  <li><a href="https://github.com/APTy">@APTy</a> added <a href="https://github.com/rust-lang/rust/pull/39007">docs and tests for join_multicast_{v4,v6}</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39743">tested item in the rustdoc –test output</a>, added <a href="https://github.com/rust-lang/rust/pull/39513">missing urls and small nits</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> improved <a href="https://github.com/rust-lang/rust/pull/39554">error message when two-arg assert_eq! receives a trailing comma</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> gave <a href="https://github.com/rust-lang/rust/pull/39444">a better error message for unknown derive messages</a>.</li>
  <li><a href="https://github.com/cengizIO">@cengizIO</a> improved <a href="https://github.com/rust-lang/rust/pull/39361">error message for uninferrable types #38812</a>.</li>
  <li><a href="https://github.com/JordiPolo">@JordiPolo</a> changed <a href="https://github.com/rust-lang/rust/pull/39545">deprecation warning to indicate custom derive support was removed</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> fixed <a href="https://github.com/rust-lang/rust/pull/39312">full path being output with <code class="highlighter-rouge">rustdoc -h</code></a>.</li>
  <li><a href="https://github.com/rspeer">@rspeer</a> fixed <a href="https://github.com/rust-lang/rust/pull/39174">a misleading statement in<code class="highlighter-rouge">Iterator.nth()</code></a>.</li>
  <li><a href="https://github.com/bluecereal">@bluecereal</a> updated <a href="https://github.com/rust-lang/rust/pull/38436">if-let.md</a>.</li>
  <li><a href="https://github.com/phungleson">@phungleson</a> fixed <a href="https://github.com/rust-lang/rust/pull/39459">short hand struct doc</a> and removed <a href="https://github.com/rust-lang/rust/pull/39443">suggestions to use things which weren’t found either</a>.</li>
  <li><a href="https://github.com/chriskrycho">@chriskrycho</a> documented <a href="https://github.com/rust-lang/rust/pull/37928">RFC 1623: static lifetime elision.</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> removed <a href="https://github.com/rust-lang/rust/pull/39374">lie about #[cfg] from reference</a> and changed <a href="https://github.com/rust-lang/rust/pull/39707">span_notes to notes in E0368/E0369</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/39710">1.15.1 relnotes</a>.</li>
  <li><a href="https://github.com/sgrif">@sgrif</a> explicitly mentioned <a href="https://github.com/rust-lang/rust/pull/39701">that <code class="highlighter-rouge">Vec::reserve</code> is based on len not capacity</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/39725">nightly book with installing nightly instructions</a>.</li>
  <li><a href="https://github.com/jethrogb">@jethrogb</a> updated <a href="https://github.com/rust-lang/rust/pull/39708">set operations documentation</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> improved <a href="https://github.com/rust-lang/rust/pull/39589">impl disambiguation in rustdoc</a>.</li>
  <li><a href="https://github.com/Gheoan">@Gheoan</a> added <a href="https://github.com/rust-lang/rust/pull/39620">missing comma</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> re-wrote <a href="https://github.com/rust-lang/rust/pull/39593">the doc index page</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39002">Debug implementations for libcollection structs</a>, displayed <a href="https://github.com/rust-lang/rust/pull/39597">correct filename with –test option</a>, added <a href="https://github.com/rust-lang/rust/pull/39649">missing urls on join_paths</a> and added <a href="https://github.com/rust-lang/rust/pull/39621">missing urls for current_dir</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 15th of February 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>