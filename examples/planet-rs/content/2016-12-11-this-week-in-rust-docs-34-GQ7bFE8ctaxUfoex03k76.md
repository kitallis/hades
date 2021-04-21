+++
title = "This Week in Rust Docs 34"
date = "2016-12-11T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-34"
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

<p>Since new rustc version is out, the controversial changes on docs have been merged:</p>

<ul>
  <li>Fold <a href="https://github.com/rust-lang/rust/pull/37728">fields for enum struct variants into a docblock in rustdoc</a></li>
  <li>Add <a href="https://github.com/rust-lang/rust/pull/37190">line breaks to where clauses a la rustfmt in rustdoc</a>.</li>
  <li>Print <a href="https://github.com/rust-lang/rust/pull/37134">more tags in rustdoc</a>.</li>
</ul>

<p>Don’t hesitate to give your feedbacks on them!</p>

<p>The way rustdoc is creating urls is problematic for the moment. A good summary of this issue can be found <a href="https://github.com/rust-lang/rust/issues/36417">here</a>. A few members of the Rust Doc team are preparing an RFC in order to improve this. If you want to get involved, feel free to speak about it with <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a> (imperio on IRC).</p>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/matklad">@matklad</a> advertised <a href="https://github.com/rust-lang/rust/pull/38297">Vec in LinkedList docs</a>.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> updated <a href="https://github.com/rust-lang/rust/pull/38216">docs of slice get() and friends</a>.</li>
  <li><a href="https://github.com/KiChjang">@KiChjang</a> displayed <a href="https://github.com/rust-lang/rust/pull/38057">better error messages for E0282</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> made a <a href="https://github.com/rust-lang/rust/pull/38215">minor fix about visibility in reference</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> escaped <a href="https://github.com/rust-lang/rust/pull/38244"> the deprecated and unstable reason text in rustdoc</a>, pointed <a href="https://github.com/rust-lang/rust/pull/38150">out the known type when field doesn’t satisfy bound</a>, provided <a href="https://github.com/rust-lang/rust/pull/38168">disambiguated syntax for candidates in E0034</a> and showed <a href="https://github.com/rust-lang/rust/pull/37493">span for trait that doesn’t implement Copy</a>.</li>
  <li><a href="https://github.com/federicomenaquintero">@federicomenaquintero</a> documented <a href="https://github.com/rust-lang/rust/pull/38247">the optional extra arguments to assert_eq!() / assert_ne!()</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> fixed <a href="https://github.com/rust-lang/rust/pull/38161">doctests with non-feature crate attrs in rustdoc</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/38264">broken src links from reexported items from macros in rustdoc</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/38208"><code class="highlighter-rouge">BTreeSet</code> documentation</a> and implemented <a href="https://github.com/rust-lang/rust/pull/38006"><code class="highlighter-rouge">fmt::Debug</code> for all structures in libstd</a>.</li>
  <li><a href="https://github.com/rkruppe">@rkruppe</a> used <a href="https://github.com/rust-lang/rust/pull/38138">abort() over loop {} for panic in the Book</a>.</li>
  <li><a href="https://github.com/Cobrand">@Cobrand</a> improved <a href="https://github.com/rust-lang/rust/pull/37941">and fixed mpsc documentation</a>.</li>
  <li><a href="https://github.com/sourcefrog">@sourcefrog</a> explained <a href="https://github.com/rust-lang/rust/pull/38158">meaning of Result iters and link to factory functions</a> and avoided <a href="https://github.com/rust-lang/rust/pull/38164">using locally installed Source Code Pro font (fixes #24355).</a>.</li>
  <li><a href="https://github.com/michael-zapata">@michael-zapata</a> harmonised <a href="https://github.com/rust-lang/rust/pull/38179">rustdoc error messages</a>.</li>
  <li><a href="https://github.com/wezm">@wezm</a> simplified <a href="https://github.com/rust-lang/rust/pull/38013">notes on testing and concurrency</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> improved <a href="https://github.com/rust-lang/rust/pull/38236">unix socket doc</a>, added <a href="https://github.com/rust-lang/rust/pull/38099">cast suggestions</a>, fix <a href="https://github.com/rust-lang/rust/pull/38255">invalid module suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/38067">more examples to UpdSocket</a> and added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> showed <a href="https://github.com/rust-lang/rust/pull/38065"><code class="highlighter-rouge">Trait</code> instead of <code class="highlighter-rouge">&lt;Struct as Trait&gt;</code> in E0323</a>.</li>
  <li><a href="https://github.com/jonathandturner">@jonathandturner</a> pointed <a href="https://github.com/rust-lang/rust/pull/38121">arg num mismatch errors back to their definition</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> added <a href="https://github.com/rust-lang/rust/pull/38105">sort lines in search index and implementors js in rustdoc</a>.</li>
  <li><a href="https://github.com/Cobrand">@Cobrand</a> updated <a href="https://github.com/rust-lang/rust/pull/38225">book/ffi to use catch_unwind</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/38186">docs for last undocumented <code class="highlighter-rouge">Default</code> <code class="highlighter-rouge">impl</code>.</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> fixed <a href="https://github.com/rust-lang/rust/pull/38163">reference definition of :tt</a>.</li>
  <li><a href="https://github.com/cardoe">@cardoe</a> fixed: <a href="https://github.com/rust-lang/rust/pull/38073">small typo in bootstrap/README</a>.</li>
  <li><a href="https://github.com/tarka">@tarka</a> made <a href="https://github.com/rust-lang/rust/pull/38112">a minor fix to testing concurrency section</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/38153">small typo</a>, added <a href="https://github.com/rust-lang/rust/pull/38123">missing examples for panicking objects</a>, added <a href="https://github.com/rust-lang/rust/pull/37780">checkup for return statement outside of a function</a>, added <a href="https://github.com/rust-lang/rust/pull/38151">examples for exit function</a>, add <a href="https://github.com/rust-lang/rust/pull/38189">missing links to Rc doc</a>, added <a href="https://github.com/rust-lang/rust/pull/37859">missing examples for Ipv6Addr</a>, added <a href="https://github.com/rust-lang/rust/pull/38020">part of missing UdpSocket’s urls and examples</a>, added <a href="https://github.com/rust-lang/rust/pull/38077">missing examples for IpAddr enum</a>, added <a href="https://github.com/rust-lang/rust/pull/38090">cloned example for Option</a> and added <a href="https://github.com/rust-lang/rust/pull/38141">Component examples</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 14th of December 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>