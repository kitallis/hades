+++
title = "This Week in Rust Docs 2"
date = "2016-05-01T00:00:00+00:00"

[extra]
author = "This week in Rust Docs"
link = "http://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-2"
+++
<p>Hello and welcome to <em>This Week in Rust Docs</em>!</p>

<p><em>This Week in Rust Docs</em> is openly developed <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs">on GitHub</a>.
If you find any errors in this week’s issue, <a href="https://github.com/GuillaumeGomez/this-week-in-rust-docs/pulls">please submit a PR</a>.</p>

<p>This week’s edition was edited by: <a href="https://github.com/GuillaumeGomez">GuillaumeGomez</a>.</p>

<h1 id="latest-news">Latest news</h1>

<p>Things are moving on: we talked about hosting crates’ documentation directly on an official doc website (the hostname hasn’t been decided yet). It has already been discussed a few times in the past but nothing came out. For now, we’re mostly trying to sort through all of the complexity. A lot of details must be taken in account and it is still at a very early stage of discussion but it started!</p>

<p>The <a href="https://github.com/rust-lang/rust/pull/32756">pull request</a> on the new rustc output is finally here (or almost)!</p>

<p><a href="https://github.com/jonathandturner">@jonathandturner</a> and <a href="https://github.com/peschkaj">@peschkaj</a> are working on a style guide in order to help developers to write better library documentations.</p>

<p>Besides this, <a href="https://github.com/peschkaj">@peschkaj</a> investigated the current doc status of the 20 biggest crates (based on number of downloads on <a href="https://crates.io">crates.io</a>). Take a look at the report <a href="https://facility9.com/2016/04/the-state-of-rust-docs-2016/">The State of Rust Docs</a>.</p>

<h1 id="current-opened-issues">Current opened issues</h1>

<p>For now, here are the issues opened for Rust documentation:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/issues/29329">The Standard Library Documentation Checklist</a></li>
  <li><a href="https://github.com/rust-lang/rust/issues/32777">Add error explanations for all error codes</a></li>
</ul>

<p>They both need help to move forward so any contribution is very welcome!</p>

<h1 id="recent-doc-contributions">Recent doc contributions</h1>

<ul>
  <li><a href="https://github.com/timothy-mcroy">@timothy-mcroy</a> added <a href="https://github.com/rust-lang/rust/pull/33229">E0434</a> and <a href="https://github.com/rust-lang/rust/pull/33294">E0501</a> errors explanation.</li>
  <li><a href="https://github.com/birkenfeld">@birkenfeld</a> improved <a href="https://github.com/rust-lang/rust/pull/33324">E0269</a> and <a href="https://github.com/rust-lang/rust/pull/33320">E0432</a> errors explanation and fixed <a href="https://github.com/rust-lang/rust/pull/33326">std::thread docs</a>. He also clarified <a href="https://github.com/rust-lang/rust/pull/33258">std::fmt dollar syntax</a>.</li>
  <li><a href="https://github.com/mrmiywj">@mrmiywj</a> improved <a href="https://github.com/rust-lang/rust/pull/33260">E0008 error explanation</a>.</li>
  <li><a href="https://github.com/bwinterton">@bwinterton</a> improved <a href="https://github.com/rust-lang/rust/pull/33276">BTreeSet::Insert</a> doc.</li>
  <li><a href="https://github.com/durka">@durka</a> worked on <a href="https://github.com/rust-lang/rust/pull/33250">tuple/unit structs</a> in the rustbook.</li>
  <li><a href="https://github.com/benaryorg">@benaryorg</a> clarified documentation of <a href="https://github.com/rust-lang/rust/pull/33167">TcpStream::connect for multiple valid addresses</a>.</li>
  <li><a href="https://github.com/fitzgen">@fitzgen</a> clarified <a href="https://github.com/rust-lang/rust/pull/33085">Iterator::enumerate doc example</a>.</li>
  <li><a href="https://github.com/GuillaumeGomez">@GuillaumeGomez</a>’s <a href="https://github.com/rust-lang/rust/pull/32230">Pull Request</a> to add the <code class="highlighter-rouge">--extend-css</code> option to rustdoc has been merged. He also worked on <a href="https://github.com/rust-lang/rust/pull/33283">process types documentation</a>.</li>
</ul>

<p>Besides all this, a lot of improvements have been made to rustdoc source code:</p>

<ul>
  <li><a href="https://github.com/rust-lang/rust/pull/33196">Linkify extern crates</a></li>
  <li><a href="https://github.com/rust-lang/rust/pull/33194">Improve accessibility of rustdoc pages</a></li>
  <li><a href="https://github.com/rust-lang/rust/pull/33191">Handle concurrent mkdir requests</a></li>
  <li><a href="https://github.com/rust-lang/rust/pull/33153">Only record the same impl once</a></li>
  <li><a href="https://github.com/rust-lang/rust/pull/33133">Inline all the impls</a></li>
</ul>

<h1 id="meetings">Meetings</h1>

<p>Next meeting will be on Wednesday 4th of May 2016 at 20:00 GMT on #rust-docs channel on irc.mozilla.org, feel free to come!</p>