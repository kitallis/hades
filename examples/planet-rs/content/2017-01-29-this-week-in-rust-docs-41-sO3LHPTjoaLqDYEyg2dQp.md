+++
title = "This Week in Rust Docs 41"
date = "2017-01-29T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-41"
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
  <li><a href="https://github.com/Wilfred">@Wilfred</a> made a <a href="https://github.com/rust-lang/rust/pull/39389">minor grammar fix ‘can not’ -&gt; ‘cannot’</a>.</li>
  <li><a href="https://github.com/cengizIO">@cengizIO</a> improved <a href="https://github.com/rust-lang/rust/pull/39361">error message for uninferrable types</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> added <a href="https://github.com/rust-lang/rust/pull/39116">clearer error message using <code class="highlighter-rouge">&amp;str + &amp;str</code></a> and fixed <a href="https://github.com/rust-lang/rust/pull/39312">full path being output with <code class="highlighter-rouge">rustdoc -h</code></a>.</li>
  <li><a href="https://github.com/durka">@durka</a> removed <a href="https://github.com/rust-lang/rust/pull/39374">lie about #[cfg] from reference</a>.</li>
  <li><a href="https://github.com/canndrew">@canndrew</a> added <a href="https://github.com/rust-lang/rust/pull/39009">warning for () to ! switch</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added a <a href="https://github.com/rust-lang/rust/pull/38103">note for individual lint name in messages set via lint group attribute</a>.</li>
  <li><a href="https://github.com/jrmuizel">@jrmuizel</a> removed <a href="https://github.com/rust-lang/rust/pull/39304">obsolete documentation about drop-flags</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> highlighted <a href="https://github.com/rust-lang/rust/pull/39300">code in <code class="highlighter-rouge">rustc --explain</code></a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> clarified <a href="https://github.com/rust-lang/rust/pull/39299">the lack of mutability inside an Rc in std:rc</a>.</li>
  <li><a href="https://github.com/rspeer">@rspeer</a> fixed <a href="https://github.com/rust-lang/rust/pull/39174">a misleading statement in <code class="highlighter-rouge">Iterator.nth()</code></a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/39233">more references between lowercase/uppercase operations.</a> and added <a href="https://github.com/rust-lang/rust/pull/39221">doc examples for <code class="highlighter-rouge">std::ffi::OsString</code> fucntions/methods.</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> fixed <a href="https://github.com/rust-lang/rust/pull/38961">wording around sort guarantees</a>.</li>
  <li><a href="https://github.com/DirkyJerky">@DirkyJerky</a> added <a href="https://github.com/rust-lang/rust/pull/39200">docs for atomic orderings: link to the ‘nomicon article for further reading</a>.</li>
  <li><a href="https://github.com/cesarb">@cesarb</a> updated The Book: <a href="https://github.com/rust-lang/rust/pull/39191">size and align in trait object vtables are used</a>.</li>
  <li><a href="https://github.com/linclark">@linclark</a> added <a href="https://github.com/rust-lang/rust/pull/38108">error explanation for E0328</a>.</li>
  <li><a href="https://github.com/jtxx000">@jtxx000</a> fixed <a href="https://github.com/rust-lang/rust/pull/39378">typo in librustc_trans/collector.rs</a>.</li>
  <li><a href="https://github.com/osa1">@osa1</a> fixed <a href="https://github.com/rust-lang/rust/pull/39360">typos in libsyntax/tokenstream.rs</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> removed <a href="https://github.com/rust-lang/rust/pull/37057">all “consider using an explicit lifetime parameter” suggestions</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> rewrote <a href="https://github.com/rust-lang/rust/pull/39314">the first sentence in slice::sort</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/39344">a few links in the docs</a>.</li>
  <li><a href="https://github.com/king6cong">@king6cong</a> made a <a href="https://github.com/rust-lang/rust/pull/39321">doc comment typo fix</a>, reworded <a href="https://github.com/rust-lang/rust/pull/39267">a doc comment rewording</a> and improved <a href="https://github.com/rust-lang/rust/pull/39238">a comment wording</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/39139">to immutable arg/fields when trying to use as &amp;mut</a> and fixed <a href="https://github.com/rust-lang/rust/pull/39214">multiple labels when some don’t have message</a>.</li>
  <li><a href="https://github.com/das-g">@das-g</a> fixed <a href="https://github.com/rust-lang/rust/pull/39278">book: refer to <code class="highlighter-rouge">add_two</code> as “tested function”</a>.</li>
  <li><a href="https://github.com/ConnyOnny">@ConnyOnny</a> updted <a href="https://github.com/rust-lang/rust/pull/38794">match enum warning in The Book</a>.</li>
  <li><a href="https://github.com/Wilfred">@Wilfred</a> added <a href="https://github.com/rust-lang/rust/pull/39248">missing URL to release notes</a>.</li>
  <li><a href="https://github.com/Eijebong">@Eijebong</a> fixed <a href="https://github.com/rust-lang/rust/pull/39242">minor typo</a>.</li>
  <li><a href="https://github.com/utkarshkukreti">@utkarshkukreti</a> replaced <a href="https://github.com/rust-lang/rust/pull/38648">all <code class="highlighter-rouge">try!</code> with <code class="highlighter-rouge">?</code> in documentation examples</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/39224">missing urls for OsStr and OsString</a>, forced <a href="https://github.com/rust-lang/rust/pull/39222">backline on all where in docs</a>, removed <a href="https://github.com/rust-lang/rust/pull/39340">doc generation if doc comments only filled with ‘white’ characters</a>, added <a href="https://github.com/rust-lang/rust/pull/39306">note for E0117</a>, added <a href="https://github.com/rust-lang/rust/pull/38819">a distinct error code and description for “main function has wrong type”</a> and added <a href="https://github.com/rust-lang/rust/pull/39276">missing urls for array docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 1st of February 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>