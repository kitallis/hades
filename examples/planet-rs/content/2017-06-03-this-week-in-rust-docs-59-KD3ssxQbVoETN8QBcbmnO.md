+++
title = "This Week in Rust Docs 59"
date = "2017-06-03T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-59"
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

<p>After a long debate, it has been decided to keep hoedown testing/rendering by default in rustdoc. However, you can test pulldown by running rustdoc with <code class="highlighter-rouge">-Z unstable-options enable-commonmark</code>.</p>

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
  <li><a href="https://github.com/kennytm">@kennytm</a> introduced <a href="https://github.com/rust-lang/rust/pull/41968">‘ui-run’ test to compiletest</a>.</li>
  <li><a href="https://github.com/mjkillough">@mjkillough</a> documented <a href="https://github.com/rust-lang/rust/pull/42027">direct implementations on type aliases.</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/alex-ozdemir">@alex-ozdemir</a> made <a href="https://github.com/rust-lang/rust/pull/42076">clearer error message for Duplicate Definition</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/pwoolcoc">@pwoolcoc</a> added <a href="https://github.com/rust-lang/rust/pull/42219"><code class="highlighter-rouge">allow_fail</code> test attribute</a>.</li>
  <li><a href="https://github.com/photoszzt">@photoszzt</a> made <a href="https://github.com/rust-lang/rust/pull/42391">better suggestion for unknown method</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> made <a href="https://github.com/rust-lang/rust/pull/42307">rustdoc.js use license comments.</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> created <a href="https://github.com/rust-lang/rust/pull/42378">the Rustdoc book</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> hid <a href="https://github.com/rust-lang/rust/pull/42394"><code class="highlighter-rouge">self: Box&lt;Self&gt;</code> in list of deref methods in rustdoc</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> printed <a href="https://github.com/rust-lang/rust/pull/42304">the two types in the span label for transmute errors.</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> changed [vec<T> pronunciation](https://github.com/rust-lang/rust/pull/42385).</T></li>
  <li><a href="https://github.com/estebank">@estebank</a> used <a href="https://github.com/rust-lang/rust/pull/42383">multiline note for trait suggestion</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, added <a href="https://github.com/rust-lang/rust/pull/42361">E0602</a> and added <a href="https://github.com/rust-lang/rust/pull/42387">E0603 error code</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/gamazeps">@gamazeps</a> expanded <a href="https://github.com/rust-lang/rust/pull/41981"><code class="highlighter-rouge">detach</code> documentation in <code class="highlighter-rouge">thread::JoinHande</code></a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/41953">releases notes for 1.18</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> clarified <a href="https://github.com/rust-lang/rust/pull/42126">docs on implementing Into</a>.</li>
  <li><a href="https://github.com/tommyip">@tommyip</a> explained <a href="https://github.com/rust-lang/rust/pull/42196">why a closure is <code class="highlighter-rouge">FnOnce</code> in closure errors</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> unified <a href="https://github.com/rust-lang/rust/pull/42260">the docs of PartialEq, PartialOrd and Ord</a> and clarified <a href="https://github.com/rust-lang/rust/pull/42252">the docs for align_of and its variants</a>.</li>
  <li><a href="https://github.com/bjorn3">@bjorn3</a> added <a href="https://github.com/rust-lang/rust/pull/42355">syntax highlight rust code in librustc/dep_graph/README.md</a> and added <a href="https://github.com/rust-lang/rust/pull/42311">syntax highlight all rust code in librustc/traits/README.md</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> rewrote <a href="https://github.com/rust-lang/rust/pull/42372">a couple <code class="highlighter-rouge">Receiver</code> doc examples</a> and rewrote <a href="https://github.com/rust-lang/rust/pull/42347">doc examples for <code class="highlighter-rouge">Receiver::recv_timeout</code></a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> added <a href="https://github.com/rust-lang/rust/pull/42283">note regarding parent module containing use statement</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/42353">various book repos for the next release.</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/42338">releases notes for 1.18</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> added <a href="https://github.com/rust-lang/rust/pull/42370">[[T]] -&gt; [T] examples to SliceConcatExt docs</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> renamed <a href="https://github.com/rust-lang/rust/pull/42360"><code class="highlighter-rouge">Vector</code> and <code class="highlighter-rouge">FixedVector</code> to <code class="highlighter-rouge">Slice</code> and <code class="highlighter-rouge">Array</code> in rustdoc</a> and cleaned up <a href="https://github.com/rust-lang/rust/pull/42286">associated const value rendering</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> improved <a href="https://github.com/rust-lang/rust/pull/42319">error message for const extern fn</a>.</li>
  <li><a href="https://github.com/rap2hpoutre">@rap2hpoutre</a> fixed <a href="https://github.com/rust-lang/rust/pull/42329">links to “module-level documentation”</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> updated <a href="https://github.com/rust-lang/rust/pull/42180">rustdoc man page</a>, added <a href="https://github.com/rust-lang/rust/pull/42264">new error codes</a>, added <a href="https://github.com/rust-lang/rust/pull/42302">new error codes next</a> and fixed <a href="https://github.com/rust-lang/rust/pull/42318">signature by adding parens when needed in rustdoc</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 7th of June 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>