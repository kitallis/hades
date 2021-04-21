+++
title = "This Week in Rust Docs 56"
date = "2017-05-14T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-56"
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
  <li><a href="https://github.com/estebank">@estebank</a> made <a href="https://github.com/rust-lang/rust/pull/41489">unsatisfied trait bounds note multiline</a>.</li>
  <li><a href="https://github.com/icefoxen">@icefoxen</a> made <a href="https://github.com/rust-lang/rust/pull/40719">a tiny update to rustdoc CSS</a>.</li>
  <li><a href="https://github.com/abonander">@abonander</a> documented <a href="https://github.com/rust-lang/rust/pull/41476">the <code class="highlighter-rouge">proc_macro</code> feature in the Unstable Book</a>.</li>
  <li><a href="https://github.com/F001">@F001</a> updated <a href="https://github.com/rust-lang/rust/pull/41989">backtrace formating text tips</a> and fixed <a href="https://github.com/rust-lang/rust/pull/41848">comma after struct update syntax</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> added <a href="https://github.com/rust-lang/rust/pull/41907">lint for unused macros</a>.</li>
  <li><a href="https://github.com/gamazeps">@gamazeps</a> added <a href="https://github.com/rust-lang/rust/pull/41980"><code class="highlighter-rouge">'static</code> and <code class="highlighter-rouge">Send</code> constraints explanations to <code class="highlighter-rouge">thread::spawn</code></a>, expanded <a href="https://github.com/rust-lang/rust/pull/41981"><code class="highlighter-rouge">detach</code> documentation in <code class="highlighter-rouge">thread::JoinHande</code></a> and explained <a href="https://github.com/rust-lang/rust/pull/41982">why <code class="highlighter-rouge">thread::yield_now</code> could be used</a>.</li>
  <li><a href="https://github.com/Aaronepower">@Aaronepower</a> updated <a href="https://github.com/rust-lang/rust/pull/41953">releases notes for 1.18</a>.</li>
  <li><a href="https://github.com/kosta">@kosta</a> improved <a href="https://github.com/rust-lang/rust/pull/41909">error message ‘can’t qualify macro invocation with ‘</a>.</li>
  <li><a href="https://github.com/dhardy">@dhardy</a> add <a href="https://github.com/rust-lang/rust/pull/41857">loop_break_value documentation for The Book</a>.</li>
  <li><a href="https://github.com/excaliburHisSheath">@excaliburHisSheath</a> improved <a href="https://github.com/rust-lang/rust/pull/41870">docs in os::windows::ffi and os::windows::fs</a>.</li>
  <li><a href="https://github.com/froydnj">@froydnj</a> fixed <a href="https://github.com/rust-lang/rust/pull/41859">confusion about parts required for float formatting</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> reduced <a href="https://github.com/rust-lang/rust/pull/41384">HTML output in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/41559">better error message when == operator is badly used</a>, added <a href="https://github.com/rust-lang/rust/pull/41772">help message if a FnOnce is moved</a> and made <a href="https://github.com/rust-lang/rust/pull/41700">–extend-css stable</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/40857">at fields that make the type recursive</a> and used <a href="https://github.com/rust-lang/rust/pull/41520">diagnostics for trace_macro instead of println</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> added <a href="https://github.com/rust-lang/rust/pull/41612">generic example of std::ops::Add in doc comments</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> updated <a href="https://github.com/rust-lang/rust/pull/41548">release notes for 1.17</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> improved <a href="https://github.com/rust-lang/rust/pull/41536">docs on <code class="highlighter-rouge">Arc&lt;T&gt;</code> and Send/Sync</a> and added <a href="https://github.com/rust-lang/rust/pull/41531">more ways to create a PathBuf to docs</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> allowed <a href="https://github.com/rust-lang/rust/pull/41785"># to appear in rustdoc code output</a> and made a <a href="https://github.com/rust-lang/rust/pull/41791">minor cleanup of UX guidelines</a>.</li>
  <li><a href="https://github.com/oli-obk">@oli-obk</a> refactored <a href="https://github.com/rust-lang/rust/pull/41876">suggestion diagnostic API to allow for multiple suggestions</a> and upgraded <a href="https://github.com/rust-lang/rust/pull/41912">some comments to doc comments</a>.</li>
  <li><a href="https://github.com/gamazeps">@gamazeps</a> improved <a href="https://github.com/rust-lang/rust/pull/41811"><code class="highlighter-rouge">thread::panicking</code> documentation</a>, improved <a href="https://github.com/rust-lang/rust/pull/41814"><code class="highlighter-rouge">thread::Thread</code> and <code class="highlighter-rouge">thread::Builder</code> documentations</a>, improved <a href="https://github.com/rust-lang/rust/pull/41854"><code class="highlighter-rouge">thread::spawn</code> documentation</a> and improved <a href="https://github.com/rust-lang/rust/pull/41809">the thread::park and thread::unpark documentation</a>.</li>
  <li><a href="https://github.com/mglagla">@mglagla</a> fixed <a href="https://github.com/rust-lang/rust/pull/41916">typo in Iterator::size_hint example comment</a>.</li>
  <li><a href="https://github.com/Eijebong">@Eijebong</a> broke <a href="https://github.com/rust-lang/rust/pull/41951">words in the location box of the sidebar in rustdoc</a>.</li>
  <li><a href="https://github.com/mbrubeck">@mbrubeck</a> removed <a href="https://github.com/rust-lang/rust/pull/41860">wrong or outdated info from CString docs.</a>.</li>
  <li><a href="https://github.com/RalfJung">@RalfJung</a> fixed <a href="https://github.com/rust-lang/rust/pull/41886">typo in Unique::empty doc</a>.</li>
  <li><a href="https://github.com/Migi">@Migi</a> fixed <a href="https://github.com/rust-lang/rust/pull/41842">typo in subst.rs</a>.</li>
  <li><a href="https://github.com/z1mvader">@z1mvader</a> fixed <a href="https://github.com/rust-lang/rust/pull/41838">argument inference for closures when coercing into ‘fn’</a>.</li>
  <li><a href="https://github.com/jz0425">@jz0425</a> updated <a href="https://github.com/rust-lang/rust/pull/41836">rustc-ux-guidelines</a>.</li>
  <li>
    <table>
      <tbody>
        <tr>
          <td><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> fixed <a href="https://github.com/rust-lang/rust/pull/41950">anchor invalid redirection to search</a>, added [markdown-before</td>
          <td>after-content options](https://github.com/rust-lang/rust/pull/41826), fixed <a href="https://github.com/rust-lang/rust/pull/41921">search when looking to sources</a> and improved <a href="https://github.com/rust-lang/rust/pull/41862">E0477 error message</a>.</td>
        </tr>
      </tbody>
    </table>
  </li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 17th of May 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>