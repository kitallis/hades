+++
title = "This Week in Rust Docs 94"
date = "2018-02-25T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-94"
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
  <li><a href="https://github.com/estebank">@estebank</a> reworded <a href="https://github.com/rust-lang/rust/pull/48138">E0044 and message for <code class="highlighter-rouge">!Send</code> types</a>, suggested <a href="https://github.com/rust-lang/rust/pull/48288">setting associated type when type argument is given instead</a> and provided <a href="https://github.com/rust-lang/rust/pull/48338">context for missing comma in match arm and if statement without block</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> improved <a href="https://github.com/rust-lang/rust/pull/46518">documentation for Borrow, AsRef, and friends</a>.</li>
  <li><a href="https://github.com/dbrgn">@dbrgn</a> whitelisted <a href="https://github.com/rust-lang/rust/pull/46815">based suggestions</a>.</li>
  <li><a href="https://github.com/vi">@vi</a> added <a href="https://github.com/rust-lang/rust/pull/47894">foldable impl blocks in rustdoc</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> fixed <a href="https://github.com/rust-lang/rust/pull/47731">how paths are printed by error messages during bootstrap</a>.</li>
  <li><a href="https://github.com/NovemberZulu">@NovemberZulu</a> rephrased <a href="https://github.com/rust-lang/rust/pull/48201">UnsafeCell doc</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/48283">readme for librustdoc</a>.</li>
  <li><a href="https://github.com/FraGag">@FraGag</a> improved <a href="https://github.com/rust-lang/rust/pull/48171">documentation of Clone and Copy implementors</a>.</li>
  <li><a href="https://github.com/zilbuz">@zilbuz</a> showed <a href="https://github.com/rust-lang/rust/pull/47574">the used type variable when issuing a “can’t use type parameters from outer function” error message</a>.</li>
  <li><a href="https://github.com/remexre">@remexre</a> fixed <a href="https://github.com/rust-lang/rust/pull/48529">docs for ASCII functions to no longer claim U+0021 is ‘@’</a>.</li>
  <li><a href="https://github.com/christianpoveda">@christianpoveda</a> made <a href="https://github.com/rust-lang/rust/pull/48474">new Cell docs</a>.</li>
  <li><a href="https://github.com/mark-i-m">@mark-i-m</a> splitted <a href="https://github.com/rust-lang/rust/pull/48446">E0404 to E0909; get rid of E0245</a> and started <a href="https://github.com/rust-lang/rust/pull/48479">moving to the rustc guide!</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> clarified <a href="https://github.com/rust-lang/rust/pull/48328">“It is an error to…” wording for zero-duration behaviors</a>.</li>
  <li><a href="https://github.com/Phlosioneer">@Phlosioneer</a> made <a href="https://github.com/rust-lang/rust/pull/48509">slight modification to the as_ref example of std::option::Option</a>.</li>
  <li><a href="https://github.com/Centril">@Centril</a> documented <a href="https://github.com/rust-lang/rust/pull/48365">panics in Clone, PartialEq, PartialOrd, Ord for RefCell</a>.</li>
  <li><a href="https://github.com/flip1995">@flip1995</a> suggested <a href="https://github.com/rust-lang/rust/pull/48432">type for overflowing bin/hex-literals</a>.</li>
  <li><a href="https://github.com/jethrogb">@jethrogb</a> clarified <a href="https://github.com/rust-lang/rust/pull/48480">interfaction between File::set_len and file cursor</a>.</li>
  <li><a href="https://github.com/lukaslueg">@lukaslueg</a> fixed <a href="https://github.com/rust-lang/rust/pull/48403">spelling s/casted/cast/</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> warned <a href="https://github.com/rust-lang/rust/pull/48326">about ignored generic bounds in <code class="highlighter-rouge">for</code></a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/48374">RELEASES.md for 1.25.0</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added back <a href="https://github.com/rust-lang/rust/pull/48337">rustc explain</a>, added <a href="https://github.com/rust-lang/rust/pull/48511">resource-suffix option for rustdoc</a>, add <a href="https://github.com/rust-lang/rust/pull/48442">root-path</a>, added <a href="https://github.com/rust-lang/rust/pull/48173">error codes for libsyntax_ext</a>, added <a href="https://github.com/rust-lang/rust/pull/48507">new warning for CStr::from_ptr</a>, added <a href="https://github.com/rust-lang/rust/pull/48381">rustdoc theme securities</a> and fixed <a href="https://github.com/rust-lang/rust/pull/48473">auto trait impl rustdoc ice</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> avoided <a href="https://github.com/rust-lang/rust/pull/48246">ICE in arg mistmatch error for tuple variants</a> and handled <a href="https://github.com/rust-lang/rust/pull/48392">custom diagnostic for <code class="highlighter-rouge">&amp;str + String</code></a>.</li>
  <li><a href="https://github.com/Aaron1011">@Aaron1011</a> generated <a href="https://github.com/rust-lang/rust/pull/47833">documentation for auto-trait impls</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> fixed <a href="https://github.com/rust-lang/rust/pull/47799">span of visibility</a>.</li>
  <li><a href="https://github.com/csmoe">@csmoe</a> informed <a href="https://github.com/rust-lang/rust/pull/48198">user where to give a type annotation</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> unified <a href="https://github.com/rust-lang/rust/pull/48312">‘Platform-specific behavior’ documentation headings</a>, fixed <a href="https://github.com/rust-lang/rust/pull/48314">broken documentation link</a> and marked <a href="https://github.com/rust-lang/rust/pull/48325">doc examples w/ <code class="highlighter-rouge">extern</code> blocks as <code class="highlighter-rouge">ignore</code></a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> moved <a href="https://github.com/rust-lang/rust/pull/48106">manual “extern crate” statements outside automatic “fn main”s in doctests in rustdoc</a> and fixed <a href="https://github.com/rust-lang/rust/pull/48415">crash when an external trait’s docs needs to import another trait in rustdoc</a>.</li>
  <li><a href="https://github.com/alercah">@alercah</a> added <a href="https://github.com/rust-lang/rust/pull/48273">a warning to File about mutability</a>.</li>
  <li><a href="https://github.com/matthiaskrgr">@matthiaskrgr</a> fixed <a href="https://github.com/rust-lang/rust/pull/48275">more typos found by codespell</a>.</li>
  <li><a href="https://github.com/varkor">@varkor</a> introduce <a href="https://github.com/rust-lang/rust/pull/48452">UnpackedKind</a>.</li>
  <li><a href="https://github.com/Manishearth">@Manishearth</a> implemented <a href="https://github.com/rust-lang/rust/pull/48335">implied shortcut links for intra-rustdoc-links</a>.</li>
  <li><a href="https://github.com/dwijnand">@dwijnand</a> fixed <a href="https://github.com/rust-lang/rust/pull/48499">capitalisation in Path#file_name’s docs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/48404">the book to promote second edition</a>.</li>
  <li><a href="https://github.com/withoutboats">@withoutboats</a> added <a href="https://github.com/rust-lang/rust/pull/48386">nonstandard_style alias for bad_style</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> made <a href="https://github.com/rust-lang/rust/pull/48438">minor wording changes to drain_filter docs</a>.</li>
  <li><a href="https://github.com/adeschamps">@adeschamps</a> made <a href="https://github.com/rust-lang/rust/pull/48436">a small grammar fix to docs for String::new()</a>.</li>
  <li><a href="https://github.com/ordovicia">@ordovicia</a> take <a href="https://github.com/rust-lang/rust/pull/48397">2^5 as examples in document of pow()</a>.</li>
  <li><a href="https://github.com/redcape">@redcape</a> fixed <a href="https://github.com/rust-lang/rust/pull/48360">count usize link typo in docs</a>.</li>
  <li><a href="https://github.com/m0ppers">@m0ppers</a> added <a href="https://github.com/rust-lang/rust/pull/48354">missing link for read_line</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> removed <a href="https://github.com/rust-lang/rust/pull/48274">hoedown from rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/48194">doc test command</a> and fixed <a href="https://github.com/rust-lang/rust/pull/48382">rustdoc test ICE</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Tuesday 27th of February 2018 at 19:00 UTC on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>