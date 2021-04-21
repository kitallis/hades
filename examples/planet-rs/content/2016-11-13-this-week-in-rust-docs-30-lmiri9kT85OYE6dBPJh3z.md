+++
title = "This Week in Rust Docs 30"
date = "2016-11-13T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-30"
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

<p>This week’s edition was edited by <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>Since new rustc version is out, the controversial changes on docs have been merged:</p>

<ul>
  <li>Fold <a href="https://github.com/rust-lang/rust/pull/37728">fields for enum struct variants into a docblock in rustdoc</a></li>
  <li>Add <a href="https://github.com/rust-lang/rust/pull/37190">line breaks to where clauses a la rustfmt in rustdoc</a>.</li>
  <li>Print <a href="https://github.com/rust-lang/rust/pull/37134">more tags in rustdoc</a>.</li>
</ul>

<p>Don’t hesitate to give your feedbacks on them!</p>

<p>The way rustdoc is creating urls is problematic for the moment. A good summary of this issue can be found <a href="https://github.com/rust-lang/rust/issues/36417">here</a>. A few members of the Rust Doc team are preparing an RFC in order to improve this. If you want to get involved, feel free to speak about it with <a href="https://github.com/GuillaumeGomez">Guillaume Gomez</a> (imperio on IRC).</p>

<p>Please take a look <a href="https://users.rust-lang.org/t/reminder-planning-the-next-rust-doc-days/6901">to the next rust doc days planning reminder</a>.</p>

<p>The topic to propose crates for the Rust Doc Days is still open and waiting for contributions <a href="https://users.rust-lang.org/t/call-for-proposals-for-next-rust-doc-days-crates/6685">here</a>. Please take a look!</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the three big issues for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/35233">Error code list which need to be updated to new format</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They all need help to move forward so any contribution is very welcome!</p>

<p>There are currently around 70 other documentation issues opened. Look for <a href="https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs">A-docs</a> tagged issues on GitHub!</p>

<h1 id="call-for-participation">Call for participation</h1>

<p>There’s now a call for participation to <a href="https://github.com/rust-lang/rust/issues/33772">display all methods of a type, even those from implicit traits in rustdoc</a>. This is a great way to help users find everything that a type can do. Any help on it would be very appreciated!</p>

<h1 id="waiting-for-merge">Waiting for merge</h1>

<ul>
  <li><a href="https://github.com/estebank">@estebank</a> added <a href="https://github.com/rust-lang/rust/pull/36409">the detection for missing <code class="highlighter-rouge">;</code> on methods with return type <code class="highlighter-rouge">()</code></a>, provided <a href="https://github.com/rust-lang/rust/pull/37442">hint when cast needs a dereference</a>, added <a href="https://github.com/rust-lang/rust/pull/37369">multiline spans in full if short enough</a>, disallowed <a href="https://github.com/rust-lang/rust/pull/37548">‘start’ feature on nested function in E0526</a> and added the <a href="https://github.com/rust-lang/rust/pull/34420">detection for double reference when applying binary op</a>.</li>
  <li><a href="https://github.com/keeperofdakeys">@keeperofdakeys</a> improved <a href="https://github.com/rust-lang/rust/pull/37749">the #[should_panic] feature</a>.</li>
  <li><a href="https://github.com/jedireza">@jedireza</a> fixed <a href="https://github.com/rust-lang/rust/pull/37743">grammar typos in ffi.md</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> removed <a href="https://github.com/rust-lang/rust/pull/37057">all “consider using an explicit lifetime parameter” suggestions</a>.</li>
  <li><a href="https://github.com/jfirebaugh">@jfirebaugh</a> removed <a href="https://github.com/rust-lang/rust/pull/37058">long diagnostic for E0002</a> and added <a href="https://github.com/rust-lang/rust/pull/37242">a distinct error code and description for “main function has wrong type”</a>.</li>
  <li><a href="https://github.com/dns2utf8">@dns2utf8</a> added <a href="https://github.com/rust-lang/rust/pull/37607">grammar verification build command</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37658">ref suggestion</a>, improved <a href="https://github.com/rust-lang/rust/pull/37375">reference cast help message</a>, added <a href="https://github.com/rust-lang/rust/pull/36320">information in case of markdown block code test failure</a> and started <a href="https://github.com/rust-lang/rust/pull/37388">implementation of proposal for E0308</a>.</li>
