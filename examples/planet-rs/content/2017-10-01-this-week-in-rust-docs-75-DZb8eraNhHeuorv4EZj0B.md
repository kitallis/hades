+++
title = "This Week in Rust Docs 75"
date = "2017-10-01T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-75"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> made <a href="https://github.com/rust-lang/rust/pull/44613">rustdoc optimizations</a> abd included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/44481">RELEASES.md for 1.21.0</a>.</li>
  <li><a href="https://github.com/budziq">@budziq</a> corrected <a href="https://github.com/rust-lang/rust/pull/44664">the CONTRIBUTING.md “External Dependencies” section</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> rendered <a href="https://github.com/rust-lang/rust/pull/44920"><code class="highlighter-rouge">[src]</code> links for trait implementors in rustdoc</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> improved <a href="https://github.com/rust-lang/rust/pull/44897">docs for size_of::&lt;#[repr(C)]&gt; items</a>.</li>
  <li><a href="https://github.com/leavehouse">@leavehouse</a> fixed <a href="https://github.com/rust-lang/rust/pull/44913">TcpStream::local_addr docs example code</a>.</li>
  <li><a href="https://github.com/Pirh">@Pirh</a> updated <a href="https://github.com/rust-lang/rust/pull/44905">docs for process::abort</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> improved <a href="https://github.com/rust-lang/rust/pull/44867">doc-test: in Markdown tests, Use all of <code class="highlighter-rouge">&lt;h1&gt;</code> to <code class="highlighter-rouge">&lt;h6&gt;</code> as the test name</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> improved <a href="https://github.com/rust-lang/rust/pull/44855">docs for CStr, CString, OsStr, OsString</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a> and added <a href="https://github.com/rust-lang/rust/pull/44892">missing fnty args rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/laumann">@laumann</a> added <a href="https://github.com/rust-lang/rust/pull/44297">suggestions for misspelled method names</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/44103">comparison operators to must-use lint (under <code class="highlighter-rouge">fn_must_use</code> feature)</a> and prevented <a href="https://github.com/rust-lang/rust/pull/44713">rustdoc to get confused by text “fn main” in a line comment</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/44625">how ChildStd{in,out,err} FDs are closed</a>.</li>
  <li><a href="https://github.com/thombles">@thombles</a> improved <a href="https://github.com/rust-lang/rust/pull/44786">diagnostics when attempting to match tuple enum variant with struct pattern</a>.</li>
  <li><a href="https://github.com/tirr-c">@tirr-c</a> made <a href="https://github.com/rust-lang/rust/pull/44735">a friendlier error message for closure argument type mismatch</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/44782].">at parameter type on E0301</a>, pointed <a href="https://github.com/rust-lang/rust/pull/44847">at signature on unused lint</a> anded point <a href="https://github.com/rust-lang/rust/pull/44782">at parameter type on E0301</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> added <a href="https://github.com/rust-lang/rust/pull/44944">trace_macros! to unstable book</a>.</li>
  <li><a href="https://github.com/pnkfelix">@pnkfelix</a> some <a href="https://github.com/rust-lang/rust/pull/44736">fixes to mir-borrowck</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/44789">warning position in rustdoc code blocks</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 4th of October 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>