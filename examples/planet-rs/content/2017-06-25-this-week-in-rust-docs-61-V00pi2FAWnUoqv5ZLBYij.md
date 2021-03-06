+++
title = "This Week in Rust Docs 61"
date = "2017-06-25T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-61"
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
  <li><a href="https://github.com/pwoolcoc">@pwoolcoc</a> added <a href="https://github.com/rust-lang/rust/pull/42219"><code class="highlighter-rouge">allow_fail</code> test attribute</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/42503">releases notes for 1.19</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/42865">a few issues with associated consts in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/42885">ICE on <code class="highlighter-rouge">use *;</code> in rustdoc</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> fixed <a href="https://github.com/rust-lang/rust/pull/42624">a couple broken links to the reference from error messages</a>.</li>
  <li><a href="https://github.com/rthomas">@rthomas</a> updated <a href="https://github.com/rust-lang/rust/pull/42831">docs for fmt::write</a>, updated <a href="https://github.com/rust-lang/rust/pull/42837">docs on Error struct</a>, updated <a href="https://github.com/rust-lang/rust/pull/42836">docs for Debug* structs</a> and updated <a href="https://github.com/rust-lang/rust/pull/42832">docs for std::fmt::format</a>.</li>
  <li><a href="https://github.com/cengizIO">@cengizIO</a> added <a href="https://github.com/rust-lang/rust/pull/42732">pager support for <code class="highlighter-rouge">rustc --explain EXXXX</code></a>.</li>
  <li><a href="https://github.com/Fourchaux">@Fourchaux</a> fixed <a href="https://github.com/rust-lang/rust/pull/42812">basic typos in Doc Comments</a>.</li>
  <li><a href="https://github.com/nagisa">@nagisa</a> fix <a href="https://github.com/rust-lang/rust/pull/42431">NaN handling in is_sign_negative/positive</a> and specialisation <a href="https://github.com/rust-lang/rust/pull/42534">of Iterator methods for Range</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> adding <a href="https://github.com/rust-lang/rust/pull/42669">diagnostic code for lifetime errors with one named, one anonymous lifetime parameter</a>.</li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/42670">hint about the return code of panic!</a>.</li>
  <li><a href="https://github.com/Emilgardis">@Emilgardis</a> fixed <a href="https://github.com/rust-lang/rust/pull/42270">wrong report on closure args mismatch when a ref is expected but not found</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, created <a href="https://github.com/rust-lang/rust/pull/42519">more error codes</a> and remove <a href="https://github.com/rust-lang/rust/pull/42887">err methods</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/alex-ozdemir">@alex-ozdemir</a> made <a href="https://github.com/rust-lang/rust/pull/42076">clearer error message for Duplicate Definition</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> printed <a href="https://github.com/rust-lang/rust/pull/42304">the two types in the span label for transmute errors</a> and made <a href="https://github.com/rust-lang/rust/pull/42804">rustc errors colorful</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> used <a href="https://github.com/rust-lang/rust/pull/42572"><code class="highlighter-rouge">create_dir_all</code> to create output directory in rustdoc</a>, fixed <a href="https://github.com/rust-lang/rust/pull/42806">compiler docs yet again</a>, and linked <a href="https://github.com/rust-lang/rust/pull/42594">directly to associated types in rustdoc</a>.</li>
  <li><a href="https://github.com/maccoda">@maccoda</a> completed <a href="https://github.com/rust-lang/rust/pull/42579">env docs</a>.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> added <a href="https://github.com/rust-lang/rust/pull/42570">dedicated docstrings to Sum/Product impl of Result</a> and simplified <a href="https://github.com/rust-lang/rust/pull/42569">FromIterator example of Result</a>.</li>
  <li><a href="https://github.com/ucarion">@ucarion</a> explicated <a href="https://github.com/rust-lang/rust/pull/42419">what “Rc” and “Arc” stand for</a>.</li>
  <li><a href="https://github.com/cramertj">@cramertj</a> changed <a href="https://github.com/rust-lang/rust/pull/42833">span label for E0435</a>.</li>
  <li><a href="https://github.com/letheed">@letheed</a> fixed <a href="https://github.com/rust-lang/rust/pull/42825">ref as mutable ref in std::rc::Rc doc</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/42787">extended information for E0562; impl Trait can only be a return type</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> removed <a href="https://github.com/rust-lang/rust/pull/42777">most “```ignore” doc tests.</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> made <a href="https://github.com/rust-lang/rust/pull/42749">additions/improvements for doc examples</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> added <a href="https://github.com/rust-lang/rust/pull/42620">compile_error!</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> removed <a href="https://github.com/rust-lang/rust/pull/42722">SUMMARY.md of the unstable book as its autogenerated</a> and introduced <a href="https://github.com/rust-lang/rust/pull/42705">tidy lint to check for inconsistent tracking issues</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/42585">E0609</a>, added <a href="https://github.com/rust-lang/rust/pull/42568">0608</a>, new <a href="https://github.com/rust-lang/rust/pull/42614">error codes</a> and error <a href="https://github.com/rust-lang/rust/pull/42654">codes new</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 28th of June 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>