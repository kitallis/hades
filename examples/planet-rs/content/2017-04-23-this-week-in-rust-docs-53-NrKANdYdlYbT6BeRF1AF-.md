+++
title = "This Week in Rust Docs 53"
date = "2017-04-23T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-53"
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
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/40857">at fields that make the type recursive</a>, emitted <a href="https://github.com/rust-lang/rust/pull/41434">diagnostic when using <code class="highlighter-rouge">const</code> storing <code class="highlighter-rouge">Fn</code> as pattern</a> and did <a href="https://github.com/rust-lang/rust/pull/41433">not show <code class="highlighter-rouge">::</code> on tuple struct diagnostics</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/stepancheg">@stepancheg</a> improved <a href="https://github.com/rust-lang/rust/pull/40747">BufRead::is_eof documentation</a>.</li>
  <li><a href="https://github.com/icefoxen">@icefoxen</a> made <a href="https://github.com/rust-lang/rust/pull/40719">a tiny update to rustdoc CSS</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> updated <a href="https://github.com/rust-lang/rust/pull/41217">docs of ‘fence’</a>.</li>
  <li><a href="https://github.com/abonander">@abonander</a> documented <a href="https://github.com/rust-lang/rust/pull/41476">the <code class="highlighter-rouge">proc_macro</code> feature in the Unstable Book</a>.</li>
  <li><a href="https://github.com/tbu-">@tbu-</a> specified <a href="https://github.com/rust-lang/rust/pull/41442">behavior of <code class="highlighter-rouge">write_all</code> for <code class="highlighter-rouge">ErrorKind::Interrupted</code> errors</a>.</li>
  <li><a href="https://github.com/projektir">@projektir</a> added <a href="https://github.com/rust-lang/rust/pull/41438">links and examples for various mspc pages</a>.</li>
  <li><a href="https://github.com/insaneinside">@insaneinside</a> removed <a href="https://github.com/rust-lang/rust/pull/41346">display of duplicate bounds for “…but the following trait bounds were not satisfied”</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/40634">more explanation on RefCell::get_mut</a>, removed <a href="https://github.com/rust-lang/rust/pull/41307">jquery dependency</a> and reduced <a href="https://github.com/rust-lang/rust/pull/41384">HTML output in rustdoc</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> reduced <a href="https://github.com/rust-lang/rust/pull/41245">visual clutter of multiline start when possible</a> and added <a href="https://github.com/rust-lang/rust/pull/41214">a way to get shorter spans until <code class="highlighter-rouge">char</code> for pointing at defs</a>.</li>
  <li><a href="https://github.com/maccoda">@maccoda</a> improved <a href="https://github.com/rust-lang/rust/pull/40987">Convert docs</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> updated <a href="https://github.com/rust-lang/rust/pull/40812"><code class="highlighter-rouge">Child</code> docs to not have a note section</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> added <a href="https://github.com/rust-lang/rust/pull/41295">top level sections to the Unstable Book.</a>.</li>
  <li><a href="https://github.com/cengizIO">@cengizIO</a> moved <a href="https://github.com/rust-lang/rust/pull/41236">E0101 and E0102 logic into new E0282 mechanism</a>.</li>
  <li><a href="https://github.com/jonhoo">@jonhoo</a> renamed <a href="https://github.com/rust-lang/rust/pull/41262">compiler_barrier to compiler_fence</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/41280">a list of headings to the sidebar in rustdoc</a>.</li>
  <li><a href="https://github.com/alexeyzab">@alexeyzab</a> fixed <a href="https://github.com/rust-lang/rust/pull/41264">old docs</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> expanded <a href="https://github.com/rust-lang/rust/pull/41376">docs and examples for PathBuf::file_name and friends</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> improved <a href="https://github.com/rust-lang/rust/pull/41348">std::path docs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> fixed <a href="https://github.com/rust-lang/rust/pull/41365">link to book repo</a> and updated <a href="https://github.com/rust-lang/rust/pull/41374">mdbook</a>.</li>
  <li><a href="https://github.com/durka">@durka</a> removed <a href="https://github.com/rust-lang/rust/pull/41391">disclaimer from bootstrap/README.md</a>.</li>
  <li><a href="https://github.com/mbrickn">@mbrickn</a> updated <a href="https://github.com/rust-lang/rust/pull/41403">links to use https</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>, made <a href="https://github.com/rust-lang/rust/pull/41290">hoedown big comeback</a>, fixed <a href="https://github.com/rust-lang/rust/pull/41342">debug infinite loop</a>, fixed <a href="https://github.com/rust-lang/rust/pull/41405">line display for hoedown</a> and re-enabled <a href="https://github.com/rust-lang/rust/pull/41431">hoedown by default</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 26th of April 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>