+++
title = "This Week in Rust Docs 27"
date = "2016-10-23T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-27"
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
  <li><a href="https://github.com/bluss">@bluss</a> added <a href="https://github.com/rust-lang/rust/pull/37343">documentation for Read, Write impls for slices and Vec</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> made <a href="https://github.com/rust-lang/rust/pull/36615">E0243/E0244 message consistent with E0107</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/37317">release notes for 1.12.1</a> and removed <a href="https://github.com/rust-lang/rust/pull/37057">all “consider using an explicit lifetime parameter” suggestions</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/37316">a few links in the docs</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> is working on rustdoc: <a href="https://github.com/rust-lang/rust/pull/37250">flag unsafe fns in module’s function list</a>.</li>
  <li><a href="https://github.com/phungleson">@phungleson</a> updated <a href="https://github.com/rust-lang/rust/pull/36466">E0072 bonus to new error format</a>.</li>
  <li><a href="https://github.com/jfirebaugh">@jfirebaugh</a> added <a href="https://github.com/rust-lang/rust/pull/37242">a distinct error code and description for “main function has wrong type”</a> and removed <a href="https://github.com/rust-lang/rust/pull/37058">long diagnostic for E0002</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> rustdoc: <a href="https://github.com/rust-lang/rust/pull/37190">add line breaks to where clauses a la rustfmt</a>.</li>
  <li><a href="https://github.com/loggerhead">@loggerhead</a> fixed <a href="https://github.com/rust-lang/rust/pull/37228">an error of ‘book/deref-coercions.html’</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">the reference isn’t normative</a>.</li>
  <li><a href="https://github.com/diwic">@diwic</a> fixed <a href="https://github.com/rust-lang/rust/pull/36849">documentation typo</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> Improve <a href="https://github.com/rust-lang/rust/pull/37324">E0277 help message</a>, Add <a href="https://github.com/rust-lang/rust/pull/37304">missing urls in collections module</a>, Add <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a> and Print <a href="https://github.com/rust-lang/rust/pull/37134">more tags in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/37187">doc example for <code class="highlighter-rouge">std::borrow::Cow</code></a>.</li>
  <li><a href="https://github.com/nabeelomer">@nabeelomer</a> updated <a href="https://github.com/rust-lang/rust/pull/37189">the docs for Error::description</a>.</li>
  <li><a href="https://github.com/aidanhs">@aidanhs</a> <code class="highlighter-rouge">as_bytes</code> <a href="https://github.com/rust-lang/rust/pull/37327">is not the iterator on String, <code class="highlighter-rouge">bytes</code> is</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> added <a href="https://github.com/rust-lang/rust/pull/37314">a simpler description of Iterator::nth</a>.</li>
  <li><a href="https://github.com/vkatsikaros">@vkatsikaros</a> made <a href="https://github.com/rust-lang/rust/pull/37309">a minor clarification in guessing game</a>.</li>
  <li><a href="https://github.com/senior">@senior</a> added <a href="https://github.com/rust-lang/rust/pull/37244">error explaination for E0182, E0230 and E0399</a>.</li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a> improved <a href="https://github.com/rust-lang/rust/pull/37174">“Doesn’t live long enough” error</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> rustdoc: <a href="https://github.com/rust-lang/rust/pull/37098">Improve playground run buttons</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> corrected <a href="https://github.com/rust-lang/rust/pull/37193">erroneous pluralization of ‘1 type argument’ error messages</a>.</li>
  <li><a href="https://github.com/jethrogb">@jethrogb</a> added <a href="https://github.com/rust-lang/rust/pull/37240">stable example to TypeId</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37165">missing urls for io types</a>, added <a href="https://github.com/rust-lang/rust/pull/36964">invalid doc comment help message</a>, added <a href="https://github.com/rust-lang/rust/pull/37043">missing urls on Vec docs</a>, Rollup <a href="https://github.com/rust-lang/rust/pull/37337">of 10 pull requests</a>, Rollup <a href="https://github.com/rust-lang/rust/pull/37289">of 7 pull requests</a>, Add <a href="https://github.com/rust-lang/rust/pull/37259">more io urls</a> and Rollup <a href="https://github.com/rust-lang/rust/pull/37237">of 6 pull requests</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 26th of October 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>