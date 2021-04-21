+++
title = "This Week in Rust Docs 24"
date = "2016-10-02T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-24"
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
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> made a <a href="https://github.com/rust-lang/rust/pull/36903">“minor” librustdoc cleanup and refactoring</a>.</li>
  <li><a href="https://github.com/BlueSpaceCanary">@BlueSpaceCanary</a> removed <a href="https://github.com/rust-lang/rust/pull/36878">the double introduction for <code class="highlighter-rouge">cargo run</code></a>.</li>
  <li><a href="https://github.com/gavinb">@gavinb</a> improved <a href="https://github.com/rust-lang/rust/pull/36798">error message and snippet for “did you mean <code class="highlighter-rouge">x</code>”</a>.</li>
  <li><a href="https://github.com/green-coder">@green-coder</a> updated <a href="https://github.com/rust-lang/rust/pull/36746">example code in the documentation</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> updated rustdoc <a href="https://github.com/rust-lang/rust/pull/36679">to print non-self arguments of bare functions and struct methods on their own line</a>.</li>
  <li><a href="https://github.com/kmcallister">@kmcallister</a> updated <a href="https://github.com/rust-lang/rust/pull/36665">Arc docs to match new Rc docs</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> updated rustdoc to <a href="https://github.com/rust-lang/rust/pull/36644">inline sidebar items, to display more in a page</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> made <a href="https://github.com/rust-lang/rust/pull/36615">E0243/E0244 message consistent with E0107</a>.</li>
  <li><a href="https://github.com/christopherdumas">@christopherdumas</a> updated <a href="https://github.com/rust-lang/rust/pull/36404"><code class="highlighter-rouge">includes!</code> documentation</a>.</li>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> added <a href="https://github.com/rust-lang/rust/pull/35880">links to interesting items in <code class="highlighter-rouge">std::ptr</code> documentation</a>, <a href="https://github.com/rust-lang/rust/pull/35759">refactored range examples</a> and added <a href="https://github.com/rust-lang/rust/pull/36364"><code class="highlighter-rouge">default</code> docstrings for <code class="highlighter-rouge">String</code>, <code class="highlighter-rouge">AtomicBool</code>, and <code class="highlighter-rouge">Generics</code></a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">reference isn’t normative</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/36637">run button appearing when it shouldn’t</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a> and fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/alygin">@alygin</a> updated <a href="https://github.com/rust-lang/rust/pull/36756">E0512 to new error format</a>.</li>
  <li><a href="https://github.com/KiChjang">@KiChjang</a> updated <a href="https://github.com/rust-lang/rust/pull/36757">E0025 to new error format</a>.</li>
  <li><a href="https://github.com/nathanmusoke">@nathanmusoke</a> fixed <a href="https://github.com/rust-lang/rust/pull/36769">minor typo in variable bindings in The Book</a>.</li>
  <li><a href="https://github.com/palango">@palango</a> added <a href="https://github.com/rust-lang/rust/pull/36813">link to format! docs</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> made <a href="https://github.com/rust-lang/rust/pull/36860"><code class="highlighter-rouge">Send</code> and <code class="highlighter-rouge">Sync</code> traits to the reference</a>.</li>
  <li><a href="https://github.com/tmiasko">@tmiasko</a> fixed <a href="https://github.com/rust-lang/rust/pull/36851">BufRead::read_until documentation</a> and reworded <a href="https://github.com/rust-lang/rust/pull/36833">description of SystemTimeError</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> made <a href="https://github.com/rust-lang/rust/pull/36872">a couple refactorings in librustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/36765">basic doc example for <code class="highlighter-rouge">core::ptr::write_bytes</code></a>.</li>
  <li><a href="https://github.com/bluss">@bluss</a> updated rustdoc’s CSS <a href="https://github.com/rust-lang/rust/pull/36676">to put <code class="highlighter-rouge">where</code> in trait listings on a new line</a>.</li>
  <li><a href="https://github.com/japaric">@japaric</a> implemented <a href="https://github.com/rust-lang/rust/pull/36586">–sysroot on rustdoc</a>.</li>
  <li><a href="https://github.com/giannicic">@giannicic</a> fixed <a href="https://github.com/rust-lang/rust/pull/36652">E0520 error message</a>.</li>
  <li><a href="https://github.com/pmatos">@pmatos</a> improved <a href="https://github.com/rust-lang/rust/pull/36672">documention troubleshooting missing linker</a>.</li>
  <li><a href="https://github.com/vanjacosic">@vanjacosic</a> updated <a href="https://github.com/rust-lang/rust/pull/36564">the “Ownership” section of The Book</a> and updated <a href="https://github.com/rust-lang/rust/pull/36563">the “Getting started” section of The Book</a>.</li>
  <li><a href="https://github.com/jonathandturner">@jonathandturner</a> updated <a href="https://github.com/rust-lang/rust/pull/36761">E0425, E0446 and E0449</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> updated <a href="https://github.com/rust-lang/rust/pull/36535">doc comments to new macro url syntax</a>, added <a href="https://github.com/rust-lang/rust/pull/36723">E0513 stuff</a>, updated <a href="https://github.com/rust-lang/rust/pull/36873">E0035, E0036 and E0370 to new error format</a>, improved <a href="https://github.com/rust-lang/rust/pull/36841">process module doc a bit</a>, added <a href="https://github.com/rust-lang/rust/pull/36810">missing urls for ops module</a>, added <a href="https://github.com/rust-lang/rust/pull/36750">missing links on cmp module</a>, fixed <a href="https://github.com/rust-lang/rust/pull/36623">some typos and improve doc comments style</a> and added <a href="https://github.com/rust-lang/rust/pull/36576">missing urls for Box doc</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 5th of October 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>