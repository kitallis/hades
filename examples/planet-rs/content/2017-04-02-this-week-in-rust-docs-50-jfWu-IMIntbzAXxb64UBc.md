+++
title = "This Week in Rust Docs 50"
date = "2017-04-02T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-50"
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

<p>The markdown renderer in rustdoc <a href="https://github.com/rust-lang/rust/pull/40338">has been changed</a>! We replaced <a href="https://github.com/hoedown/hoedown">hoedown</a> with <a href="https://github.com/google/pulldown-cmark">pulldown-cmark</a>. Bugs might appear after this switch so any feedback is very welcomed! An issue has been opened <a href="https://github.com/rust-lang/rust/issues/40912">here</a>.</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the two big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/40775">using enum when a variant is used as a type</a>, pointed <a href="https://github.com/rust-lang/rust/pull/40767">to definition when modifying field of immutable variable</a>, pointed <a href="https://github.com/rust-lang/rust/pull/40857">at fields that make the type recursive</a> and added <a href="https://github.com/rust-lang/rust/pull/40565">an explicit help message for binop type mismatch</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> fixed <a href="https://github.com/rust-lang/rust/pull/41019">typo in doc comments for swap_remove</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> made a <a href="https://github.com/rust-lang/rust/pull/40653">couple minor improvements for tidy error handling</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> added <a href="https://github.com/rust-lang/rust/pull/40927">a note about overflow for fetch_add/fetch_sub</a> and improved <a href="https://github.com/rust-lang/rust/pull/40949">some docs for VecDeque</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40999">option API docs</a> and improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> fixed <a href="https://github.com/rust-lang/rust/pull/41014">link to current() in std::thread docs</a>.</li>
  <li><a href="https://github.com/pgerber">@pgerber</a> improved <a href="https://github.com/rust-lang/rust/pull/41007">documentation for <code class="highlighter-rouge">std::fs::DirBuilder</code></a>.</li>
  <li><a href="https://github.com/sagebind">@sagebind</a> improved <a href="https://github.com/rust-lang/rust/pull/41008">examples for ThreadId</a>.</li>
  <li><a href="https://github.com/stepancheg">@stepancheg</a> improved <a href="https://github.com/rust-lang/rust/pull/40747">BufRead::is_eof documentation</a>.</li>
  <li><a href="https://github.com/maccoda">@maccoda</a> improved <a href="https://github.com/rust-lang/rust/pull/40987">Convert docs</a>.</li>
  <li><a href="https://github.com/donniebishop">@donniebishop</a> added <a href="https://github.com/rust-lang/rust/pull/40997">links to types in from_utf8 description</a> and added <a href="https://github.com/rust-lang/rust/pull/40992">links to from_utf8 methods in Utf8Error</a>.</li>
  <li><a href="https://github.com/Technius">@Technius</a> added <a href="https://github.com/rust-lang/rust/pull/40981">links and some examples to std::sync::mpsc docs</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> updated <a href="https://github.com/rust-lang/rust/pull/40812"><code class="highlighter-rouge">Child</code> docs to not have a note section</a> and updated <a href="https://github.com/rust-lang/rust/pull/40829">ChildStderr docs to be clearer</a>.</li>
  <li><a href="https://github.com/projektir">@projektir</a> updated <a href="https://github.com/rust-lang/rust/pull/40977">the description for BarrierWaitResult</a>.</li>
  <li><a href="https://github.com/pirate">@pirate</a> added <a href="https://github.com/rust-lang/rust/pull/40765">contribution instructions to stdlib docs</a>.</li>
  <li><a href="https://github.com/tschottdorf">@tschottdorf</a> improved <a href="https://github.com/rust-lang/rust/pull/40413">error when violating <code class="highlighter-rouge">for&lt;'a&gt; T: 'a</code></a>.</li>
  <li><a href="https://github.com/GAJaloyan">@GAJaloyan</a> fixed <a href="https://github.com/rust-lang/rust/pull/40797">mistakes in the README.md file</a>.</li>
  <li><a href="https://github.com/icefoxen">@icefoxen</a> made <a href="https://github.com/rust-lang/rust/pull/40719">a tiny update to rustdoc CSS</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>, added <a href="https://github.com/rust-lang/rust/pull/40919">support for image, rules and footnotes in the new rustdoc markdown renderer</a>, fixed <a href="https://github.com/rust-lang/rust/pull/40608">mutex’s docs inconsistency</a> and added <a href="https://github.com/rust-lang/rust/pull/40634">more explanation on RefCell::get_mut</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/projektir">@projektir</a> added <a href="https://github.com/rust-lang/rust/pull/40866">linking for Once docs</a>, added <a href="https://github.com/rust-lang/rust/pull/40871">links for Atomics docs</a> and updated <a href="https://github.com/rust-lang/rust/pull/40828">rustdoc to accept <code class="highlighter-rouge">#</code> at the start of a markdown file</a>.</li>
  <li><a href="https://github.com/donniebishop">@donniebishop</a> modified <a href="https://github.com/rust-lang/rust/pull/40935"><code class="highlighter-rouge">str</code> structs descriptions</a>, linked <a href="https://github.com/rust-lang/rust/pull/40907">str in from_utf_unchecked</a>, added <a href="https://github.com/rust-lang/rust/pull/40824">a <code class="highlighter-rouge">fromStr</code> implementation example</a> and linked <a href="https://github.com/rust-lang/rust/pull/40819">ParseBoolError to from_str method of bool</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> synced <a href="https://github.com/rust-lang/rust/pull/40694">all unstable features with Unstable Book; add tidy lint.</a> and added <a href="https://github.com/rust-lang/rust/pull/40786">all unstable features to Unstable Book.</a>.</li>
  <li><a href="https://github.com/pirate">@pirate</a> added <a href="https://github.com/rust-lang/rust/pull/40763">helpful hint in io docs about how ? is not allowed in main()</a>.</li>
  <li><a href="https://github.com/topecongiro">@topecongiro</a> made <a href="https://github.com/rust-lang/rust/pull/40728">overlapping_inherent_impls lint a hard error</a>.</li>
  <li><a href="https://github.com/MaloJaffre">@MaloJaffre</a> removed <a href="https://github.com/rust-lang/rust/pull/40898">unused feature from error index generator</a> and avoid <a href="https://github.com/rust-lang/rust/pull/40901">linking to a moved page in rust.html</a>.</li>
  <li><a href="https://github.com/SamWhited">@SamWhited</a> improved <a href="https://github.com/rust-lang/rust/pull/40934">the docs for the write and writeln macros</a>.</li>
  <li><a href="https://github.com/DaseinPhaos">@DaseinPhaos</a> added <a href="https://github.com/rust-lang/rust/pull/40925">missing link in unstable-book</a>.</li>
  <li><a href="https://github.com/rap2hpoutre">@rap2hpoutre</a> added<a href="https://github.com/rust-lang/rust/pull/40904">example to std::process::abort</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> made <a href="https://github.com/rust-lang/rust/pull/40888">the rustdoc sidebar white on <code class="highlighter-rouge">src</code> pages</a>.</li>
  <li><a href="https://github.com/ctjhoa">@ctjhoa</a> improved <a href="https://github.com/rust-lang/rust/pull/40869">os::linux documentation</a>.</li>
  <li><a href="https://github.com/abonander">@abonander</a> memorized <a href="https://github.com/rust-lang/rust/pull/40814"><code class="highlighter-rouge">pub use</code>-reexported macros so they don’t appear twice in docs</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> clarified <a href="https://github.com/rust-lang/rust/pull/40816">suggetion for field used as method</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> fixed <a href="https://github.com/rust-lang/rust/pull/40897">typo in libcore/char.rs</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> updated <a href="https://github.com/rust-lang/rust/pull/40864">various book modules</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> improved <a href="https://github.com/rust-lang/rust/pull/40838">std::net docs</a>.</li>
  <li><a href="https://github.com/stepancheg">@stepancheg</a> documented <a href="https://github.com/rust-lang/rust/pull/40783">Cursor::new position is 0</a>.</li>
  <li><a href="https://github.com/TigleyM">@TigleyM</a> updated <a href="https://github.com/rust-lang/rust/pull/40682">docs for std::str</a>.</li>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/40853">broken Markdown and bad links in the error index</a> and fixed <a href="https://github.com/rust-lang/rust/pull/40852">compiler docs again</a>.</li>
  <li><a href="https://github.com/alanstoate">@alanstoate</a> changed <a href="https://github.com/rust-lang/rust/pull/40837">string references in asciiext</a>.</li>
  <li><a href="https://github.com/Wallacoloo">@Wallacoloo</a> fixed <a href="https://github.com/rust-lang/rust/pull/40833">typo in char::to_uppercase documentation</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> replaced <a href="https://github.com/rust-lang/rust/pull/40338">hoedown with pulldown in rustdoc</a>, replaced <a href="https://github.com/rust-lang/rust/pull/40338">hoedown with pull in rustdoc</a> and added <a href="https://github.com/rust-lang/rust/pull/40703">missing urls in ptr docs</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 5th of April 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>