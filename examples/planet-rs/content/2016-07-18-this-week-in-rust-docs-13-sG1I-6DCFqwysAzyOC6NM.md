+++
title = "This Week in Rust Docs 13"
date = "2016-07-18T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-13"
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

<p>The <a href="https://github.com/rust-lang/rfcs/pull/1574">“More api documentation conventions” RFC</a> has been merged!</p>

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
  <li><a href="https://github.com/durka">@durka</a> documented <a href="https://github.com/rust-lang/rust/pull/34732">DoubleEndedIterator::next_back</a>.</li>
  <li><a href="https://github.com/shepmaster">@shepmaster</a> improved <a href="https://github.com/rust-lang/rust/pull/34884">{String,Vec}::from_raw_parts documentation</a>.</li>
  <li><a href="https://github.com/xitep">@xitep</a> made <a href="https://github.com/rust-lang/rust/pull/34880"><code class="highlighter-rouge">.enumerate()</code> example self-explanatory</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> indicated <a href="https://github.com/rust-lang/rust/pull/34875">where <code class="highlighter-rouge">std::slice</code> structs originate from</a> and partially <a href="https://github.com/rust-lang/rust/pull/34853">rewrote/expanded <code class="highlighter-rouge">Vec::truncate</code> documentation</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> clarified <a href="https://github.com/rust-lang/rust/pull/34520">UnsafeCell docs</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for missplaced doc comments</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/34855">examples for VecDeque</a> and added <a href="https://github.com/rust-lang/rust/pull/34854">examples for LinkedList</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/34752">methods in seach results in rustdoc</a>.</li>
  <li><a href="https://github.com/alexandermerritt">@alexandermerritt</a> made <a href="https://github.com/rust-lang/rust/pull/34745">docs for <code class="highlighter-rouge">clone_from_slice</code> consistent with <code class="highlighter-rouge">copy_from_slice</code></a>.</li>
  <li><a href="https://github.com/izgzhen">@izgzhen</a> improved <a href="https://github.com/rust-lang/rust/pull/34733">arc doc</a>.</li>
  <li><a href="https://github.com/davidko">@davidko</a> fixed <a href="https://github.com/rust-lang/rust/pull/34770">a few typos in The Book</a>.</li>
  <li><a href="https://github.com/glandium">@glandium</a> added <a href="https://github.com/rust-lang/rust/pull/34777">a mention to the fact that writeln! and println! always use LF</a>.</li>
  <li><a href="https://github.com/wuranbo">@wuranbo</a> fixed <a href="https://github.com/rust-lang/rust/pull/34799">ffi referenced rust-snappy which couldn’t compile</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/34818">where <code class="highlighter-rouge">std::vec</code> structs originate from</a>, made <a href="https://github.com/rust-lang/rust/pull/34737">various <code class="highlighter-rouge">std::process</code> doc</a> and added <a href="https://github.com/rust-lang/rust/pull/34794">doc example for <code class="highlighter-rouge">std::process::ExitStatus::success</code></a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> fixed up <a href="https://github.com/rust-lang/rust/pull/34838">documentation around no_std</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> made a few doc <a href="https://github.com/rust-lang/rust/pulls?utf8=%E2%9C%93&amp;q=is%3Apr%20is%3Aclosed%2034849%2034848">fixes</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/34804">examples for FpCategory</a> and added <a href="https://github.com/rust-lang/rust/pull/34637">libsyntax error code explanations</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 20th of July 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>