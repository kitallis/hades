+++
title = "Rst Server"
date = "2016-11-30T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/2016/11/30/rst-server.html"
+++
<h1 id="rst-03-released-now-with-a-web-ui-and-windows-linux-and-mac-osx-binaries">rst 0.3 released, now with a web ui and windows, linux and mac osx binaries!</h1>

<p>I have just released the 0.3beta release of <a href="https://github.com/vitiral/rst/releases">rst</a>,
the requirements tracking tool made for developers. This release brings many benefits,
including support for all major platforms and a web-ui for viewing requirements. In
addition, I have a clear path forward for how to make the web-ui be able to edit
the requirements as well.</p>

<p>There are many points I would like to cover in this post, in summary they are:</p>
<ul>
  <li>rst’s licensing changes</li>
  <li>learning a new programing langugage to write the front end in, <a href="http://elm-lang.org/">elm</a></li>
  <li>using <a href="https://github.com/nickel-org/nickel.rs">nickel.rs</a> and the
  <a href="https://github.com/ethcore/jsonrpc-core">jsonrpc_core</a> rust libraries
  for the backend</li>
  <li>packaging the static html files directly into the released rust binary</li>
</ul>

<p><strong>rst</strong>’s primary goal is to be <em>simple</em> – to make it easy to track requirements and
integrate into development tools. It will always run locally on your machine</p>

<h2 id="change-to-a-copy-left-license">Change to a copy-left license</h2>
<p>Just before the 0.3 release I switched rst’s license from the MIT permissive
license to the LGPLv3+ copy-left license and I would like to talk a little bit
about why.</p>

<p>rst is an application tool that I strongly feel developers need. Requirements
tracking and detailed design are core pillars to effective software development,
and yet there are really no tools to express these ideas that improve a
developer’s <em>day to day</em> work. In summary, there is nothing like git for
requirements tracking.</p>

<p>But there should be! What are requirements but a name,
a piece of text and links to other requirements? This is a simple problem with
a simple solution, but the industry has not pursued simplicity – all we’ve
gotten is large web tools with database backends.</p>

<p>By having your requirements be simple text files that live in your code,
you get another benefit – the requirements can easily link <strong>to</strong> your code
and be revision controlled <strong>with</strong> your code. This allows you to integrate
your requirements with tools you already use for tracking your code development
(github, Atlassian, etc).</p>

<p>Because rst’s very design is to enable simplicity, it was important to me
that the core library always remain open source. That is why I chose LGPL, it
allows propriety companies to use rst as a library and build on it, but if
they want new features then they need to share. Free developer tools should
remain free and companies that want to improve them should contribute.</p>

<p>On the other hand, I fully support using rst within a larger application,
and that is why I am using the “Lesser” GPL which allows others to build
on rst and why the format for the artifact files are public domain.</p>

<h2 id="learning-elm">Learning Elm</h2>
<p>The rst backend and cmd line tool are written in
<a href="https://www.rust-lang.org/en-US/">rust</a> a systems programming language that
runs blazingly fast, prevents segfaults, and guarantees thread safety. There
really is no replacing rust – it is an amazing language for the backend.
The safety guarantees make runtime errors practically impossible.</p>

<p>Elm is a functional programming language designed from the ground up to make
WebUIs that are maintainable and fun to program. I have to say they accomplished
all those goals, and learning elm after learning rust was pretty much a matter
of just learning new syntax. Many of the concepts that rust taught such as
<a href="http://rustbyexample.com/expression.html">expressions returning a value</a>
and <a href="http://rustbyexample.com/flow_control/match.html">pattern matching</a>
were utilized for exactly the same benefit in elm.</p>

<p>A frontend in elm with a backend in rust is a match made in heaven: fun,
performant and safe.</p>

<h2 id="rust-web-development">Rust web development</h2>
<p>Before I got too far, I would like to give a shoutout to
<a href="https://github.com/nickel-org/nickel.rs">nickel.rs</a> and
<a href="https://github.com/ethcore/jsonrpc-core">jsonrpc_core</a>. They made developing
a json-rpc backend about as easy as anyone could ask for.</p>

<p>What was really fun was learning how to package an elm/node app into
a rust binary – it turned out to be surprisingly easy! I used
<a href="https://github.com/casey/just">just</a> to create a simple makefile which
compiles my node/elm frontend into a tar file that I included in the source.
I then then use the <code class="language-plaintext highlighter-rouge">include_bytes!</code> macro to include the tar file directly in
the rust binary during the build. I then followed the directions on
<a href="https://github.com/japaric/rust-everywhere">rust-everywhere</a> to compile
rust for linux, mac and windows. It was a pain figuring out how to
package the node-app in a <em>cross platform</em> manner, but other than that
it went really smoothly!</p>

<p>With nickel, hosting was as simple as
<code class="language-plaintext highlighter-rouge">server.utilize(StaticFilesHandler::new(&amp;tmp_dir))</code>, where <code class="language-plaintext highlighter-rouge">tmp_dir</code> was
just the unpacked <a href="https://github.com/alexcrichton/tar-rs">tarfile</a>
in a temporary directory created by
<a href="https://github.com/rust-lang-nursery/tempdir">tempdir</a>. Big shoutout to
<a href="https://github.com/alexcrichton">alexcrichton</a>, along with your
<a href="https://github.com/alexcrichton/toml-rs">toml</a> library I have used an
incredible number of your tools for core development needs – you have
done an incredible amount to make rust useful for developing complex
applications!</p>

<p>If you want more details, check out the source code at
https://github.com/vitiral/rst.</p>

<h2 id="summary">Summary</h2>
<p>Huge steps have been made in the development of rst, with a read-only web-ui
deployed, and a fully supported one which includes editing just around the
corner. I simply cannot be thankful enough to the wonderful people who have made
rust and elm what they are today. These languages make development easy and
<em>fun</em>, and anything that I can do to support them I will do.</p>