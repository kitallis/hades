+++
title = "This Week in Rust Docs 39"
date = "2017-01-15T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-39"
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

<p><a href="https://github.com/Manishearth">@Manishearth</a> improved <a href="https://github.com/rust-lang/rust/pull/38843">rustdoc rendering for unstable features</a>! The rendering is available on <a href="https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html">nightly docs</a>.</p>

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
  <li><a href="https://github.com/ollie27">@ollie27</a> gave <a href="https://github.com/rust-lang/rust/pull/39076">primitive types stability attributes in rustdoc</a>.</li>
  <li><a href="https://github.com/radix">@radix</a> made a <a href="https://github.com/rust-lang/rust/pull/39072">minor improvement to strange grammar in E0525</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/39065">doc examples &amp; description in <code class="highlighter-rouge">std::os::unix::ffi</code>.</a>, made <a href="https://github.com/rust-lang/rust/pull/39028">minor improvements to docs in std::env structures/functions.</a> and added <a href="https://github.com/rust-lang/rust/pull/38761">‘platform-specific’ section to <code class="highlighter-rouge">sleep_ms</code> to match <code class="highlighter-rouge">sleep</code>.</a>.</li>
  <li><a href="https://github.com/jmdyck">@jmdyck</a> added <a href="https://github.com/rust-lang/rust/pull/39043">a link to the second edition of The Book</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> fixed <a href="https://github.com/rust-lang/rust/pull/38961">wording around sort guarantees</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> taught <a href="https://github.com/rust-lang/rust/pull/38955">diagnostics to highlight text</a> and provided <a href="https://github.com/rust-lang/rust/pull/38168">disambiguated syntax for candidates in E0034</a>.</li>
  <li><a href="https://github.com/APTy">@APTy</a> added <a href="https://github.com/rust-lang/rust/pull/39007">docs and tests for join_multicast_{v4,v6}</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/38966">1.15 release notes</a> and removed <a href="https://github.com/rust-lang/rust/pull/37057">all “consider using an explicit lifetime parameter” sug…</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> documented <a href="https://github.com/rust-lang/rust/pull/38247">the optional extra arguments to assert_eq!() / assert_ne!()</a>.</li>
  <li><a href="https://github.com/emilio">@emilio</a> added <a href="https://github.com/rust-lang/rust/pull/38825">an attribute to ignore collecting tests per-item in rustdoc</a>.</li>
  <li><a href="https://github.com/chris-morgan">@chris-morgan</a> fixed <a href="https://github.com/rust-lang/rust/pull/38922">a couple of bad Markdown links</a>.</li>
  <li><a href="https://github.com/insaneinside">@insaneinside</a> updated <a href="https://github.com/rust-lang/rust/pull/38930">src/libcore/ops.rs docs for RFC#1228 (Placement Left Arrow)</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> expanded <a href="https://github.com/rust-lang/rust/pull/38518">documentation of process::exit and exec</a>.</li>
  <li><a href="https://github.com/linclark">@linclark</a> added <a href="https://github.com/rust-lang/rust/pull/38108">error explanation for E0328.</a>.</li>
  <li><a href="https://github.com/chriskrycho">@chriskrycho</a> documented <a href="https://github.com/rust-lang/rust/pull/37928">RFC 1623: static lifetime elision.</a>.</li>
  <li><a href="https://github.com/utkarshkukreti">@utkarshkukreti</a> replaced <a href="https://github.com/rust-lang/rust/pull/38648">all <code class="highlighter-rouge">try!</code> with <code class="highlighter-rouge">?</code> in documentation examples</a>.</li>
  <li><a href="https://github.com/ConnyOnny">@ConnyOnny</a> added warning <a href="https://github.com/rust-lang/rust/pull/38794">for match enum in The Book</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/39069">missing blank space issue</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>, added <a href="https://github.com/rust-lang/rust/pull/38819">a distinct error code and description for “main function has wrong prototype</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a> and fixed <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/chris-morgan">@chris-morgan</a> fixed <a href="https://github.com/rust-lang/rust/pull/38569">rustdoc highlighting of <code class="highlighter-rouge">&amp;</code> and <code class="highlighter-rouge">*</code></a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> fixed <a href="https://github.com/rust-lang/rust/pull/38995">docs for min/max algorithms</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> escaped <a href="https://github.com/rust-lang/rust/pull/38244">the deprecated and unstable reason text in rustdoc</a> and taught <a href="https://github.com/rust-lang/rust/pull/38916">diagnostics to correct margin of multiline messages</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> improved <a href="https://github.com/rust-lang/rust/pull/38910">safety warning on ptr::swap</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> expanded <a href="https://github.com/rust-lang/rust/pull/38839">{Path,OsStr}::{to_str,to_string_lossy} doc examples</a>, clarified <a href="https://github.com/rust-lang/rust/pull/38581">behavior of <code class="highlighter-rouge">VecDeque::insert</code></a> and clarified <a href="https://github.com/rust-lang/rust/pull/38310">zero-value behavior of <code class="highlighter-rouge">ctlz</code>/<code class="highlighter-rouge">cttz</code> intrinsics</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> improved <a href="https://github.com/rust-lang/rust/pull/38843">rustdoc rendering for unstable features</a>, added <a href="https://github.com/rust-lang/rust/pull/38816">more docs for CoerceUnsized and Unsize</a> and removed <a href="https://github.com/rust-lang/rust/pull/38929">restrictions on docs in compiler-docs mode</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> updated <a href="https://github.com/rust-lang/rust/pull/38841">usage of rustc</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/38836">typo in tuple docs</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> changed <a href="https://github.com/rust-lang/rust/pull/39024"><code class="highlighter-rouge">to_owned</code> to <code class="highlighter-rouge">to_string</code> in docs</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> added <a href="https://github.com/rust-lang/rust/pull/38914">tidy check for lang gate tests</a>.</li>
  <li><a href="https://github.com/behnam">@behnam</a> fixed <a href="https://github.com/rust-lang/rust/pull/39027">typo in documentation</a>.</li>
  <li><a href="https://github.com/BenWiederhake">@BenWiederhake</a> fixed <a href="https://github.com/rust-lang/rust/pull/38994">some typos in Nomicon</a>.</li>
  <li><a href="https://github.com/minaguib">@minaguib</a> fixed <a href="https://github.com/rust-lang/rust/pull/38799">docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/38362">instant doc</a>, added <a href="https://github.com/rust-lang/rust/pull/38965">missing doc examples for Mutex</a> and added <a href="https://github.com/rust-lang/rust/pull/38946">missing links and examples for path modules and structs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 18th of January 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>