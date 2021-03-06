+++
title = "Rust2018 And The Great Cli Awakening"
date = "2018-01-17T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/2018/01/17/rust2018-and-the-great-cli-awakening.html"
+++
<h1 id="rust2018-and-the-great-cli-awakening">#Rust2018 and the Great CLI Awakening</h1>
<p>This is a response to the <a href="https://blog.rust-lang.org/2018/01/03/new-years-rust-a-call-for-community-blogposts.html">#Rust2018</a> call for blog posts with a little bit of my
experience and how I see the 2018 year moving forward.</p>

<h2 id="the-experience-of-writing-a-cli-in-rust">The Experience of Writing a CLI in Rust</h2>
<blockquote>
  <p><strong>spoiler</strong>: simultaniously refreshing and painful</p>
</blockquote>

<p>Rust is a fantastic language for writing a Command Line Application (CLI). For the
ergonomics of hacking, it has one of the best <a href="https://github.com/TeXitoi/structopt">argument parsers
ever</a>, has seriously the <a href="https://github.com/serde-rs/serde">best serialization library ever</a>
and it compiles to almost any target and goes <em>fast</em> when it runs.</p>

<p>For testing and robustness, its type system prevents you from making stupid
errors, hooking into <a href="https://github.com/AltSysrq/proptest">fuzz-testing</a> is trivial, and there are <a href="https://github.com/killercup/assert_cli">better
and better</a> libraries for testing CLI applications. It is simply
a fantastic language and will be my goto for writing CLIs from now on (and I
come from python!).</p>

<p>However, there are some serious difficulties. The std library is small, and while
adding functionality from crates with <a href="https://github.com/killercup/cargo-edit">cargo-edit</a> is a simple
<code class="language-plaintext highlighter-rouge">cargo add</code> away, there are <em>so</em> many crates that make up a typical CLI, each
one requring you to add <code class="language-plaintext highlighter-rouge">extern crate foo</code> to your <code class="language-plaintext highlighter-rouge">main.rs</code> and then have to
write <code class="language-plaintext highlighter-rouge">use foo</code> at the top of every file you want to use it.</p>

<p>Probably the most significant paper cut is that rust???s error handling semantics
of <code class="language-plaintext highlighter-rouge">Result&lt;T, E&gt;</code> don???t work well for the CLI use case. In most cases a CLI
works like a compiler: I want to show the user <em>all</em> the things that are wrong,
not fail on the first error I find! There needs to be a standard crate for
handling a ???stream of errors and warnings???, probably via a channel, which I can
query ???were there errors in the last function????.</p>

<h2 id="filling-the-gaps">Filling the Gaps</h2>
<p>I have a strong desire to fill the gaps that exist in the ecosystem today. I am
currently doing a complete rewrite of my command line and web application
<a href="https://github.com/vitiral/artifact">artifact</a>, and am using that use case to ???fill the holes??? that I see.</p>

<p>Here are some of the things I am working on to fill the gaps. I would like
help from the community as well.</p>

<h3 id="gap-1-working-with-paths">Gap 1: Working with Paths</h3>
<p>I found working directly with paths to be <em>extremely annoying</em>. Does my path
exist? Is it equal to this other path? Is my path a file or a directory and why
can???t a type tell me? Why is creating/writing/appending a string to my path not
possible in a single line of code?</p>

<p>To solve this I wrote the crate <a href="https://github.com/vitiral/path_abs">path_abs</a>. It exports types which
guarantee that all paths exist and are absolute. Even better, it has <code class="language-plaintext highlighter-rouge">PathFile</code>
and <code class="language-plaintext highlighter-rouge">PathDir</code> types with commonly used methods attached to them.</p>

<p>Oh and did I mention that you can serialize the paths with serde? That was
one of my biggest pain points ??? if I have a path I can???t stick it in a struct
which I want to send anywhere. This library solves that and more.</p>

<p>This was one of my biggest pain points developing a CLI in rust, and it was
filled (by me???) in early 2018 ??? so the year is already looking good for
CLI ergonomics!</p>

<h3 id="gap-2-writing-tests">Gap 2: writing tests</h3>
<p>Rust makes it extremely simple to write <em>unit tests</em>, but once you want to
do a more end-to-end test there isn???t a lot out there. Unfortunately, testing
a CLI application requires end to end tests to be confident you are giving
the user the experience you expected.</p>

<p>If all your text is plain (not colored or formatted) then
<a href="https://github.com/killercup/assert_cli">assert_cli</a> is the crate for you. However, the second you put
style or color ??? or even a table ??? in your application it becomes a tedious
affair of editing a string of raw bytes every time you make a minor change to
your application.</p>

<p>I???m currently working on a CLI testing <em>framework</em> built in rust, using the new
crate I wrote called <a href="https://github.com/vitiral/termstyle">termstyle</a> to aid in being able to easily
express your styles (when you write your app) and then make it easy to test
them as well. The goal is to be able to write your CLI tests in a simple YAML
file and get clear diffs against the expected vs result. One of the benefits:
while you are writing your tests there is no compilation time if you are not
touching your source code ??? and running the tests takes almost no time at all!</p>

