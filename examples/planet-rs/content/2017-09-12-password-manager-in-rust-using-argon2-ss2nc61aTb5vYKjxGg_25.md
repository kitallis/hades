+++
title = "Password Manager In Rust Using Argon2"
date = "2017-09-12T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/2017/09/12/password-manager-in-rust-using-argon2.html"
+++
<h1 id="my-second-cli-application-in-rust-was-so-fun">My second CLI application in rust was so fun!</h1>

<p>I’ve had an itch to write a better password manager using my favorite
language (rust), and this last weekend I decided to give it a go.
I’ve spent the last year writing the <a href="https://github.com/vitiral/artifact">artifact</a> command line
application (which just hit 1.0, check it out!).</p>

<p>I had already written a “vault” password manager in python, called
<a href="https://github.com/vitiral/litevault">litevault</a> using scrypt to decrypt AND encrypt the passwords.
That was cool, but had several issues:</p>
<ul>
  <li>Didn’t run on python3. I’m pretty good at writing for both but
I kept hitting minor bugs and finally just gave up.</li>
  <li>Adding passwords was a pain – I wrote a helper to “merge” two
vaults but it was a nerve wracking experience.</li>
  <li>The thought of forgetting to commit the vault was scarry – the
passwords were randomly generated so they would be just <em>gone</em>.</li>
</ul>

<p>I have kept my eye on the application <a href="http://masterpasswordapp.com/">masterpassword</a> for some time, but
I’ve never been quite happy with its mixture of features. Why does it need
to know my name? What is up with the colorful symbols coresponding to a
password?</p>

<p>All I wanted was something that using a hash of the password for
validation purposes, and then site passwords are just a hash of the
same password. The setings, reference hash and existing sites should
be stored in a simple toml file, which can be put on github.</p>

<p>So I set out to write it and have completed the working implementation:
<a href="https://github.com/vitiral/novault">novault</a>.</p>

<h2 id="rust-is-da-best">Rust is da best</h2>
<p>The first thing that hit me was how quickly I was up and running.  I started by
thinking I would use SHA512 hashing and quickly <a href="https://github.com/sfackler/rust-openssl">found a library</a>. I used
<a href="https://www.google.com/search?q=rust+cargo+add&amp;ie=utf-8&amp;oe=utf-8">cargo-edit</a> to achieve an impresive pace of adding libraries – every
library was a quick cli command away. Having written a cli app before, I knew
what I wanted. I used a <a href="https://github.com/vitiral/stdcli">list of cli libraries</a> I had compiled to cut
through my requirements like butter. Within a few hours of work I had a working
prototype that compiled and <em>actually ran</em>.</p>

<p>I spent the next few hours cleaning up the docs. I discovered that maybe SHA512
wasn’t secure enough for storing the hash in plain-text on the internet so
instead settled on <a href="https://en.wikipedia.org/wiki/Argon2">Argon2, the winner of the 2015 Password Hashing
Competition</a>. The <code class="language-plaintext highlighter-rouge">argon2rs</code> library was a quick <code class="language-plaintext highlighter-rouge">cargo add</code> away, and the docs
could be read with <code class="language-plaintext highlighter-rouge">cargo doc --open -p argon2rs</code>.</p>

<p>I refactored the library to use types for everything I cared about. It is now
IMPOSSIBLE to serialize the user’s master password (outside of <code class="language-plaintext highlighter-rouge">secure.rs</code>), and
serializing a site password (even PRINTING it) requires using it’s
<code class="language-plaintext highlighter-rouge">password.audit_this</code> attribute.</p>

<h2 id="the-application">The Application</h2>
<p>The NoVault application itself is now in a pretty good state.  I would really
appreciate some people more versed in security to give it an audit, as well as
any contributions or tests anyone wants to contribute. It can currently be
installed by installing rust and using</p>

<div class="language-plaintext highlighter-rouge"><div class="highlight"><pre class="highlight"><code>cargo install novault
</code></pre></div></div>

<p>I will probably start actually using it in the comming month. For now I am
just using it to store my reddit password, which is <a href="https://github.com/vitiral/dotfiles">on github</a>. Feedback
and audits would be great!</p>