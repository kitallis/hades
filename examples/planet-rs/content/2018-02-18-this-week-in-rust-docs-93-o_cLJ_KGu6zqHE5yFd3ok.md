+++
title = "This Week in Rust Docs 93"
date = "2018-02-18T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-93"
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

<p>Hoedown is finally being removed from rustdoc! I’ll post the approval message from <a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> in here:</p>

<blockquote>
  <p>The preparations are complete. It is time…</p>

  <p><em><strong>Begone, demon of the foul C! Your presence is no longer wanted here! With this strike, I commit you to the depths of history, never to torment our fair land again!</strong></em></p>
</blockquote>

<p>You can see the pull request <a href="https://github.com/rust-lang/rust/pull/48274">here</a>.</p>

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
  <li><a href="https://github.com/estebank">@estebank</a> reworded <a href="https://github.com/rust-lang/rust/pull/48138">E0044 and message for <code class="highlighter-rouge">!Send</code> types</a>, detected <a href="https://github.com/rust-lang/rust/pull/47763">possibly non-Rust closure syntax during parse</a>, suggested <a href="https://github.com/rust-lang/rust/pull/48288">setting associated type when type argument is given instead</a> and avoidED <a href="https://github.com/rust-lang/rust/pull/48246">ICE in arg mistmatch error for tuple variants</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> added <a href="https://github.com/rust-lang/rust/pull/47894">foldable impl blocks in rustdoc</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> fixed <a href="https://github.com/rust-lang/rust/pull/47731">how paths are printed by error messages during bootstrap</a>.</li>
  <li><a href="https://github.com/Aaron1011">@Aaron1011</a> generated <a href="https://github.com/rust-lang/rust/pull/47833">documentation for auto-trait impls</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> fixed <a href="https://github.com/rust-lang/rust/pull/47799">span of visibility</a>.</li>
  <li><a href="https://github.com/NovemberZulu">@NovemberZulu</a> rephrased <a href="https://github.com/rust-lang/rust/pull/48201">UnsafeCell doc</a>.</li>
  <li><a href="https://github.com/csmoe">@csmoe</a> informed <a href="https://github.com/rust-lang/rust/pull/48198">user where to give a type annotation</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> unified <a href="https://github.com/rust-lang/rust/pull/48312">‘Platform-specific behavior’ documentation headings</a>, fixed <a href="https://github.com/rust-lang/rust/pull/48314">broken documentation link</a> and removed <a href="https://github.com/rust-lang/rust/pull/48305">section about compiler-rt in COPYRIGHT</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> moved <a href="https://github.com/rust-lang/rust/pull/48106">manual “extern crate” statements outside automatic “fn main”s in doctests in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/48283">readme for librustdoc</a>.</li>
  <li><a href="https://github.com/alercah">@alercah</a> added <a href="https://github.com/rust-lang/rust/pull/48273">a warning to File about mutability</a>.</li>
  <li><a href="https://github.com/matthiaskrgr">@matthiaskrgr</a> fixed <a href="https://github.com/rust-lang/rust/pull/48275">more typos found by codespell</a>.</li>
  <li><a href="https://github.com/FraGag">@FraGag</a> improved <a href="https://github.com/rust-lang/rust/pull/48171">documentation of Clone and Copy implementors</a>.</li>
  <li><a href="https://github.com/zilbuz">@zilbuz</a> showed <a href="https://github.com/rust-lang/rust/pull/47574">the used type variable when issuing a “can’t use type parameters from outer function” error message</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/48274">hoedown from rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/48194">doc test command</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/antoyo">@antoyo</a> greatly improved <a href="https://github.com/rust-lang/rust/pull/48152">primitive docs</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/47843"><code class="highlighter-rouge">-Zteach</code> documentation</a>.</li>
  <li><a href="https://github.com/PramodBisht">@PramodBisht</a> changed <a href="https://github.com/rust-lang/rust/pull/47806">color of struct link from #ff794d to #2dbfb8 for Rust docs</a>.</li>
  <li><a href="https://github.com/matthiaskrgr">@matthiaskrgr</a> fixed typo <a href="https://github.com/rust-lang/rust/pull/48133">endianess to endianness (this also changes function names!)</a>.</li>
  <li><a href="https://github.com/SergioBenitez">@SergioBenitez</a> clarified <a href="https://github.com/rust-lang/rust/pull/48286">contiguity of Vec’s elements</a>.</li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/48260">link to yield_now</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/48095">unit tests for rustdoc’s processing of doctests</a>.</li>
  <li><a href="https://github.com/jacob-hughes">@jacob-hughes</a> clarified <a href="https://github.com/rust-lang/rust/pull/48210">why <code class="highlighter-rouge">Sized</code> bound not implicit on trait’s implicit <code class="highlighter-rouge">Self</code> type</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/47286">RELEASES.md for 1.24.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> showed <a href="https://github.com/rust-lang/rust/pull/48033">better warning for trying to cast non-u8 scalar to char</a>, rollup <a href="https://github.com/rust-lang/rust/pull/48294">of 8 pull requests</a> and fix <a href="https://github.com/rust-lang/rust/pull/48239">condvar example</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 20th of February 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>