<blockquote>
  <p>Shoutout to <a href="https://github.com/colin-kiegel/rust-pretty-assertions">pretty_assertions</a> for making reading diffs
in regular rust tests SO much easier. This is a life saver for CLI
applications!</p>

  <p>Also shoutout to the <a href="https://doc.rust-lang.org/std/fmt/#sign0"><code class="language-plaintext highlighter-rouge">"{:#?}"</code></a> pretty printing formatter.  When
I discovered it I felt like the largest pain point of rust just vanished.</p>
</blockquote>

<h3 id="gap-3-working-with-errors">Gap 3: working with errors</h3>
<p>This one is a little bit more difficult and I am currently trying out solutions
in my rewrite of artifact.</p>

<p>The basic idea is this:</p>
<ul>
  <li>Instead of returning errors, I get passed a <a href="https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html"><code class="language-plaintext highlighter-rouge">Sender</code></a> object
where I push errors/warnings/info to that may or may not end up getting
displayed to the user. (note: I use a sender so that multithreading is
trivial)</li>
  <li>If I don???t get any useful work done, I return <code class="language-plaintext highlighter-rouge">None</code>, but there might have
been <em>several</em> errors that contributed to that instead of only one.</li>
  <li>The function above me checks for errors and can choose to exit early.
At the highest levels, the errors are sorted and displayed to the user.</li>
</ul>

<p>What I would like to do is leverage something like <a href="https://github.com/slog-rs/slog">slog</a> for this
purpose. <code class="language-plaintext highlighter-rouge">slog</code> has a similar design: it wants you to pass a ???log sender???
into every function/struct and log to that. What I need is a way to <em>query</em>
what logs have been passed (in particular whether they are errors). More
investigation is necessary, but I (personally) found slog very difficult to get
running. Maybe simply improving <code class="language-plaintext highlighter-rouge">slog</code>???s ergonomics could fill this gap and
simultaniously provide built-in logging?</p>

<h3 id="gap-4-bringing-it-all-together">Gap 4: bringing it all together</h3>
<p>The fourth major gap is the ergonomics <em>of actually using</em> the ecosystem. It is
often stated that rust???s ecosystem is immature. While this is somewhat true,
the real issue is in <em>finding</em> and <em>using</em> the pieces you need.</p>

<p>My final major project this year (along with artifact itself) is to flush out
the <a href="https://github.com/vitiral/stdcli">stdcli</a> crate. The goal (unlike the similar crate <a href="https://github.com/brson/stdx">stdx</a>) is
to have a <em>specific use case in mind</em> ??? which is to create a Command Line SDK.</p>

<p>I have made significant progress already, and will flush out the first release
as I rewrite artifact, but the primary goal is to reduce the number of <code class="language-plaintext highlighter-rouge">use foo</code>
and <code class="language-plaintext highlighter-rouge">external crate foo</code> statements that have to be made (especially regarding
traits), by providing a <em>standard prelude</em> for developing CLI applications. The
idea is that developing a CLI using stdcli should almost feel like a completely
different language. Everything you need should be directly within reach, and
the only crates you need to add should be application specific.</p>

<p>Unlike stdx, I plan on using the <code class="language-plaintext highlighter-rouge">&gt;=X.Y.Z</code> syntax for all dependencies. I then
plan on setting up something similar to Crater. If your application depends on
stdcli and you have a solid set of tests then you can have your project run
automated tests with it???s dependencies continuously updated. This will give
you a feeling of security that you can stay on the cutting edge, because stdcli
will be detecting any regressions and opening bugs with the relevant libraries.</p>

<h2 id="gap-5-stretch-distributing-your-application">Gap 5 (stretch): Distributing Your Application</h2>
<p>This one is a bit of a stretch, but I would like to have a simple way to
distribute applications written in rust that doesn???t require me to maintain a
build systems on Travis and Appveyor for 3 platforms. I am seriously looking at
the Nix project as the savior to all my woes. I don???t have much more to say
except that the rust team???s goal of making it easier to integrate cargo into
build system is <em>much</em> appreciated.</p>

<h2 id="conclusion">Conclusion</h2>
<p>I think CLI applications are (perhaps surprisingly) one of rust???s strong suites,
and it needs only a few relatively minor tweaks to make it the best language
for developing CLIs bar none.</p>

<p>The introduction of webassembly and being able to write client-side web
applications will only make this case <em>stronger</em>. Imagine being able to expose
a CLI application that can launch a static (or dynamic+backend) webpage
whenever that makes sense. Imagine <code class="language-plaintext highlighter-rouge">ps</code> (for monitoring processes), but you
could do <code class="language-plaintext highlighter-rouge">ps serve</code> and it would pop open a dynamically updating processor
status in your browser, with complex searching and graphing functionality built
in.</p>

<p>Rust will push this frontier. We are going to see more and more hybrid CLI/Web
applications being built every day and rust could be the language of choice. I
think it will be, and I think #Rust2018 will be what turns the tide.</p>