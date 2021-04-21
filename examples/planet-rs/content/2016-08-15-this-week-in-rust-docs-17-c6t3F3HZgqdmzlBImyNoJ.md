+++
title = "This Week in Rust Docs 17"
date = "2016-08-15T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-17"
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

<p>The <a href="https://github.com/rust-lang/rfcs/pull/1683#issuecomment-237384575">doc team RFC</a> has been merged. The rust doc team is now official! Take a look <a href="https://www.rust-lang.org/en-US/team.html#Documentation-team">here</a>.</p>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>A new RFC has been opened: <a href="https://github.com/rust-lang/rfcs/pull/1687">Add API documentation front page styleguide</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/35233">Error code list which need to be updated to new format</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 50 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on github!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/jhod0">@jhod0</a> added <a href="https://github.com/rust-lang/rust/pull/34970">diagnostics for rustc_metadata</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for misplaced doc comments</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/35003">item types from some title pages from rustdoc</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">the reference isn’t normative</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> centered <a href="https://github.com/rust-lang/rust/pull/35682">content of the generated docs</a>, added <a href="https://github.com/rust-lang/rust/pull/35680">new error code tests</a> and fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a></li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week, I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/munyari">@munyari</a>: <a href="https://github.com/rust-lang/rust/pull/35470">E0214</a>, <a href="https://github.com/rust-lang/rust/pull/35537">E0038</a></li>
  <li><a href="https://github.com/garekkream">@garekkream</a>: <a href="https://github.com/rust-lang/rust/pull/35524">E0162</a>, <a href="https://github.com/rust-lang/rust/pull/35644">E0302</a>, <a href="https://github.com/rust-lang/rust/pull/35643">E0301</a></li>
  <li><a href="https://github.com/hank-der-hafenarbeiter">@hank-der-hafenarbeiter</a>: <a href="https://github.com/rust-lang/rust/pull/35507">E0221</a>, <a href="https://github.com/rust-lang/rust/pull/35541">E0045</a>, <a href="https://github.com/rust-lang/rust/pull/35536">E0433</a></li>
  <li><a href="https://github.com/razielgn">@razielgn</a>: <a href="https://github.com/rust-lang/rust/pull/35504">E0026</a></li>
  <li><a href="https://github.com/srdja">@srdja</a>: <a href="https://github.com/rust-lang/rust/pull/35530">E0007 and E0008</a></li>
  <li><a href="https://github.com/Vassah">@Vassah</a>: <a href="https://github.com/rust-lang/rust/pull/35528">E0091 and E0092</a></li>
  <li><a href="https://github.com/clementmiao">@clementmiao</a>: <a href="https://github.com/rust-lang/rust/pull/35616">E0067</a>, <a href="https://github.com/rust-lang/rust/pull/35615">E0070</a></li>
  <li><a href="https://github.com/crypto-universe">@crypto-universe</a>: <a href="https://github.com/rust-lang/rust/pull/35596">E0254</a></li>
  <li><a href="https://github.com/shyaamsundhar">@shyaamsundhar</a>: <a href="https://github.com/rust-lang/rust/pull/35586">E0248, E0267 and E0268</a></li>
  <li><a href="https://github.com/circuitfox">@circuitfox</a>: <a href="https://github.com/rust-lang/rust/pull/35576">E0072</a>, <a href="https://github.com/rust-lang/rust/pull/35555">E0128</a></li>
  <li><a href="https://github.com/wdv4758h">@wdv4758h</a>: <a href="https://github.com/rust-lang/rust/pull/35573">E0138</a>, <a href="https://github.com/rust-lang/rust/pull/35565">E0133</a></li>
  <li><a href="https://github.com/lukehinds">@lukehinds</a>: <a href="https://github.com/rust-lang/rust/pull/35558">E0253</a></li>
  <li><a href="https://github.com/Limeth">@Limeth</a>: <a href="https://github.com/rust-lang/rust/pull/35557">E0263</a></li>
  <li><a href="https://github.com/theypsilon">@theypsilon</a>: <a href="https://github.com/rust-lang/rust/pull/35552">E0384</a>, <a href="https://github.com/rust-lang/rust/pull/35646">E0094</a></li>
</ul>

<p>Others contributions:</p>

<ul>
  <li><a href="https://github.com/IvanUkhov">@IvanUkhov</a> fixed <a href="https://github.com/rust-lang/rust/pull/35661">a couple of typos in RawVec</a>.</li>
  <li><a href="https://github.com/pietroalbini">@pietroalbini</a>: fixed <a href="https://github.com/rust-lang/rust/pull/35569">docs typo in std::os::unix::net::SocketAddr::is_unnamed</a>.</li>
  <li><a href="https://github.com/matthew-piziak">@matthew-piziak</a> fixed <a href="https://github.com/rust-lang/rust/pull/35622">small typos in std::convert documentation</a>.</li>
  <li><a href="https://github.com/cvubrugier">@cvubrugier</a> fixed <a href="https://github.com/rust-lang/rust/pull/35620">the hidden find() functions in The Book</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/35597"><code class="highlighter-rouge">&amp;str</code> calling in the doc</a>.</li>
  <li><a href="https://github.com/qolop">@qolop</a> fixed <a href="https://github.com/rust-lang/rust/pull/34941">typo (privledge-&gt;privilege)</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/35477">E0132 error display</a> and added <a href="https://github.com/rust-lang/rust/pull/35431">new error code tests</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 17th of August 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>