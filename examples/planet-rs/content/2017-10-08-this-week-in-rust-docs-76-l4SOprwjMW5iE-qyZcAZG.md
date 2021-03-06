+++
title = "This Week in Rust Docs 76"
date = "2017-10-08T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-76"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> made <a href="https://github.com/rust-lang/rust/pull/44613">rustdoc optimizations</a>, abd included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a>, showed <a href="https://github.com/rust-lang/rust/pull/45039">in docs whether the return type of a function impls Iterator/Read/Write</a>, documented <a href="https://github.com/rust-lang/rust/pull/44969">trait impls when the type appears in the trait’s generics</a> and let <a href="https://github.com/rust-lang/rust/pull/44989">rustdoc print the crate version into docs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> improved <a href="https://github.com/rust-lang/rust/pull/44867">doc-test: in Markdown tests, Use all of <code class="highlighter-rouge">&lt;h1&gt;</code> to <code class="highlighter-rouge">&lt;h6&gt;</code> as the test name</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> improved <a href="https://github.com/rust-lang/rust/pull/44855">docs for CStr, CString, OsStr, OsString</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/44970">syntax when finding method on array</a>, warned <a href="https://github.com/rust-lang/rust/pull/44881">when assigning a block that doesn’t have an implicit return</a> and referred <a href="https://github.com/rust-lang/rust/pull/44642">to types using the local identifier</a>.</li>
  <li><a href="https://github.com/sunjay">@sunjay</a> documented <a href="https://github.com/rust-lang/rust/pull/45098">the process for when rustfmt/rls break</a>.</li>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> don’t <a href="https://github.com/rust-lang/rust/pull/44962">encourage people to ignore threading errors in the docs</a>.</li>
  <li><a href="https://github.com/jacwah">@jacwah</a> mentionned <a href="https://github.com/rust-lang/rust/pull/45082">Clone and refs in –explain E0382</a>.</li>
  <li><a href="https://github.com/Nemo157">@Nemo157</a> rendered <a href="https://github.com/rust-lang/rust/pull/44994">cfg(feature) requirements in documentation</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a> and added <a href="https://github.com/rust-lang/rust/pull/45055">tabs for search for better information access</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/44481">RELEASES.md for 1.21.0</a>.</li>
  <li><a href="https://github.com/budziq">@budziq</a> corrected <a href="https://github.com/rust-lang/rust/pull/44664">the CONTRIBUTING.md “External Dependencies” section</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> rendered <a href="https://github.com/rust-lang/rust/pull/44920"><code class="highlighter-rouge">[src]</code> links for trait implementors in rustdoc</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> improved <a href="https://github.com/rust-lang/rust/pull/44897">docs for size_of::&lt;#[repr(C)]&gt; items</a>.</li>
  <li><a href="https://github.com/leavehouse">@leavehouse</a> fixed <a href="https://github.com/rust-lang/rust/pull/44913">TcpStream::local_addr docs example code</a>.</li>
  <li><a href="https://github.com/Pirh">@Pirh</a> updated <a href="https://github.com/rust-lang/rust/pull/44905">docs for process::abort</a>.</li>
  <li><a href="https://github.com/hunteke">@hunteke</a> fixed <a href="https://github.com/rust-lang/rust/pull/45058">typo</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/44980">books for next release</a> and updated <a href="https://github.com/rust-lang/rust/pull/44977">mdbook</a>.</li>
  <li><a href="https://github.com/brennie">@brennie</a> updated <a href="https://github.com/rust-lang/rust/pull/45042">trait summaries for std::fmt</a>.</li>
  <li><a href="https://github.com/vitiral">@vitiral</a> added <a href="https://github.com/rust-lang/rust/pull/44935">links to headers in README and CONTRIBUTING</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/45024">the issue number to doc_masked’s feature gate</a> and let <a href="https://github.com/rust-lang/rust/pull/44949">htmldocck.py check for directories</a>.</li>
  <li><a href="https://github.com/MaikKlein">@MaikKlein</a> fixed <a href="https://github.com/rust-lang/rust/pull/45006">typo in <code class="highlighter-rouge">librustc/README.md</code></a>.</li>
  <li><a href="https://github.com/laumann">@laumann</a> fixed typo: <a href="https://github.com/rust-lang/rust/pull/44955">geneartor -&gt; generator</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> added <a href="https://github.com/rust-lang/rust/pull/44944">trace_macros! to unstable book</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/44892">missing fnty args rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/45053">missing links for AtomicBool</a> and added <a href="https://github.com/rust-lang/rust/pull/45017">missing urls for Mutex</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 11th of October 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>