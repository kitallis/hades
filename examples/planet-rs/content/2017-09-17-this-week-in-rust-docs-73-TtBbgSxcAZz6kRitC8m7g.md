+++
title = "This Week in Rust Docs 73"
date = "2017-09-17T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-73"
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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> hid <a href="https://github.com/rust-lang/rust/pull/44026">internal types/traits from std docs via new #[doc(masked)] attribute</a> and made <a href="https://github.com/rust-lang/rust/pull/44613">rustdoc optimizations</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> added <a href="https://github.com/rust-lang/rust/pull/44124">E0623 for return types - both parameters are anonymous</a> and extended <a href="https://github.com/rust-lang/rust/pull/44549">E0623 for earlybound and latebound for structs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> deprecated <a href="https://github.com/rust-lang/rust/pull/44138">several flags in rustdoc</a>.</li>
  <li><a href="https://github.com/laumann">@laumann</a> added <a href="https://github.com/rust-lang/rust/pull/44297">suggestions for misspelled method names</a>.</li>
  <li><a href="https://github.com/zackmdavis">@zackmdavis</a> added <a href="https://github.com/rust-lang/rust/pull/44103">comparison operators to must-use lint (under <code class="highlighter-rouge">fn_must_use</code> feature)</a>.</li>
  <li><a href="https://github.com/bluss">@bluss</a> documented <a href="https://github.com/rust-lang/rust/pull/44651">thread builder panics for nul bytes in thread names</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> removed <a href="https://github.com/rust-lang/rust/pull/44215">suggestion of placing <code class="highlighter-rouge">use</code> statements into expanded code</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> expanded <a href="https://github.com/rust-lang/rust/pull/44648">size_of docs</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/44625">how ChildStd{in,out,err} FDs are closed</a> and fixed <a href="https://github.com/rust-lang/rust/pull/44622">incorrect <code class="highlighter-rouge">into_inner</code> link in docs</a>.</li>
  <li><a href="https://github.com/eddyb">@eddyb</a> pretty-printed <a href="https://github.com/rust-lang/rust/pull/44562">unevaluated expressions in types</a>.</li>
  <li><a href="https://github.com/nikomatsakis">@nikomatsakis</a> reworked <a href="https://github.com/rust-lang/rust/pull/44505">the README.md for rustc and add other readmes</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/44481">RELEASES.md for 1.21.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/43870">deref suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/44165">display of cfg in rustdoc</a>, updated <a href="https://github.com/rust-lang/rust/pull/44397">codeblock color</a>, removed <a href="https://github.com/rust-lang/rust/pull/44350">small id false positive in rustdoc html diff</a>, added <a href="https://github.com/rust-lang/rust/pull/44636">short error message-format</a> and added <a href="https://github.com/rust-lang/rust/pull/44554">pub visibility for methods as well</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> extended <a href="https://github.com/rust-lang/rust/pull/44079">E0623 for LateBound and EarlyBound Regions</a> and extended <a href="https://github.com/rust-lang/rust/pull/44516">E0623 for fn items</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/44430">mdbook</a>.</li>
  <li><a href="https://github.com/tommyip">@tommyip</a> added <a href="https://github.com/rust-lang/rust/pull/44453">doc example to String::as_mut_str</a>, added <a href="https://github.com/rust-lang/rust/pull/44449">doc example to String::as_str</a> and added <a href="https://github.com/rust-lang/rust/pull/44497">doc example to str::from_boxed_utf8_unchecked</a>.</li>
  <li><a href="https://github.com/smt923">@smt923</a> added <a href="https://github.com/rust-lang/rust/pull/44472">short doc examples for str::from_utf8_mut</a>.</li>
  <li><a href="https://github.com/toidiu">@toidiu</a> updated <a href="https://github.com/rust-lang/rust/pull/44467">documentation to demonstrate mutability</a>.</li>
  <li><a href="https://github.com/napen123">@napen123</a> added <a href="https://github.com/rust-lang/rust/pull/44457">doc examples for str::as_bytes_mut</a> and added <a href="https://github.com/rust-lang/rust/pull/44477">doc examples to str::from_utf8_unchecked_mut</a>.</li>
  <li><a href="https://github.com/frehberg">@frehberg</a> extended <a href="https://github.com/rust-lang/rust/pull/44378">UdpSocket API doc</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> moved <a href="https://github.com/rust-lang/rust/pull/44413">the man directory to a subdirectory</a>, used <a href="https://github.com/rust-lang/rust/pull/44569">“avoid” instead of “disable” because it’s a better word there</a> and fixed <a href="https://github.com/rust-lang/rust/pull/44386">mispositioned error indicators</a>.</li>
  <li><a href="https://github.com/joshlf">@joshlf</a> documented <a href="https://github.com/rust-lang/rust/pull/44396">std::thread::LocalKey limitation with initializers</a>.</li>
  <li><a href="https://github.com/jonhoo">@jonhoo</a> mentionned <a href="https://github.com/rust-lang/rust/pull/44609">that HashMap::new and HashSet::new do not allocate</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/44368">double count of ids when using –enable-commonmark</a>.</li>
  <li><a href="https://github.com/rwakulszowa">@rwakulszowa</a> added <a href="https://github.com/rust-lang/rust/pull/44521">an example of std::str::encode_utf16</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/44572">return type of <code class="highlighter-rouge">String::from_utf16_lossy</code></a>.</li>
  <li><a href="https://github.com/adlerd">@adlerd</a> fixed <a href="https://github.com/rust-lang/rust/pull/44534">drain_filter doctest</a>.</li>
  <li><a href="https://github.com/Havvy">@Havvy</a> fixed <a href="https://github.com/rust-lang/rust/pull/44536">example in transmute</a>.</li>
  <li><a href="https://github.com/42triangles">@42triangles</a> added <a href="https://github.com/rust-lang/rust/pull/44485">an example for <code class="highlighter-rouge">std::str::into_boxed_bytes()</code></a>.</li>
  <li><a href="https://github.com/carols10cents">@carols10cents</a> updated <a href="https://github.com/rust-lang/rust/pull/44476">label explanations</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> clarified <a href="https://github.com/rust-lang/rust/pull/44388">the behavior of UDP sockets wrt. multiple addresses in <code class="highlighter-rouge">connect</code></a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilised <a href="https://github.com/rust-lang/rust/pull/43949">compile_fail</a>, fixed <a href="https://github.com/rust-lang/rust/pull/44254">rendering of const keyword for functions</a>, reduced <a href="https://github.com/rust-lang/rust/pull/44347">false positives number in rustdoc html diff</a> and updated <a href="https://github.com/rust-lang/rust/pull/44541">openoptions docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 20th of September 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>