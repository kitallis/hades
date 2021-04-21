+++
title = "This Week in Rust Docs 82"
date = "2017-11-19T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-82"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> included <a href="https://github.com/rust-lang/rust/pull/44781">external files in documentation in rustdoc (RFC 1990)</a> and showed in docs <a href="https://github.com/rust-lang/rust/pull/45039">whether the return type of a function impls Iterator/Read/Write</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/45454">Release notes for 1.22.0</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/45776">code on diagnostics when underlined</a>, accounted <a href="https://github.com/rust-lang/rust/pull/45997">for missing keyword in fn/struct definition</a>, used <a href="https://github.com/rust-lang/rust/pull/46024">the proper term when using non-existing variant</a>, used <a href="https://github.com/rust-lang/rust/pull/45946">multiline text for crate conflict diagnostics</a>, displayed <a href="https://github.com/rust-lang/rust/pull/45953"><code class="highlighter-rouge">\t</code> in diagnostics code as four spaces</a>, highlighted <a href="https://github.com/rust-lang/rust/pull/45752">code on diagnostics when underlined</a> and removed <a href="https://github.com/rust-lang/rust/pull/45947">dereference suggestion on tuple argument</a>.</li>
  <li><a href="https://github.com/fhartwig">@fhartwig</a> made <a href="https://github.com/rust-lang/rust/pull/45645">rustdoc not include self-by-value methods from Deref target</a>.</li>
  <li><a href="https://github.com/JRegimbal">@JRegimbal</a> changed <a href="https://github.com/rust-lang/rust/pull/45898">“Types/modules” title of search tab to be more accurate</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/46035">structured suggestions for various “use” suggestions</a> and included <a href="https://github.com/rust-lang/rust/pull/46052">rendered diagnostic in json</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> added <a href="https://github.com/rust-lang/rust/pull/46088">doc for doing <code class="highlighter-rouge">Read</code> from <code class="highlighter-rouge">&amp;str</code></a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> combined <a href="https://github.com/rust-lang/rust/pull/46049">“this expression will panic at run-time” warnings into <code class="highlighter-rouge">const_err</code> lint</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/45998">broken CSS for book redirect pages</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/46081">path search</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/45767">talk about #![doc(test(no_crate_inject))] and #![doc(test(attr(…)))] in rustdoc</a>, tweaked <a href="https://github.com/rust-lang/rust/pull/45815">notes on ignore/compile_fail examples in rustdoc</a> and updated examples: <a href="https://github.com/rust-lang/rust/pull/45993">in Cow::into_owned don’t need to wrap result in Cows</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> started <a href="https://github.com/rust-lang/rust/pull/45692">shipping the Cargo book</a>.</li>
  <li><a href="https://github.com/pornel">@pornel</a> removed <a href="https://github.com/rust-lang/rust/pull/45828">deprecated message</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> removed <a href="https://github.com/rust-lang/rust/pull/46039">left over dead code from suggestion diagnostic refactoring</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> fixed <a href="https://github.com/rust-lang/rust/pull/45977">several pulldown warnings when documenting libstd</a>.</li>
  <li><a href="https://github.com/lnicola">@lnicola</a> escaped <a href="https://github.com/rust-lang/rust/pull/46010">doc root URLs in rustdoc</a>.</li>
  <li><a href="https://github.com/ExpHP">@ExpHP</a> added <a href="https://github.com/rust-lang/rust/pull/45984">context to E0084, E0517, E0518</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> deduplicated <a href="https://github.com/rust-lang/rust/pull/45952">projection error (E0271) messages</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/45673">search over generic types in docs</a>, fixed <a href="https://github.com/rust-lang/rust/pull/46066">primitive types not showing up</a>, set <a href="https://github.com/rust-lang/rust/pull/46005">short-message feature unstable</a> and added <a href="https://github.com/rust-lang/rust/pull/45970">missing links in FromStr docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 21st of November 2017 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>