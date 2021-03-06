+++
title = "This Week in Rust Docs 51"
date = "2017-04-09T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-51"
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

<p>The markdown renderer in rustdoc <a href="https://github.com/rust-lang/rust/pull/40338">has been changed</a>! We replaced <a href="https://github.com/hoedown/hoedown">hoedown</a> with <a href="https://github.com/google/pulldown-cmark">pulldown-cmark</a>. Bugs might appear after this switch so any feedback is very welcomed!</p>

<p>However, thanks to everyone’s efforts, all known issues have been fixed. Don’t forget to check if your document is common-mark compliant!</p>

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
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> updated <a href="https://github.com/rust-lang/rust/pull/41084">formatting of fn signatures and where clauses to match style rfcs in rustdoc</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> pointed <a href="https://github.com/rust-lang/rust/pull/40857">at fields that make the type recursive</a>, added <a href="https://github.com/rust-lang/rust/pull/40565">an explicit help message for binop type mismatch</a>, added <a href="https://github.com/rust-lang/rust/pull/41136">end line display of multiline annotations</a> and used <a href="https://github.com/rust-lang/rust/pull/41087">proper span for tuple index parsed as float</a>.</li>
  <li><a href="https://github.com/frewsxcv">@frewsxcv</a> made a <a href="https://github.com/rust-lang/rust/pull/40653">couple minor improvements for tidy error handling</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40706">documentation of next_power_of_two</a>.</li>
  <li><a href="https://github.com/sagebind">@sagebind</a> improved <a href="https://github.com/rust-lang/rust/pull/41008">examples for ThreadId</a>.</li>
  <li><a href="https://github.com/stepancheg">@stepancheg</a> improved <a href="https://github.com/rust-lang/rust/pull/40747">BufRead::is_eof documentation</a>.</li>
  <li><a href="https://github.com/maccoda">@maccoda</a> improved <a href="https://github.com/rust-lang/rust/pull/40987">Convert docs</a>.</li>
  <li><a href="https://github.com/mgattozzi">@mgattozzi</a> updated <a href="https://github.com/rust-lang/rust/pull/40812"><code class="highlighter-rouge">Child</code> docs to not have a note section</a> and updated <a href="https://github.com/rust-lang/rust/pull/40829">ChildStderr docs to be clearer</a>.</li>
  <li><a href="https://github.com/pirate">@pirate</a> added <a href="https://github.com/rust-lang/rust/pull/40765">contribution instructions to stdlib docs</a>.</li>
  <li><a href="https://github.com/icefoxen">@icefoxen</a> made <a href="https://github.com/rust-lang/rust/pull/40719">a tiny update to rustdoc CSS</a>.</li>
  <li><a href="https://github.com/projektir">@projektir</a> added <a href="https://github.com/rust-lang/rust/pull/41103">channel error docs</a>.</li>
  <li><a href="https://github.com/lukaramu">@lukaramu</a> improved <a href="https://github.com/rust-lang/rust/pull/41125">std::hash docs</a>.</li>
  <li><a href="https://github.com/palango">@palango</a> improved <a href="https://github.com/rust-lang/rust/pull/41122">module documentation for std::f32 and std::f64</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a> and added <a href="https://github.com/rust-lang/rust/pull/40634">more explanation on RefCell::get_mut</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/ollie27">@ollie27</a> fixed <a href="https://github.com/rust-lang/rust/pull/41111">Markdown issues in the docs</a> and used <a href="https://github.com/rust-lang/rust/pull/41112">pulldown-cmark for Markdown HTML rendering in rustdox</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> suggested <a href="https://github.com/rust-lang/rust/pull/40775">using enum when a variant is used as a type</a>, identified <a href="https://github.com/rust-lang/rust/pull/40815">missing item category in <code class="highlighter-rouge">impl</code>s</a> and removed <a href="https://github.com/rust-lang/rust/pull/41062">recommendations of private fields called as method</a>.</li>
  <li><a href="https://github.com/mandeep">@mandeep</a> fixed <a href="https://github.com/rust-lang/rust/pull/41019">typo in doc comments for swap_remove</a>.</li>
  <li><a href="https://github.com/stjepang">@stjepang</a> added <a href="https://github.com/rust-lang/rust/pull/40927">a note about overflow for fetch_add/fetch_sub</a> and improved <a href="https://github.com/rust-lang/rust/pull/40949">some docs for VecDeque</a>.</li>
  <li><a href="https://github.com/irfanhudda">@irfanhudda</a> improved <a href="https://github.com/rust-lang/rust/pull/40999">option API docs</a>.</li>
  <li><a href="https://github.com/SimonSapin">@SimonSapin</a> fixed <a href="https://github.com/rust-lang/rust/pull/41014">link in std::thread docs</a>.</li>
  <li><a href="https://github.com/pgerber">@pgerber</a> improved <a href="https://github.com/rust-lang/rust/pull/41007">documentation for <code class="highlighter-rouge">std::fs::DirBuilder</code></a>.</li>
  <li><a href="https://github.com/donniebishop">@donniebishop</a> added <a href="https://github.com/rust-lang/rust/pull/40997">links to types in from_utf8 description</a> and added <a href="https://github.com/rust-lang/rust/pull/40992">links to from_utf8 methods in Utf8Error</a>.</li>
  <li><a href="https://github.com/Technius">@Technius</a> added <a href="https://github.com/rust-lang/rust/pull/40981">links and some examples to std::sync::mpsc docs</a>.</li>
  <li><a href="https://github.com/projektir">@projektir</a> updated <a href="https://github.com/rust-lang/rust/pull/40977">the description for BarrierWaitResult</a>.</li>
  <li><a href="https://github.com/GAJaloyan">@GAJaloyan</a> fixed <a href="https://github.com/rust-lang/rust/pull/40797">mistakes in the README.md file</a>.</li>
  <li><a href="https://github.com/japaric">@japaric</a> documented <a href="https://github.com/rust-lang/rust/pull/41135">some existing unstable features</a>.</li>
  <li><a href="https://github.com/euclio">@euclio</a> collapsed <a href="https://github.com/rust-lang/rust/pull/41131">docblock before showing label in rustdoc</a>.</li>
  <li><a href="https://github.com/rap2hpoutre">@rap2hpoutre</a> added <a href="https://github.com/rust-lang/rust/pull/41090">example to std::process::abort</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> fixed <a href="https://github.com/rust-lang/rust/pull/41066">links</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/40919">support for image, rules and footnotes in the new rustdoc markdown renderer</a>, fixed <a href="https://github.com/rust-lang/rust/pull/40608">mutex’s docs inconsistency</a> and replaced <a href="https://github.com/rust-lang/rust/pull/41043"><code class="highlighter-rouge">^</code> with <code class="highlighter-rouge">&lt;sup&gt;</code> html balise</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 12th of April 2017 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>