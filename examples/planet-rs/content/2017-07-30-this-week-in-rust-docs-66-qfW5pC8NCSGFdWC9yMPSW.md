+++
title = "This Week in Rust Docs 66"
date = "2017-07-30T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-66"
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

<p><a href="https://github.com/steveklabnik">@steveklabnik</a> ended the first version of <a href="https://github.com/steveklabnik/rustdoc">the rewrite of rustdoc</a> using RLS. It’s far from done but don’t hesitate to give it a try!</p>

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
  <li><a href="https://github.com/RalfJung">@RalfJung</a> clarified <a href="https://github.com/rust-lang/rust/pull/43176">wording for E0122</a>.</li>
  <li><a href="https://github.com/xliiv">@xliiv</a> added <a href="https://github.com/rust-lang/rust/pull/43423">simple docs example for struct Cell</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> exposed <a href="https://github.com/rust-lang/rust/pull/43348">all OS-specific modules in libstd doc</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> printed <a href="https://github.com/rust-lang/rust/pull/43515">associated types in traits “implementors” section in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/43529">documentation for function pointers as a primitive</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> made <a href="https://github.com/rust-lang/rust/pull/43528">rustdoc build only at the topmost stage</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a> and removed <a href="https://github.com/rust-lang/rust/pull/43397">warn on unused field on union</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> adjusted <a href="https://github.com/rust-lang/rust/pull/43386">new suggestions to the suggestion guidelines</a>.</li>
  <li><a href="https://github.com/waywardmonkeys">@waywardmonkeys</a> fixed <a href="https://github.com/rust-lang/rust/pull/43428">some doc comment typos</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/43413">generic example of std::ops::Sub in doc comments</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> clarified <a href="https://github.com/rust-lang/rust/pull/43374">that sort_unstable is deterministic</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made <a href="https://github.com/rust-lang/rust/pull/43409">into_iter example more concise</a>.</li>
  <li><a href="https://github.com/cuviper">@cuviper</a> corrected <a href="https://github.com/rust-lang/rust/pull/43401">the spelling of “homogeneous”</a>.</li>
  <li><a href="https://github.com/s3rvac">@s3rvac</a> added <a href="https://github.com/rust-lang/rust/pull/43379">a missing verb to the description of std::process::ExitStatus::success()</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/43509">[src] links to associated functions inside an impl block in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/43455">a note to Vec’s Extend&lt;&amp;T&gt; impl about its slice specialization</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> changed <a href="https://github.com/rust-lang/rust/pull/43541">E0623 error message - both anonymous lifetime regions</a> and improved <a href="https://github.com/rust-lang/rust/pull/43298">case with both anonymous lifetime parameters #43269</a>.</li>
  <li><a href="https://github.com/pczarn">@pczarn</a> made <a href="https://github.com/rust-lang/rust/pull/43432">the macro parser theory description more accurate</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> made <a href="https://github.com/rust-lang/rust/pull/43489">better diagnostics and recovery for <code class="highlighter-rouge">mut ref</code> in patterns</a>.</li>
  <li><a href="https://github.com/joshlf">@joshlf</a> fixed <a href="https://github.com/rust-lang/rust/pull/43456">grammar in std::thread::spawn documentation</a>.</li>
  <li><a href="https://github.com/ivanbakel">@ivanbakel</a> extended <a href="https://github.com/rust-lang/rust/pull/43479">error message for mut borrow conflicts in loops</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> made <a href="https://github.com/rust-lang/rust/pull/43445">major section headers self-links in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/43446">unions to whitelist of sidebar types in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/43436">layout of Fields section in documentation for unions</a>.</li>
  <li><a href="https://github.com/leshow">@leshow</a> fixed docs: <a href="https://github.com/rust-lang/rust/pull/43366">BufReader/File doesn’t need to be mut</a>.</li>
  <li><a href="https://github.com/ranweiler">@ranweiler</a> documented <a href="https://github.com/rust-lang/rust/pull/43342">usage of <code class="highlighter-rouge">compiler_builtins</code> with <code class="highlighter-rouge">no_std</code> binaries</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/43009">lint for when doc comments are added where they’re unused</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 2nd of August 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>