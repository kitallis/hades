+++
title = "This Week in Rust Docs 16"
date = "2016-08-08T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-16"
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

<p>The <a href="https://github.com/rust-lang/rfcs/pull/1683#issuecomment-237384575">doc team RFC</a> has entered its final comment period!</p>

<p>A new RFC has been opened: <a href="https://github.com/rust-lang/rfcs/pull/1687">Add API documentation front page styleguide</a>.</p>

<p>A topic to propose crates for the Rust Doc Days has been created <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

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
  <li><a href="https://github.com/qolop">@qolop</a> fixed <a href="https://github.com/rust-lang/rust/pull/34941">typo (privledge-&gt;privilege)</a>.</li>
  <li><a href="https://github.com/jhod0">@jhod0</a> added <a href="https://github.com/rust-lang/rust/pull/34970">diagnostics for rustc_metadata</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/33922">a specific error message for misplaced doc comments</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/35003">item types from some title pages from rustdoc</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made it clear that <a href="https://github.com/rust-lang/rust/pull/35102">the reference isn’t normative</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/35012">formatting generation for rustdoc code examples</a></li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<p>This week, I’ll split between the <a href="https://github.com/rust-lang/rust/issues/35233">new error code format</a> contributions and the others. Let’s start by the first one:</p>

<ul>
  <li><a href="https://github.com/munyari">@munyari</a>: <a href="https://github.com/rust-lang/rust/pull/35468">E0205</a>, <a href="https://github.com/rust-lang/rust/pull/35455">E0204</a></li>
  <li><a href="https://github.com/terrynsun">@terrynsun</a>: <a href="https://github.com/rust-lang/rust/pull/35467">E0116</a></li>
  <li><a href="https://github.com/Detegr">@Detegr</a>: <a href="https://github.com/rust-lang/rust/pull/35454">E0117 and E0118</a></li>
  <li><a href="https://github.com/franleplant">@franleplant</a>: <a href="https://github.com/rust-lang/rust/pull/35443">E0101 and E0102</a></li>
  <li><a href="https://github.com/pcn">@pcn</a>: <a href="https://github.com/rust-lang/rust/pull/35439">E0010</a></li>
  <li><a href="https://github.com/intrepion">@intrepion</a>: <a href="https://github.com/rust-lang/rust/pull/35434">E0121</a></li>
  <li><a href="https://github.com/razielgn">@razielgn</a>: <a href="https://github.com/rust-lang/rust/pull/35421">E0225</a>, <a href="https://github.com/rust-lang/rust/pull/35370">E0306</a>, <a href="https://github.com/rust-lang/rust/pull/35285">E0071</a></li>
  <li><a href="https://github.com/Keats">@Keats</a>: <a href="https://github.com/rust-lang/rust/pull/35419">E0243 and E0244</a>, <a href="https://github.com/rust-lang/rust/pull/35372">E0323, E0324 and E0325</a>, <a href="https://github.com/rust-lang/rust/pull/35319">E0137</a>, <a href="https://github.com/rust-lang/rust/pull/35298">E0120</a></li>
  <li><a href="https://github.com/Limeth">@Limeth</a>: <a href="https://github.com/rust-lang/rust/pull/35417">E0131</a></li>
  <li><a href="https://github.com/silenuss">@silenuss</a>: <a href="https://github.com/rust-lang/rust/pull/35413">E0029</a>, <a href="https://github.com/rust-lang/rust/pull/35410">E0027</a></li>
  <li><a href="https://github.com/KiChjang">@KiChjang</a>: <a href="https://github.com/rust-lang/rust/pull/35411">E0223</a>, <a href="https://github.com/rust-lang/rust/pull/35402">E0206</a></li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a>: <a href="https://github.com/rust-lang/rust/pull/35394">E0201</a></li>
  <li><a href="https://github.com/TheZoq2">@TheZoq2</a>: <a href="https://github.com/rust-lang/rust/pull/35380">E0004</a></li>
  <li><a href="https://github.com/trixnz">@trixnz</a>: <a href="https://github.com/rust-lang/rust/pull/35376">E0373</a>, <a href="https://github.com/rust-lang/rust/pull/35328">E0062</a></li>
  <li><a href="https://github.com/mrabault">@mrabault</a>: <a href="https://github.com/rust-lang/rust/pull/35374">E0229</a></li>
  <li><a href="https://github.com/oijazsh">@oijazsh</a>: <a href="https://github.com/rust-lang/rust/pull/35373">E0107</a></li>
  <li><a href="https://github.com/medzin">@medzin</a>: <a href="https://github.com/rust-lang/rust/pull/35366">E0282</a>, <a href="https://github.com/rust-lang/rust/pull/35362">E0252</a>, <a href="https://github.com/rust-lang/rust/pull/35296">E0178</a></li>
  <li><a href="https://github.com/kc1212">@kc1212</a>: <a href="https://github.com/rust-lang/rust/pull/35364">E0379</a></li>
  <li><a href="https://github.com/Archytaus">@Archytaus</a>: <a href="https://github.com/rust-lang/rust/pull/35359">E0391 and E0404</a></li>
  <li><a href="https://github.com/shri3k">@shri3k</a>: <a href="https://github.com/rust-lang/rust/pull/35357">E0040</a>, <a href="https://github.com/rust-lang/rust/pull/35355">E0046</a></li>
  <li><a href="https://github.com/Tiwalun">@Tiwalun</a>: <a href="https://github.com/rust-lang/rust/pull/35356">E0106</a></li>
  <li><a href="https://github.com/poveda-ruiz">@poveda-ruiz</a>: <a href="https://github.com/rust-lang/rust/pull/35353">E0081</a></li>
  <li><a href="https://github.com/jaredwy">@jaredwy</a>: <a href="https://github.com/rust-lang/rust/pull/35351">E0069</a></li>
  <li><a href="https://github.com/birryree">@birryree</a>: <a href="https://github.com/rust-lang/rust/pull/35350">E0368</a>, <a href="https://github.com/rust-lang/rust/pull/35289">E0060 and E0061</a></li>
  <li><a href="https://github.com/nickmass">@nickmass</a>: <a href="https://github.com/rust-lang/rust/pull/35333">E0055</a></li>
  <li><a href="https://github.com/circuitfox">@circuitfox</a>: <a href="https://github.com/rust-lang/rust/pull/35326">E0119</a>, <a href="https://github.com/rust-lang/rust/pull/35299">E0110</a>, <a href="https://github.com/rust-lang/rust/pull/35266">E0109</a></li>
  <li><a href="https://github.com/sciyoshi">@sciyoshi</a>: <a href="https://github.com/rust-lang/rust/pull/35318">E0124</a></li>
  <li><a href="https://github.com/yossi-k">@yossi-k</a>: <a href="https://github.com/rust-lang/rust/pull/35314">E0185 and E0186</a>, <a href="https://github.com/rust-lang/rust/pull/35291">E0079</a></li>
  <li><a href="https://github.com/saml">@saml</a>: <a href="https://github.com/rust-lang/rust/pull/35297">E0001</a></li>
  <li><a href="https://github.com/Roybie">@Roybie</a>: <a href="https://github.com/rust-lang/rust/pull/35294">E0172</a>, <a href="https://github.com/rust-lang/rust/pull/35288">E0166</a></li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a>: <a href="https://github.com/rust-lang/rust/pull/35264">E0132</a></li>
