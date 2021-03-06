+++
title = "This Week in Rust Docs 69"
date = "2017-08-20T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-69"
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

<p>The <a href="https://github.com/steveklabnik/rustdoc">rewrite of rustdoc</a> is still under heavy development. Don’t hesitate to give it a try!</p>

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
  <li><a href="https://github.com/oli-obk">@oli-obk</a> added <a href="https://github.com/rust-lang/rust/pull/43641">lint casting signed integers smaller than <code class="highlighter-rouge">isize</code> to raw pointers</a> and improved <a href="https://github.com/rust-lang/rust/pull/43929">placement of <code class="highlighter-rouge">use</code> suggestions</a>.</li>
  <li><a href="https://github.com/ruuda">@ruuda</a> pointed <a href="https://github.com/rust-lang/rust/pull/43631">“deref coercions” links to new book</a>.</li>
  <li><a href="https://github.com/gaurikholkar">@gaurikholkar</a> added <a href="https://github.com/rust-lang/rust/pull/43700">E0623 for structs</a> and improved <a href="https://github.com/rust-lang/rust/pull/43851">error message for one named, one anonymous lifetime parameters - underline Type</a>.</li>
  <li><a href="https://github.com/qnighy">@qnighy</a> added <a href="https://github.com/rust-lang/rust/pull/43426">hints when intercrate ambiguity causes overlap</a>.</li>
  <li><a href="https://github.com/shanavas786">@shanavas786</a> fixed <a href="https://github.com/rust-lang/rust/pull/43996">typo in doc</a>.</li>
  <li><a href="https://github.com/huntiep">@huntiep</a> improved <a href="https://github.com/rust-lang/rust/pull/43984">Try error messages</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/43854">out missing if conditional</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> added <a href="https://github.com/rust-lang/rust/pull/43849">new “Implementations on Foreign Types” section to traits in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/41991">warnings when rustdoc html rendering differs</a>, added <a href="https://github.com/rust-lang/rust/pull/43978">missing links in docs</a>, removed <a href="https://github.com/rust-lang/rust/pull/43977">outline when details have focus</a>, compile <a href="https://github.com/rust-lang/rust/pull/43949">fail stable</a>, removed <a href="https://github.com/rust-lang/rust/pull/43966">duplicates in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/43870">deref suggestion</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/nrc">@nrc</a> fixed <a href="https://github.com/rust-lang/rust/pull/43782">include! in doc tests</a> and uplifted <a href="https://github.com/rust-lang/rust/pull/43922">fix for include! in doc tests to beta</a>.</li>
  <li><a href="https://github.com/Eijebong">@Eijebong</a> fixed <a href="https://github.com/rust-lang/rust/pull/43814">some typos</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> improved <a href="https://github.com/rust-lang/rust/pull/43819">doc examples for <code class="highlighter-rouge">include*</code> macros</a>, made <a href="https://github.com/rust-lang/rust/pull/43965">a minor Iterator::filter_map description rewording</a>, made <a href="https://github.com/rust-lang/rust/pull/43919">a minor rewrite of char primitive unicode intro</a>, clarified <a href="https://github.com/rust-lang/rust/pull/43883">writable behavior of readonly-named <code class="highlighter-rouge">Permissions</code> methods</a> and rewrote/reorganized <a href="https://github.com/rust-lang/rust/pull/43848">docs for stack size/thread names for spawned threads</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> wrote <a href="https://github.com/rust-lang/rust/pull/43790">the “passes” chapter of the rustdoc book</a>, updated <a href="https://github.com/rust-lang/rust/pull/43914">the books for next release</a> and shipped <a href="https://github.com/rust-lang/rust/pull/43863">the rustdoc book</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> fixed <a href="https://github.com/rust-lang/rust/pull/43933">bad span for attributes</a>.</li>
  <li><a href="https://github.com/anthonyclays">@anthonyclays</a> fixed <a href="https://github.com/rust-lang/rust/pull/43928">typo in RefCell::get_mut</a>.</li>
  <li><a href="https://github.com/Songbird0">@Songbird0</a> improved <a href="https://github.com/rust-lang/rust/pull/43912">E0106 - field lifetimes</a>.</li>
  <li><a href="https://github.com/adrian5">@adrian5</a> fixed <a href="https://github.com/rust-lang/rust/pull/43915">typo in doc</a>.</li>
  <li><a href="https://github.com/partim">@partim</a> documented <a href="https://github.com/rust-lang/rust/pull/43905">that <code class="highlighter-rouge">std::hash::Hasher::finish()</code> does not reset the hasher</a>.</li>
  <li><a href="https://github.com/Fourchaux">@Fourchaux</a> fixed <a href="https://github.com/rust-lang/rust/pull/43891">typos &amp; us spellings</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> put <a href="https://github.com/rust-lang/rust/pull/43862">auto-hidden docblock labels in line with the toggle in rustdoc</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> added <a href="https://github.com/rust-lang/rust/pull/43868">missing newline in Deref docs to fix rendering</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> removed <a href="https://github.com/rust-lang/rust/pull/43736">external impls to implementors js in rustdoc</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/43803">missing links doc</a>, udpdated <a href="https://github.com/rust-lang/rust/pull/43901">error message for unsized union field</a>, added <a href="https://github.com/rust-lang/rust/pull/43864">help for static method invalid use</a>, removed <a href="https://github.com/rust-lang/rust/pull/43867">useless help part</a> and added <a href="https://github.com/rust-lang/rust/pull/43850">a note to unused variables</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 23rd of August 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>