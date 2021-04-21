+++
title = "Artifact 2.0 Rust Full Stack Web And Cli Application"
date = "2018-07-16T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/2018/07/16/artifact-2.0-rust-full-stack-web-and-cli-application.html"
+++
<p>After at least 6 months of effort (and interruptions… so many interruptions)
I have finally completed the complete rewrite of artifact – including
rewriting the Web UI to be in rust compiled into web assembly. <a href="https://github.com/vitiral/artifact/releases">2.0 is
complete and you can download it here (for linux only
ATM</a>)</p>

<p>This release spans a huge amount of effort. The TLDR version:</p>
<ul>
  <li>I felt like I needed a cohesive CLI SDK and so I created the
<a href="https://github.com/rust-crates/ergo">ergo</a> crate. This included several
other crates created by me:
    <ul>
      <li><a href="https://github.com/vitiral/path_abs">path_abs</a> for better path handling.</li>
      <li><a href="https://github.com/vitiral/std_prelude">std_prelude</a> for glob importing what I need.</li>
      <li><a href="https://github.com/vitiral/termstyle">termstyle</a> for cleaner terminal styles.</li>
      <li><a href="https://github.com/vitiral/taken">taken</a> for taking ownership.</li>
      <li><a href="https://github.com/vitiral/stfu8">stfu8</a> for serializing/deserializing
arbitrary bytes that are “mostly” utf8 compliant (i.e. paths).</li>
    </ul>
  </li>
  <li>I used <a href="https://github.com/DenisKolodin/yew">yew</a> for developing the frontend. I ended up
contributing a fair amount to both yew and <a href="https://github.com/koute/stdweb">stdweb</a> but
these frameworks were <em>surprisingly stable</em>. I also made a few crates:
    <ul>
      <li><a href="https://github.com/vitiral/jrpc">jrpc</a>: for easily creating JSON-RPC
requests and handling responses.</li>
      <li><a href="https://github.com/vitiral/yew_simple">yew_simple</a>: a simpler (in my
opinion) imlementation of several yew services. It is important to note
that yew is flexible enough that this is even possible :)</li>
    </ul>
  </li>
</ul>

<p>What I ended up with is the first (to my knowledge) full stack application in
rust that is more than a toy. Artifact is a reasonably complex piece of software,
and rust delivers its promises of speed and stability even at the cutting edge of
the web.</p>

<h2 id="why-rewrite-everything">Why Rewrite Everything?</h2>
<p>The first question is probly why I decided a complete rewrite was necessary.
There are two answers: for fun and because I initially wrote artifact when I
was just learning rust for the first time and it <em>showed</em>.</p>

<p>I wanted to write artifact “the right way” – using standardized libraries and
split into several reusable crates. However, the standardized those libraries
didn’t really exist yet (or at least they weren’t cohesive) so I went about
building ergo. It was my way of giving back to the fantastic rust
community :)</p>

<p>The other reasons to rewrite were:</p>
<ul>
  <li>Speed: I wanted to make the implementation completely parallel, and I did.
It is <em>blazingly</em> fast now.</li>
  <li>Simplicity: the filesystem interface now has an API which acts like a CRUD
database. You simply give it a list of operations (create/modify/delete) and
it makes sure they are valid and consistent and performs them. This made
implementing the CLI and Server layers trivial, as they could both use this
simple interface.</li>
  <li>New Features: I added several new features that the old library made
difficult to add and test.
    <ul>
      <li>Sub Artifacts: you can add sub-artifacts using <code class="language-plaintext highlighter-rouge">[[.subart]]</code> syntax in the
text of any artifact, making it possible to split up the definition
of how to implement the artifact in source code. This also includes
test subarts (<code class="language-plaintext highlighter-rouge">[[.tst-subart]]</code>).</li>
      <li>Markdown format: the default way to specify artifacts is now using
an extended form of commonmark markdown.</li>
      <li>The docs (including the Simple Quality book) are now distributed directly
with the web app.</li>
    </ul>
  </li>
</ul>

<h2 id="how-was-wasm--yew">How was WASM / yew?</h2>
<p>Rewriting the frontend in rust using yew (and compiling to webassembly) was
truly fun and exciting. There was definitely a few areas that needed
improvement; and being willing to contribute to fix your own issues was
<em>critical</em> with the ecosystem where it is at now – but the responsiveness of
the rust developers has never ceased to amaze me.</p>

<p>Probably the most amazing thing about writing your whole stack in a single
langauge is code reuse. I have a single crate, <code class="language-plaintext highlighter-rouge">artifact-ser</code> with the
serialization datatypes and logic that can be used anywhere in the stack.
This allows me to implement code once and use it anywhere – a benefit
that can seriously not be overstated!</p>

<h1 id="conclusion">Conclusion</h1>
<p>I love rust, it is definitely my favorite language at the moment. Writing
a full stack application in pure rust has been an amazing experience.</p>

<p>The highlights however are definitely my interactions with the community. After
my <a href="https://vitiral.github.io/2018/01/17/rust2018-and-the-great-cli-awakening.htm">Rust and the Great CLI
Awakening</a>
blog post I was invited to be on the <a href="https://github.com/rust-lang-nursery/cli-wg">CLI Working
Group</a>. Unfortunately, that
pulled too much of my time away from rewriting artifact so I had to drop out.
However, my experience there and with the WASM folks has been nothing but
positive. Its been said before and I’ll say it again: Rust has a great
community!</p>

<p>There is still much work to be done for artifact. For one thing I need to
figure out how to run <code class="language-plaintext highlighter-rouge">cargo-web</code> in CI for mac and windows so that I can
distrubute artifact for those platforms. I also need to finish end to end
testing (including using frontend testing using rust, more on that later!).</p>

<p>However, one of the primary distractions over the last several months has
been interviewing at Google… and the distractions paid off! I will be
starting as a Test Engineer at Google working on <a href="http://fortune.com/2018/05/07/volvo-google-assistant-cars/">Android
Auto</a>. This is
super exciting, but will not leave a lot of time for working on artifact.
Therefore this blog post is basically also a notice that I will be on haitus
for the next several months as I get up to speed on everything Googly.</p>

<p>If you find artifact valuable, please leave
<a href="https://vitiral.github.io/artifact/docs/Feedback.html">feedback</a>. Help
in maintaining it would also be much appreciated :).</p>