+++
title = "This Week in Rust Docs 14"
date = "2016-07-25T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-14"
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

<p>This week’s edition was edited by: <a href="https://github.com/GuillaumeGomez">GuillaumeGomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>A topic to propose crates for the Rust Doc Days has been created <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>.</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They both need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 50 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on github!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/qolop">@qolop</a> fixed <a href="https://github.com/rust-lang/rust/pull/34941">typo (privledge-&gt;privilege)</a>.</li>
  <li><a href="https://github.com/jhod0">@jhod0</a> added <a href="https://github.com/rust-lang/rust/pull/34970">diagnostics for rustc_metadata</a>.</li>
  <li><a href="https://github.com/abhijeetbhagat">@abhijeetbhagat</a> updated <a href="https://github.com/rust-lang/rust/pull/34990">underscore usage</a> and updated <a href="https://github.com/rust-lang/rust/pull/34974">VecDeque documentation to specify direction of index 0</a>.</li>
  <li><a href="https://github.com/rdwilliamson">@rdwilliamson</a> fixed <a href="https://github.com/rust-lang/rust/pull/35001">HashMap’s values_mut example to use println!</a>.</li>
  <li><a href="https://github.com/nrc">@nrc</a> simplified <a href="https://github.com/rust-lang/rust/pull/35020">rustdoc URLs</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> documented <a href="https://github.com/rust-lang/rust/pull/34732">DoubleEndedIterator::next_back</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> clarified <a href="https://github.com/rust-lang/rust/pull/34520">UnsafeCell docs</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for missplaced doc comments</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/35003">item types from some title pages from rustdoc</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> rewrote/expanded <a href="https://github.com/rust-lang/rust/pull/35019"><code class="highlighter-rouge">slice::split</code> doc examples</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a>, improved <a href="https://github.com/rust-lang/rust/pull/35009"><code class="highlighter-rouge">DirEntry</code> doc</a>, added <a href="https://github.com/rust-lang/rust/pull/34995">DirBuilder doc examples</a>, added <a href="https://github.com/rust-lang/rust/pull/34935">HashMap Entry enums examples</a> and improved <a href="https://github.com/rust-lang/rust/pull/35010"><code class="highlighter-rouge">Open</code> doc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/mark-buer">@mark-buer</a> removed <a href="https://github.com/rust-lang/rust/pull/34895">rustdoc reference to <code class="highlighter-rouge">walk_dir</code></a>.</li>
  <li><a href="https://github.com/wettowelreactor">@wettowelreactor</a> fixed <a href="https://github.com/rust-lang/rust/pull/34977">some typos in char.rs</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> rewrote/expanded <a href="https://github.com/rust-lang/rust/pull/34911">doc examples for <code class="highlighter-rouge">Vec::set_len</code></a>, improved <a href="https://github.com/rust-lang/rust/pull/34988">doc examples for <code class="highlighter-rouge">slice::windows</code></a>, added <a href="https://github.com/rust-lang/rust/pull/34930">doc examples for <code class="highlighter-rouge">Vec::{as_slice,as_mut_slice}</code></a>, indicated <a href="https://github.com/rust-lang/rust/pull/34875">where <code class="highlighter-rouge">std::slice</code> structs originate from</a> and partially <a href="https://github.com/rust-lang/rust/pull/34853">rewrote/expanded <code class="highlighter-rouge">Vec::truncate</code> documentation</a>.</li>
  <li><a href="https://github.com/xitep">@xitep</a> made <a href="https://github.com/rust-lang/rust/pull/34880"><code class="highlighter-rouge">.enumerate()</code> example self-explanatory</a>.</li>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> improved <a href="https://github.com/rust-lang/rust/pull/34884">{String,Vec}::from_raw_parts documentation</a>.
 <a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/34921">Fix unwanted top margin for toggle wrapper in rustdoc CSS</a>, added <a href="https://github.com/rust-lang/rust/pull/34919">doc for btree_map types</a>, added <a href="https://github.com/rust-lang/rust/pull/34976"><code class="highlighter-rouge">BuildHasher</code> examples</a>, added <a href="https://github.com/rust-lang/rust/pull/34975"><code class="highlighter-rouge">RandomState</code> doc</a>, added <a href="https://github.com/rust-lang/rust/pull/34855">examples for VecDeque</a> and added <a href="https://github.com/rust-lang/rust/pull/34854">examples for LinkedList</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 27th of July 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>