</ul>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/johnthagen">@johnthagen</a> added <a href="https://github.com/rust-lang/rust/pull/37386">example using Self to reference</a>.</li>
  <li><a href="https://github.com/brson">@brson</a> added <a href="https://github.com/rust-lang/rust/pull/37600">changelog for 1.13.0</a> and removed <a href="https://github.com/rust-lang/rust/pull/37601">platform compatibility table from the Book, link to the forge</a>.</li>
  <li><a href="https://github.com/estebank">@estebank</a> fixed <a href="https://github.com/rust-lang/rust/pull/37531">invalid “ref mut mut” sugestion</a>, included <a href="https://github.com/rust-lang/rust/pull/37370">type of missing trait methods in error</a>, added <a href="https://github.com/rust-lang/rust/pull/37695">note “how to escape” on fmt string with unescaped <code class="highlighter-rouge">{</code></a>, removed <a href="https://github.com/rust-lang/rust/pull/37481">hint to add lifetime on impl items</a>, grouped <a href="https://github.com/rust-lang/rust/pull/37456">unused import warnings per import list</a>, showed <a href="https://github.com/rust-lang/rust/pull/37447">one error for duplicated type definitions</a>, pointed <a href="https://github.com/rust-lang/rust/pull/37428">to type argument span when used as trait</a> and reworded <a href="https://github.com/rust-lang/rust/pull/36520">error when data-less enum variant called as function</a>.</li>
  <li><a href="https://github.com/mikhail-m1">@mikhail-m1</a> improved <a href="https://github.com/rust-lang/rust/pull/37554">“Doesn’t live long enough” error</a>.</li>
  <li><a href="https://github.com/liigo">@liigo</a> <a href="https://github.com/rust-lang/rust/pull/37250">marked unsafe fns in module page with superscript icons in rustdoc</a>.</li>
  <li><a href="https://github.com/xfix">@xfix</a> matched <a href="https://github.com/rust-lang/rust/pull/37483">guessing game output to newest language version</a>.</li>
  <li><a href="https://github.com/steveklabnik">@steveklabnik</a> made <a href="https://github.com/rust-lang/rust/pull/35102">it clear that the reference isn’t normative</a>.</li>
  <li><a href="https://github.com/joshtriplett">@joshtriplett</a> documented <a href="https://github.com/rust-lang/rust/pull/37472">convention for using both fmt::Write and io::Write</a>.</li>
  <li><a href="https://github.com/sinkuu">@sinkuu</a> made <a href="https://github.com/rust-lang/rust/pull/36615">E0243/E0244 message consistent with E0107</a>.</li>
  <li><a href="https://github.com/QuietMisdreavus">@QuietMisdreavus</a> folded <a href="https://github.com/rust-lang/rust/pull/37728">fields for enum struct variants into a docblock in rustdoc</a> and <a href="https://github.com/rust-lang/rust/pull/37190">added line breaks to where clauses a la rustfmt in rustdoc</a>.</li>
  <li><a href="https://github.com/Mark-Simulacrum">@Mark-Simulacrum</a> added <a href="https://github.com/rust-lang/rust/pull/37527">Error implementation for std::sync::mpsc::RecvTimeoutError.</a>.</li>
  <li><a href="https://github.com/wesleywiser">@wesleywiser</a> added <a href="https://github.com/rust-lang/rust/pull/37662">documentation to some of the unstable intrinsics</a>.</li>
  <li><a href="https://github.com/nwin">@nwin</a> removed <a href="https://github.com/rust-lang/rust/pull/37503">remark about poor code style</a>.</li>
  <li><a href="https://github.com/trotter">@trotter</a> updated <a href="https://github.com/rust-lang/rust/pull/37368">testing.md to reflect changes to cargo new</a>.</li>
  <li><a href="https://github.com/tshepang">@tshepang</a> fixed <a href="https://github.com/rust-lang/rust/pull/37680">doc typo</a>.</li>
  <li><a href="https://github.com/est31">@est31</a> documented <a href="https://github.com/rust-lang/rust/pull/37664">the question mark operator in reference and the Book’s syntax index</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a> added <a href="https://github.com/rust-lang/rust/pull/37588">missing urls on io structs</a>, printed <a href="https://github.com/rust-lang/rust/pull/37134">more tags in rustdoc</a>, added <a href="https://github.com/rust-lang/rust/pull/37669">missing urls for FusedIterator and TrustedLen traits</a>, added <a href="https://github.com/rust-lang/rust/pull/37698">missing urls for marker’s traits</a>, added <a href="https://github.com/rust-lang/rust/pull/37716">missing mem urls</a>, fixed <a href="https://github.com/rust-lang/rust/pull/37727">invalid src url</a>, added <a href="https://github.com/rust-lang/rust/pull/37627">missing urls and made few local rewrites</a> and added <a href="https://github.com/rust-lang/rust/pull/37650">missing urls for Sum and Product traits</a>.</li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 16th of November 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org. Feel free to come!</p>