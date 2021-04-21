+++
title = "This Week in Rust Docs 60"
date = "2017-06-11T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-60"
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
  <li><a href="https://github.com/oli-obk">@oli-obk</a> changed <a href="https://github.com/rust-lang/rust/pull/42033">some notes into suggestions</a>.</li>
  <li><a href="https://github.com/alex-ozdemir">@alex-ozdemir</a> made <a href="https://github.com/rust-lang/rust/pull/42076">clearer error message for Duplicate Definition</a>.</li>
  <li><a href="https://github.com/pwoolcoc">@pwoolcoc</a> added <a href="https://github.com/rust-lang/rust/pull/42219"><code class="highlighter-rouge">allow_fail</code> test attribute</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> printed <a href="https://github.com/rust-lang/rust/pull/42304">the two types in the span label for transmute errors</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> used <a href="https://github.com/rust-lang/rust/pull/42572"><code class="highlighter-rouge">create_dir_all</code> to create output directory in rustdoc</a>.</li>
  <li><a href="https://github.com/maccoda">@maccoda</a> completed <a href="https://github.com/rust-lang/rust/pull/42579">env docs</a>.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> added <a href="https://github.com/rust-lang/rust/pull/42570">dedicated docstrings to Sum/Product impl of Result</a> and simplified <a href="https://github.com/rust-lang/rust/pull/42569">FromIterator example of Result</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/42503">releases notes for 1.19</a>.</li>
  <li><a href="https://github.com/ucarion">@ucarion</a> explicated <a href="https://github.com/rust-lang/rust/pull/42419">what “Rc” and “Arc” stand for</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, created <a href="https://github.com/rust-lang/rust/pull/42519">more error codes</a>, added <a href="https://github.com/rust-lang/rust/pull/42585">E0609</a> and added <a href="https://github.com/rust-lang/rust/pull/42568">0608</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> created <a href="https://github.com/rust-lang/rust/pull/42378">the Rustdoc book</a>.</li>
  <li><a href="https://github.com/mjkillough">@mjkillough</a> documented <a href="https://github.com/rust-lang/rust/pull/42027">direct implementations on type aliases</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/photoszzt">@photoszzt</a> made <a href="https://github.com/rust-lang/rust/pull/42391">better suggestion for unknown method</a>.</li>
  <li><a href="https://github.com/clarcharr">@clarcharr</a> made <a href="https://github.com/rust-lang/rust/pull/42307">rustdoc.js use license comments</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> hid <a href="https://github.com/rust-lang/rust/pull/42394"><code class="highlighter-rouge">self: Box&lt;Self&gt;</code> in list of deref methods in rustdoc</a> and renamed <a href="https://github.com/rust-lang/rust/pull/42360"><code class="highlighter-rouge">Vector</code> and <code class="highlighter-rouge">FixedVector</code> to <code class="highlighter-rouge">Slice</code> and <code class="highlighter-rouge">Array</code> in rustdoc</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> changed [vec<T> pronunciation](https://github.com/rust-lang/rust/pull/42385).</T></li>
  <li><a href="https://github.com/estebank">@estebank</a> used <a href="https://github.com/rust-lang/rust/pull/42383">multiline note for trait suggestion</a> and showed <a href="https://github.com/rust-lang/rust/pull/42362">trait method signature when impl differs</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> skipped <a href="https://github.com/rust-lang/rust/pull/42485">printing for skipped doc tests</a> and skipped <a href="https://github.com/rust-lang/rust/pull/42437">documentation files without ``` when running markdown tests</a>.</li>
  <li><a href="https://github.com/xfq">@xfq</a> updated <a href="https://github.com/rust-lang/rust/pull/42558">TRPL link in README.md</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> improved <a href="https://github.com/rust-lang/rust/pull/42551">cell doc example</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/42470">doc examples for <code class="highlighter-rouge">CString</code> methods</a> and improved <a href="https://github.com/rust-lang/rust/pull/42414"><code class="highlighter-rouge">Cow</code> method doc examples</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> updated <a href="https://github.com/rust-lang/rust/pull/42510">step_by docs to say iterator instead of range</a>.</li>
  <li><a href="https://github.com/citizen428">@citizen428</a> updated <a href="https://github.com/rust-lang/rust/pull/42469">doc for the assert macros</a>.</li>
  <li><a href="https://github.com/king6cong">@king6cong</a> reworded <a href="https://github.com/rust-lang/rust/pull/42438">doc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/42361">E0602</a> and added <a href="https://github.com/rust-lang/rust/pull/42387">E0603 error code</a>.</li>
  <li><a href="https://github.com/tommyip">@tommyip</a> only <a href="https://github.com/rust-lang/rust/pull/42580">emit one error for <code class="highlighter-rouge">use foo::self;</code></a> and better <a href="https://github.com/rust-lang/rust/pull/42443">closure error message</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 14th of June 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>