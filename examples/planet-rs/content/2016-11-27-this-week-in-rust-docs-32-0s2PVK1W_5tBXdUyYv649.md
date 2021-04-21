+++
title = "This Week in Rust Docs 32"
date = "2016-11-27T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-32"
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
  <li><a href="https://github.com/alygin">@alygin</a> fixed <a href="https://github.com/rust-lang/rust/pull/38007">error explanation formatting</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> got <a href="https://github.com/rust-lang/rust/pull/37911">back missing crate-name when –playground-url is used in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/37910">suggestions “#[macro_use]” for all undefined macros</a>.</li>
  <li><a href="https://github.com/sourcefrog">@sourcefrog</a> documented <a href="https://github.com/rust-lang/rust/pull/38018">that Process::command will search the PATH</a> and made <a href="https://github.com/rust-lang/rust/pull/38019">a clearer description of std::path::MAIN_SEPARATOR</a>.</li>
  <li><a href="https://github.com/Cobrand">@Cobrand</a> improved <a href="https://github.com/rust-lang/rust/pull/37941">and fixed mpsc documentation</a>.</li>
  <li><a href="https://github.com/wezm">@wezm</a> simplified <a href="https://github.com/rust-lang/rust/pull/38013">notes on testing and concurrency</a>.</li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a> added <a href="https://github.com/rust-lang/rust/pull/37863">hint to fix error for immutable ref in arg</a>.</li>
  <li><a href="https://github.com/sanxiyn">@sanxiyn</a> warned <a href="https://github.com/rust-lang/rust/pull/37946">unused type parameters</a> and warned <a href="https://github.com/rust-lang/rust/pull/37631">unused type aliases</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> showed <a href="https://github.com/rust-lang/rust/pull/37369">multiline spans in full if short enough</a> and detected <a href="https://github.com/rust-lang/rust/pull/36409">missing <code class="highlighter-rouge">;</code> on methods with return type <code class="highlighter-rouge">()</code></a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> replaced <a href="https://github.com/rust-lang/rust/pull/37934">rustup.sh with rustup.rs in The Book</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37983">examples for TcpListener struct</a>, added <a href="https://github.com/rust-lang/rust/pull/38020">part of missing UdpSocket’s urls and examples</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a>, added <a href="https://github.com/rust-lang/rust/pull/37780">checkup for return statement outside of a function</a>, added <a href="https://github.com/rust-lang/rust/pull/37859">missing examples for Ipv6Addr</a>, started <a href="https://github.com/rust-lang/rust/pull/37388">of implementation of proposal for E0308</a>, added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a> and removed <a href="https://github.com/rust-lang/rust/pull/37870">unneeded tricky macro doc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/eddyb">@eddyb</a> separated <a href="https://github.com/rust-lang/rust/pull/37890">test collection from the main “clean”-ing pipeline in rustdoc</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> provided <a href="https://github.com/rust-lang/rust/pull/37442">hint when cast needs a dereference</a>.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> fixed <a href="https://github.com/rust-lang/rust/pull/37876">duplicate bullet points in feature list</a>.</li>
  <li><a href="https://github.com/brcooley">@brcooley</a> fixed <a href="https://github.com/rust-lang/rust/pull/37840">grammar error in lifetimes.md</a>.</li>
  <li><a href="https://github.com/ojsheikh">@ojsheikh</a> updated <a href="https://github.com/rust-lang/rust/pull/37835">E0088 to new error format</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> clarified <a href="https://github.com/rust-lang/rust/pull/37836">the reference’s status.</a>.</li>
  <li><a href="https://github.com/sfackler">@sfackler</a> fixed <a href="https://github.com/rust-lang/rust/pull/37963">two small issues in iterator docs</a>.</li>
  <li><a href="https://github.com/vickenty">@vickenty</a> followed <a href="https://github.com/rust-lang/rust/pull/38001">our own recommendations in the examples</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> documented <a href="https://github.com/rust-lang/rust/pull/38010">how lock ‘guard’ structures are created.</a>.</li>
  <li><a href="https://github.com/fkjogu">@fkjogu</a> defined <a href="https://github.com/rust-lang/rust/pull/37978"><code class="highlighter-rouge">bound</code> argument in std::sync::mpsc::sync_channel in the documentation</a>.</li>
  <li><a href="https://github.com/samestep">@samestep</a> replaced <a href="https://github.com/rust-lang/rust/pull/37961">“radicum” with “radices”</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37880">missing examples in SocketAddr</a>, added <a href="https://github.com/rust-lang/rust/pull/38004">missing urls and examples to TcpStream</a> and added <a href="https://github.com/rust-lang/rust/pull/37962">missing examples to SocketAddrV6</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 30th of November 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>