+++
title = "This Week in Rust Docs 107"
date = "2018-06-03T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-107"
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

<p>Nothing important enough.</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
  <li><a href="https://github.com/rust-lang-nursery/reference/projects/1">Document all features in the Rust reference</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/labels/T-doc">T-doc</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/fkjogu">@fkjogu</a> documented <a href="https://github.com/rust-lang/rust/pull/50342">round-off error in <code class="highlighter-rouge">.mod_euc()</code>-method</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> added <a href="https://github.com/rust-lang/rust/pull/50143">deprecation lint for duplicated <code class="highlighter-rouge">macro_export</code>s</a> and stabilize <a href="https://github.com/rust-lang/rust/pull/50911"><code class="highlighter-rouge">use_extern_macros</code></a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/50852">doc comment to hiding portions of code example</a>.</li>
  <li><a href="https://github.com/ogham">@ogham</a> documented <a href="https://github.com/rust-lang/rust/pull/51156">Vec’s particular IntoIter behaviour</a>, mentioned <a href="https://github.com/rust-lang/rust/pull/51158">spec and indented blocks in doctest docs</a>.</li>
  <li><a href="https://github.com/kennytm">@kennytm</a> pointed <a href="https://github.com/rust-lang/rust/pull/51111">to the rustdoc attribute where intralink resolution failed</a>.</li>
  <li><a href="https://github.com/teiesti">@teiesti</a> updated <a href="https://github.com/rust-lang/rust/pull/51183">rustdoc book to suggest using Termination trait instead of hidden ‘foo’ function</a>.</li>
  <li><a href="https://github.com/kornelski">@kornelski</a> used <a href="https://github.com/rust-lang/rust/pull/51081">String, not &amp;str in some collection examples</a>.</li>
  <li><a href="https://github.com/avdv">@avdv</a> fixed <a href="https://github.com/rust-lang/rust/pull/51255">confusing error message for sub_instant</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/51261">RELEASES.md for 1.27.0</a>.</li>
  <li><a href="https://github.com/cmdd">@cmdd</a> added <a href="https://github.com/rust-lang/rust/pull/50296">error message for using &gt;= 65535 hashes for raw string literal escapes</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/50617">extern prelude failure in rustdoc</a>, introduce <a href="https://github.com/rust-lang/rust/pull/51140">the #[doc(keyword=””)] attribute for documenting keywords</a> and fixed <a href="https://github.com/rust-lang/rust/pull/51256">crate-name option in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/F001">@F001</a> added <a href="https://github.com/rust-lang/rust/pull/50682">lint for multiple associated types</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> provided <a href="https://github.com/rust-lang/rust/pull/50919">more context for what the {f32,f64}::EPSILON values represent</a>, clarified <a href="https://github.com/rust-lang/rust/pull/51312">the difference between get_mut and into_mut for OccupiedEntry</a>, reworded <a href="https://github.com/rust-lang/rust/pull/51124">{ptr,mem}::replace docs</a> and added <a href="https://github.com/rust-lang/rust/pull/51127">doc link from discriminant struct to function</a>.</li>
  <li><a href="https://github.com/davidtwco">@davidtwco</a> added <a href="https://github.com/rust-lang/rust/pull/50892">rustdoc documentation to compiler docs</a>.</li>
  <li><a href="https://github.com/simartin">@simartin</a> improved <a href="https://github.com/rust-lang/rust/pull/50914">error diagnostic with missing commas after struct fields</a>.</li>
  <li><a href="https://github.com/d-e-s-o">@d-e-s-o</a> fixed <a href="https://github.com/rust-lang/rust/pull/50913">typo in cell.rs</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> used <a href="https://github.com/rust-lang/rust/pull/50875">“short form” doc(cfg) printing even when combined with other conditionals in rustdoc</a> and hid <a href="https://github.com/rust-lang/rust/pull/51011">macro export statements from docs</a>.</li>
  <li><a href="https://github.com/euclio">@euclio</a> used <a href="https://github.com/rust-lang/rust/pull/51313">type name in E0599 enum variant suggestion</a>.</li>
  <li><a href="https://github.com/evincarofautumn">@evincarofautumn</a> fixed <a href="https://github.com/rust-lang/rust/pull/51291">typos of ‘ambiguous’</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> removed <a href="https://github.com/rust-lang/rust/pull/51272">feature flag from fs::read_to_string example</a>.</li>
  <li><a href="https://github.com/petrochenkov">@petrochenkov</a> fixed <a href="https://github.com/rust-lang/rust/pull/50879">naming conventions for new lints</a>.</li>
  <li><a href="https://github.com/crlf0710">@crlf0710</a> replaced <a href="https://github.com/rust-lang/rust/pull/51152"><code class="highlighter-rouge">if</code> with <code class="highlighter-rouge">if and only if</code> in the definition dox of <code class="highlighter-rouge">Sync</code></a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> tweaked <a href="https://github.com/rust-lang/rust/pull/51135">output on E0599 for assoc fn used as method</a> and suggested <a href="https://github.com/rust-lang/rust/pull/51100">using <code class="highlighter-rouge">as_ref</code> on some borrow errors [hack]</a>.</li>
  <li><a href="https://github.com/akoserwal">@akoserwal</a> updated <a href="https://github.com/rust-lang/rust/pull/51123">build instructions</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> extended <a href="https://github.com/rust-lang/rust/pull/51134">from_raw_parts docs for slices and strs to mention alignment requirement</a>.</li>
  <li><a href="https://github.com/nickbabcock">@nickbabcock</a> documented <a href="https://github.com/rust-lang/rust/pull/51142">additional use case for iter::inspect</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> stabilized <a href="https://github.com/rust-lang/rust/pull/49546">short error format</a>, added <a href="https://github.com/rust-lang/rust/pull/50846">E0665</a>, fixed <a href="https://github.com/rust-lang/rust/pull/51297">run button style</a>, added <a href="https://github.com/rust-lang/rust/pull/50953">attributes for trait and methods as well</a>, stabilize <a href="https://github.com/rust-lang/rust/pull/51078">Formatter alignment</a>, added <a href="https://github.com/rust-lang/rust/pull/51262">missing whitespace in num example</a> and fixed <a href="https://github.com/rust-lang/rust/pull/51193">some style issues in rustdoc “implementations on Foreign types”</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 5th of June 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>