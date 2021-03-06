+++
title = "This Week in Rust Docs 54"
date = "2017-04-30T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-54"
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

<p>The jquery dependency <a href="https://github.com/rust-lang/rust/pull/41307">is being removed</a> from the rustdoc javascript. When navigating in the docs, please check if everything’s working as expected!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/labels/T-doc">T-doc</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/40857">at fields that make the type recursive</a>, emitted <a href="https://github.com/rust-lang/rust/pull/41434">diagnostic when using <code class="highlighter-rouge">const</code> storing <code class="highlighter-rouge">Fn</code> as pattern</a>, cleaned <a href="https://github.com/rust-lang/rust/pull/41488">up callable type mismatch errors</a>, made <a href="https://github.com/rust-lang/rust/pull/41489">unsatisfied trait bounds note multiline</a> and used <a href="https://github.com/rust-lang/rust/pull/41520">diagnostics for trace_macro instead of println</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/icefoxen">@icefoxen</a> made <a href="https://github.com/rust-lang/rust/pull/40719">a tiny update to rustdoc CSS</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> updated <a href="https://github.com/rust-lang/rust/pull/41217">docs of ‘fence’</a>.</li>
  <li><a href="https://github.com/abonander">@abonander</a> documented <a href="https://github.com/rust-lang/rust/pull/41476">the <code class="highlighter-rouge">proc_macro</code> feature in the Unstable Book</a>.</li>
  <li><a href="https://github.com/ranma42">@ranma42</a> generate <a href="https://github.com/rust-lang/rust/pull/41600">XZ-compressed tarballs</a>.</li>
  <li><a href="https://github.com/alexeyzab">@alexeyzab</a> fixed <a href="https://github.com/rust-lang/rust/pull/41547">error message for mismatched types</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/41612">generic example of std::ops::Add in doc comments</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/41548">release notes for 1.17</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> improved [docs on Arc<T> and Send/Sync](https://github.com/rust-lang/rust/pull/41536) and added [more ways to create a PathBuf to docs](https://github.com/rust-lang/rust/pull/41531).</T></li>
  <li><a href="https://github.com/z1mvader">@z1mvader</a> rewrote <a href="https://github.com/rust-lang/rust/pull/41543">the thread struct docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/41307">jquery dependency</a>, reduced <a href="https://github.com/rust-lang/rust/pull/41384">HTML output in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/41559">better error message when == operator is badly used</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> removes <a href="https://github.com/rust-lang/rust/pull/41433">display of <code class="highlighter-rouge">::</code> on tuple struct diagnostics</a> and pointed <a href="https://github.com/rust-lang/rust/pull/41523">at variable moved by closure</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> specified <a href="https://github.com/rust-lang/rust/pull/41442">behavior of <code class="highlighter-rouge">write_all</code> for <code class="highlighter-rouge">ErrorKind::Interrupted</code> errors</a> and fixed <a href="https://github.com/rust-lang/rust/pull/41518">a copy-paste error in <code class="highlighter-rouge">Instant::sub_duration</code></a>.</li>
  <li><a href="https://github.com/projektir">@projektir</a> added <a href="https://github.com/rust-lang/rust/pull/41438">links and examples for various mspc pages</a>.</li>
  <li><a href="https://github.com/hsivonen">@hsivonen</a> explained <a href="https://github.com/rust-lang/rust/pull/41602">why zero-length slices require a non-null pointer</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> bumped <a href="https://github.com/rust-lang/rust/pull/41572">mdbook dep to pick up new ‘create missing’ toggle feature.</a>.</li>
  <li><a href="https://github.com/moosingin3space">@moosingin3space</a> explained <a href="https://github.com/rust-lang/rust/pull/41636">process::exit in mem::forget docs</a>.</li>
  <li><a href="https://github.com/cuviper">@cuviper</a> fixed <a href="https://github.com/rust-lang/rust/pull/41613">links in RELEASES.md for 1.10.0 through 1.12.0</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> cleaned <a href="https://github.com/rust-lang/rust/pull/41526">up TcpStream example</a>, addressed <a href="https://github.com/rust-lang/rust/pull/41499">platform-specific behavior in TcpStream::shutdown</a>, fixed <a href="https://github.com/rust-lang/rust/pull/41535">up vec guarantee around capacity</a>, clarified <a href="https://github.com/rust-lang/rust/pull/41528">“side effect” in peek’s docs</a>, clarified <a href="https://github.com/rust-lang/rust/pull/41527">the doc index</a> and used <a href="https://github.com/rust-lang/rust/pull/41500">the word ‘length’ in Vec::len’s docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/40634">more explanation on RefCell::get_mut</a> and improved <a href="https://github.com/rust-lang/rust/pull/41501">error message for invalid module location</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 3rd of May 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>