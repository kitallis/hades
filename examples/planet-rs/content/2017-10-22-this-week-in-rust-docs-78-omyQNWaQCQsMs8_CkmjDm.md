+++
title = "This Week in Rust Docs 78"
date = "2017-10-22T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-78"
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

<p>The switch to <a href="https://github.com/google/pulldown-cmark">Pulldown</a> for the rust doc rendering has finally <a href="https://github.com/rust-lang/rust/pull/41991">started</a>!</p>

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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a>, showed <a href="https://github.com/rust-lang/rust/pull/45039">in docs whether the return type of a function impls Iterator/Read/Write</a> and updated <a href="https://github.com/rust-lang/rust/pull/45421">pulldown and fixed spurious rendering difference around footnotes</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/45429">docs around <code class="highlighter-rouge">Once::call_once_force</code> and <code class="highlighter-rouge">OnceState</code></a>.</li>
  <li><a href="https://github.com/carols10cents">@carols10cents</a> corrected <a href="https://github.com/rust-lang/rust/pull/45398">misspelling in error text: re-assignment =&gt; reassignment</a>.</li>
  <li><a href="https://github.com/Technius">@Technius</a> improved <a href="https://github.com/rust-lang/rust/pull/45295">std::process module docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a>, improved <a href="https://github.com/rust-lang/rust/pull/45187">sidebar rendering and added methods list</a>, limited <a href="https://github.com/rust-lang/rust/pull/45212">the sidebar height</a> and added <a href="https://github.com/rust-lang/rust/pull/45361">missing code examples</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> made <a href="https://github.com/rust-lang/rust/pull/44613">rustdoc optimizations</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/45419">most rendering warnings from switching to CommonMark</a>.</li>
  <li><a href="https://github.com/sunjay">@sunjay</a> documented <a href="https://github.com/rust-lang/rust/pull/45098">the process for when rustfmt/rls break</a>.</li>
  <li><a href="https://github.com/jacwah">@jacwah</a> mentionned <a href="https://github.com/rust-lang/rust/pull/45082">Clone and refs in –explain E0382</a>.</li>
  <li><a href="https://github.com/goffrie">@goffrie</a> provided <a href="https://github.com/rust-lang/rust/pull/45123">the full span of method calls to <code class="highlighter-rouge">check_argument_types</code></a> and marked <a href="https://github.com/rust-lang/rust/pull/45316">block exits as reachable if the block can break</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> made <a href="https://github.com/rust-lang/rust/pull/45181">a list of all lang items in unstable book</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expanded <a href="https://github.com/rust-lang/rust/pull/45227">docs/examples for TCP <code class="highlighter-rouge">set_nonblocking</code> methods</a>.</li>
  <li><a href="https://github.com/0xAX">@0xAX</a> fixed <a href="https://github.com/rust-lang/rust/pull/45264">a typo in src/bootstrap/README.md</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> fixed <a href="https://github.com/rust-lang/rust/pull/45217">out of date unstable book entries for <code class="highlighter-rouge">alloc_*</code> features</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> fixed <a href="https://github.com/rust-lang/rust/pull/45407">typos in README.md</a>, fixed <a href="https://github.com/rust-lang/rust/pull/45376">typos in src/librustc/README.md</a> and fixed <a href="https://github.com/rust-lang/rust/pull/45377">typos in librustc/ty/README.md</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> removed <a href="https://github.com/rust-lang/rust/pull/45418">“gender” from code of conduct, keep only “gender identity and expression”</a>.</li>
  <li><a href="https://github.com/neunenak">@neunenak</a> added <a href="https://github.com/rust-lang/rust/pull/45356">explanatory text for error E0599</a>.</li>
  <li><a href="https://github.com/christianpoveda">@christianpoveda</a> added <a href="https://github.com/rust-lang/rust/pull/45349">examples of closures for str::find</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/45232">code suggestions for non-shorthand field pattern, no-mangle lints</a> and removed <a href="https://github.com/rust-lang/rust/pull/45315">“expected statement after outer attr.” after inner attr</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> updated docs: <a href="https://github.com/rust-lang/rust/pull/45340">a LocalKey might start in the Valid state</a>.</li>
  <li><a href="https://github.com/xfix">@xfix</a> updated <a href="https://github.com/rust-lang/rust/pull/45339">array documentation for Clone trait changes</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> added <a href="https://github.com/rust-lang/rust/pull/45308">missing headlines in rustdoc book</a> and fixed <a href="https://github.com/rust-lang/rust/pull/45307">typo in rustdoc book</a>.</li>
  <li><a href="https://github.com/Pirh">@Pirh</a> documented <a href="https://github.com/rust-lang/rust/pull/45151">defaults for stdin, stdout, and stderr methods of Command</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> saved <a href="https://github.com/rust-lang/rust/pull/45281">selected search tab</a>, saved <a href="https://github.com/rust-lang/rust/pull/45288">the highlighted item when switching tab</a>, removed <a href="https://github.com/rust-lang/rust/pull/45280">terribly useless and problematic margin when searching on mobile</a>, fixed <a href="https://github.com/rust-lang/rust/pull/45289">arrow display</a>, hid <a href="https://github.com/rust-lang/rust/pull/45290">help when search bar is focused</a>, printed <a href="https://github.com/rust-lang/rust/pull/45324">rustdoc rendering warnings all the time</a> and removed <a href="https://github.com/rust-lang/rust/pull/45329">duplicated word</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 24th of October 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>