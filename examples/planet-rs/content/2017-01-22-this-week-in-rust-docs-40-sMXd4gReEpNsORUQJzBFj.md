+++
title = "This Week in Rust Docs 40"
date = "2017-01-22T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-40"
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
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/39233">more references between lowercase/uppercase operations.</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> fixed <a href="https://github.com/rust-lang/rust/pull/38961">wording around sort guarantees</a>.</li>
  <li><a href="https://github.com/bluecereal">@bluecereal</a> updated <a href="https://github.com/rust-lang/rust/pull/38436">if-let.md</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/DirkyJerky">@DirkyJerky</a> added <a href="https://github.com/rust-lang/rust/pull/39200">docs for atomic orderings: link to the ‘nomicon article for further reading</a>.</li>
  <li><a href="https://github.com/rspeer">@rspeer</a> fixed <a href="https://github.com/rust-lang/rust/pull/39174">a misleading statement in <code class="highlighter-rouge">Iterator.nth()</code></a>.</li>
  <li><a href="https://github.com/cesarb">@cesarb</a> updated The Book: <a href="https://github.com/rust-lang/rust/pull/39191">size and align in trait object vtables are used</a>.</li>
  <li><a href="https://github.com/apasel422">@apasel422</a> updated <a href="https://github.com/rust-lang/rust/pull/39196">nomicon to describe <code class="highlighter-rouge">#[may_dangle]</code></a>.</li>
  <li><a href="https://github.com/APTy">@APTy</a> added <a href="https://github.com/rust-lang/rust/pull/39007">docs and tests for join_multicast_{v4,v6} for UDP socket</a>.</li>
  <li><a href="https://github.com/linclark">@linclark</a> added <a href="https://github.com/rust-lang/rust/pull/38108">error explanation for E0328</a>.</li>
  <li><a href="https://github.com/Freyskeyd">@Freyskeyd</a> improved <a href="https://github.com/rust-lang/rust/pull/38823">doc cfg(test) and tests directory</a>.</li>
  <li><a href="https://github.com/insaneinside">@insaneinside</a> updated <a href="https://github.com/rust-lang/rust/pull/38930">src/libcore/ops.rs docs for RFC#1228 (Placement Left Arrow)</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39224">missing urls for OsStr and OsString</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>, forced <a href="https://github.com/rust-lang/rust/pull/39222">backline on all where in docs</a>, added ref <a href="https://github.com/rust-lang/rust/pull/37658">suggestion</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/ollie27">@ollie27</a> gave <a href="https://github.com/rust-lang/rust/pull/39076">primitive types stability attributes in rustdoc</a>.</li>
  <li><a href="https://github.com/radix">@radix</a> made a <a href="https://github.com/rust-lang/rust/pull/39072">minor improvement to strange grammar in E0525</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/39065">doc examples &amp; description in <code class="highlighter-rouge">std::os::unix::ffi</code></a>, made <a href="https://github.com/rust-lang/rust/pull/39028">minor improvements to docs in std::env structures/functions</a>, added <a href="https://github.com/rust-lang/rust/pull/38761">‘platform-specific’ section to <code class="highlighter-rouge">sleep_ms</code> to match <code class="highlighter-rouge">sleep</code></a>, added <a href="https://github.com/rust-lang/rust/pull/39221">doc examples for <code class="highlighter-rouge">std::ffi::OsString</code> fucntions/methods</a>, made <a href="https://github.com/rust-lang/rust/pull/39165">a few improvements to the slice docs</a> and made <a href="https://github.com/rust-lang/rust/pull/38457">improvements to ‘include’ macro documentation</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/38966">1.15 release notes</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> documented <a href="https://github.com/rust-lang/rust/pull/38247">the optional extra arguments to assert_eq!() / assert_ne!()</a>.</li>
  <li><a href="https://github.com/chris-morgan">@chris-morgan</a> fixed <a href="https://github.com/rust-lang/rust/pull/38922">a couple of bad Markdown links</a>.</li>
  <li><a href="https://github.com/ranma42">@ranma42</a> documented <a href="https://github.com/rust-lang/rust/pull/39203">that <code class="highlighter-rouge">Metadata</code> can be obtained from <code class="highlighter-rouge">symlink_metadata</code></a>.</li>
  <li><a href="https://github.com/TheCycoONE">@TheCycoONE</a> clarified <a href="https://github.com/rust-lang/rust/pull/39135">when range is removed by drain</a>.</li>
  <li><a href="https://github.com/grimreaper">@grimreaper</a> fixed a <a href="https://github.com/rust-lang/rust/pull/39121">minor typo</a>.</li>
  <li><a href="https://github.com/king6cong">@king6cong</a> improved <a href="https://github.com/rust-lang/rust/pull/39115">doc wording</a>.</li>
  <li><a href="https://github.com/richard-imaoka">@richard-imaoka</a> added <a href="https://github.com/rust-lang/rust/pull/39091">error explanation for E0491</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/39069">missing blank space issue</a> and specified <a href="https://github.com/rust-lang/rust/pull/39210">the result of integer cast on boolean</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 25th of January 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>