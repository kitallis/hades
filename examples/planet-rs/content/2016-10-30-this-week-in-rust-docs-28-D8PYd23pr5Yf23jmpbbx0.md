+++
title = "This Week in Rust Docs 28"
date = "2016-10-30T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-28"
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

<p>The way rustdoc is creating urls is problematic for the moment. A good summary of this issue can be found <a href="https://github.com/rust-lang/rust/issues/36417">here</a>. A few members of the Rust Doc team are preparing an RFC in order to improve this. If you want to get involved, feel free to speak about it with <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a> (imperio on IRC).</p>

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
  <li><a href="https://github.com/joshtriplett">@joshtriplett</a> copied/edited <a href="https://github.com/rust-lang/rust/pull/37473">on documentation for write! and writeln!</a> and documented <a href="https://github.com/rust-lang/rust/pull/37472">convention for using both fmt::Write and io::Write</a>.</li>
  <li><a href="https://github.com/AndiDog">@AndiDog</a> added <a href="https://github.com/rust-lang/rust/pull/37475">E0532 error explanation</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> added an <a href="https://github.com/rust-lang/rust/pull/37250">unsafe sign for unsafe functions in module page with superscript icons</a>.</li>
  <li><a href="https://github.com/achanda">@achanda</a> clarified <a href="https://github.com/rust-lang/rust/pull/37432">that send_to might panic in certain cases</a>.</li>
  <li><a href="https://github.com/Cobrand">@Cobrand</a> improved <a href="https://github.com/rust-lang/rust/pull/37438">docs for Index and IndexMut</a>.</li>
  <li><a href="https://github.com/polo-language">@polo-language</a> added <a href="https://github.com/rust-lang/rust/pull/37425">error note to illegal code snippet</a>.</li>
  <li><a href="https://github.com/jfirebaugh">@jfirebaugh</a> removed <a href="https://github.com/rust-lang/rust/pull/37058">long diagnostic for E0002</a> and added <a href="https://github.com/rust-lang/rust/pull/37242">a distinct error code and description for “main function has wrong type”</a>.</li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a> improved <a href="https://github.com/rust-lang/rust/pull/37405">“Doesn’t live long enough” error</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> made <a href="https://github.com/rust-lang/rust/pull/36615">E0243/E0244 message consistent with E0107</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> removed <a href="https://github.com/rust-lang/rust/pull/37057">all “consider using an explicit lifetime parameter” suggestions</a> and added <a href="https://github.com/rust-lang/rust/pull/37317">release notes for 1.12.1</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/37316">a few links in the docs</a>.</li>
  <li><a href="https://github.com/phungleson">@phungleson</a> updated <a href="https://github.com/rust-lang/rust/pull/36466">E0072 bonus to new error format</a>.</li>
  <li><a href="https://github.com/johnthagen">@johnthagen</a> added <a href="https://github.com/rust-lang/rust/pull/37386">example using Self to reference</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> started <a href="https://github.com/rust-lang/rust/pull/37388">the implementation of proposal for E0308</a>, improved <a href="https://github.com/rust-lang/rust/pull/37375">reference cast help message</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a> and printed <a href="https://github.com/rust-lang/rust/pull/37134">more tags in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/bluss">@bluss</a> added <a href="https://github.com/rust-lang/rust/pull/37343">documentation for Read, Write impls for slices and Vec</a>.</li>
  <li><a href="https://github.com/loggerhead">@loggerhead</a> fixed <a href="https://github.com/rust-lang/rust/pull/37228">an error of ‘book/deref-coercions.html’</a>.</li>
  <li><a href="https://github.com/mcarton">@mcarton</a> fixed <a href="https://github.com/rust-lang/rust/pull/36206">bad error message with <code class="highlighter-rouge">::&lt;</code> in types</a>.</li>
  <li><a href="https://github.com/robinst">@robinst</a> added <a href="https://github.com/rust-lang/rust/pull/37430">semicolon to “Maybe a missing <code class="highlighter-rouge">extern crate foo</code>” message</a>.</li>
  <li><a href="https://github.com/zoffixznet">@zoffixznet</a> fixed <a href="https://github.com/rust-lang/rust/pull/37398">typo</a>.</li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a> made <a href="https://github.com/rust-lang/rust/pull/37396">error E0221 more helpful</a>.</li>
  <li><a href="https://github.com/vtduncan">@vtduncan</a> fixed <a href="https://github.com/rust-lang/rust/pull/37391">broken links in Vec docs</a> and added <a href="https://github.com/rust-lang/rust/pull/37372">link to PathBuf from the Path docs</a>.</li>
  <li><a href="https://github.com/typelist">@typelist</a> fixed <a href="https://github.com/rust-lang/rust/pull/37358">typo that resulted in comparison-to-self</a>.</li>
  <li><a href="https://github.com/aidanhs">@aidanhs</a> <code class="highlighter-rouge">as_bytes</code> <a href="https://github.com/rust-lang/rust/pull/37327">is not the iterator on String, <code class="highlighter-rouge">bytes</code> is</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/37324">E0277 help message</a> and added <a href="https://github.com/rust-lang/rust/pull/37304">missing urls in collections module</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 2nd of November 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>