</ul>

<p>Others contributions:</p>

<ul>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/35239">doc for <code class="highlighter-rouge">std::thread::park_timeout</code></a>.</li>
  <li><a href="https://github.com/shantanuraj">@shantanuraj</a> updated <a href="https://github.com/rust-lang/rust/pull/35283">wording on E0080</a>.</li>
  <li><a href="https://github.com/apasel422">@apasel422</a> added <a href="https://github.com/rust-lang/rust/pull/35182">doc example for <code class="highlighter-rouge">std::ffi::NulError::nul_position</code></a>, cleaned up <a href="https://github.com/rust-lang/rust/pull/35281"><code class="highlighter-rouge">std::raw</code> docs</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/35436">doc example for <code class="highlighter-rouge">std::ffi::NulError::into_vec</code></a>, made <a href="https://github.com/rust-lang/rust/pull/35175">a couple <code class="highlighter-rouge">std::net</code> doc improvements</a>, rewrote <a href="https://github.com/rust-lang/rust/pull/35134">rewrite <code class="highlighter-rouge">slice::chunks</code> doc example to not require printing</a>, added <a href="https://github.com/rust-lang/rust/pull/35041">doc examples for <code class="highlighter-rouge">range::RangeArgument::{start,end}</code></a> and rewrote <a href="https://github.com/rust-lang/rust/pull/35134"><code class="highlighter-rouge">slice::chunks</code> doc example to not require printing</a>.</li>
  <li><a href="https://github.com/jongiddy">@jongiddy</a> provide <a href="https://github.com/rust-lang/rust/pull/35137">a more explicit example of wildcard version in guessing game doc</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> clarified <a href="https://github.com/rust-lang/rust/pull/34520">UnsafeCell docs</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/35076">doc examples for FileType struct</a>, added <a href="https://github.com/rust-lang/rust/pull/35109"><code class="highlighter-rouge">io::Error</code> doc examples</a>, added <a href="https://github.com/rust-lang/rust/pull/35181">doc example for <code class="highlighter-rouge">Vec</code></a>, added <a href="https://github.com/rust-lang/rust/pull/35393">new error codes</a> and added even more <a href="https://github.com/rust-lang/rust/pull/35363">error code tests</a> and added even even more <a href="https://github.com/rust-lang/rust/pull/35274">error code tests</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 10th of August 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>