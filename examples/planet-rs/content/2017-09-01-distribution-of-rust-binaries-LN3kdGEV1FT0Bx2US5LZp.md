+++
title = "Distribution Of Rust Binaries"
date = "2017-09-01T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/2017/09/01/distribution-of-rust-binaries.html"
+++
<h1 id="rust-ergonomics-101-fast-and-easy-access-to-the-tools">Rust ergonomics 101: fast and easy access to the tools</h1>

<h2 id="the-problem">The Problem</h2>
<p>One of the pain points often mentioned around rust is the long compilation
times. While this is certainly an issue I would like addressed, it is often
over-hyped in my opinion when talking about developing in rust.</p>

<p>However, when trying to download applications distributed in rust, the compile
times become a <em>huge pain</em> since the standard as of today seems to be to
distribute applications via cargo. <code class="language-plaintext highlighter-rouge">cargo install &lt;crate&gt;</code> requires the full
download and compilation of all crate dependencies, which can easily take more
than half an hour for several applications.</p>

<p>This is unnacceptable in my opinion, and it really harms rust as a language
that tools like <code class="language-plaintext highlighter-rouge">rustfix</code>, <code class="language-plaintext highlighter-rouge">cargo-edit</code>, etc have to be installed through
cargo. When I want to create a fake virtual env I require all my contributors
to have these in their local path – but setting up all these tools takes
upwards of 40 minutes on my laptop (!).</p>

<h2 id="one-possible-solution-0install">One possible solution: 0install</h2>
<p>The solution is simple: rust allows extremly easy cross compilation and
deployment of binaries through the amazing template <a href="https://github.com/japaric/trust">trust</a> which uses
<a href="https://github.com/japaric/cross">cross</a>. In the future I would like to make a script for setting up this
template, but for now it works quite well and <a href="https://github.com/vitiral/artifact">many</a> <a href="https://github.com/BurntSushi/ripgrep">rust</a> projects use
it.</p>

<p>However, distributing a binary is not enough – there needs to be an easy way
to install it for any platform – whether that be included in a build script,
a “virtual environment”, or a user’s desktop. I thought about going out and
creating such a tool and then I came accross <a href="http://0install.net/">0install</a>.</p>

<p>0install relies on only a <em>webpage</em> for knowing how to install software.
The webpage simply needs to include the xml data of the binaries and
dependencies – and <em>that’s it</em>.</p>

<p>I like the design philosophy of 0install, but I had an impossible time
<em>actually installing it</em> on my arch linux system. There is an <a href="https://github.com/0install/0install/issues/58">open bug</a>
about this. In addition, I opened <a href="https://github.com/0install/0install/issues/59">another one</a> requesting a pre-compiled
binary. Since 0install is written in OCaml, I don’t know how likely
this is to be fulfilled.</p>

<h2 id="another-solution-where-rust-can-shine">Another Solution: where rust can shine</h2>
<p>Distributing an easy to install binary is an area where rust can really shine.
I <a href="https://github.com/vitiral/artifact">already distribute artifact</a> as a pre-compiled binary and it is
works really well.</p>

<p>So this blog post might be the lanch of my next project: a simple binary
package manager written in rust. The requirements are similar to
0install but I am trying to make it even simpler. I call the tool
wpkg (“web package”, like wget but for package installation). I’ve started
the initial design approach and would love to know who is interested
and if anyone might want to join the effort.</p>

<p><a href="https://github.com/vitiral/wpkg">https://github.com/vitiral/wpkg</a></p>

<p>If you have an opinion (either positive or negative) please
<a href="https://github.com/vitiral/wpkg/issues">open an issue</a> and let me know!</p>

<h2 id="edit-nix-to-the-rescue">Edit: Nix to the rescue</h2>
<p>Although this doesn’t meet quite ALL my requirements (mainly that it still
requires packaging in the Nix repo), I am very interested in the <a href="https://nixos.org/nix/">Nix package
manager</a>. <a href="https://dzone.com/articles/isolated-development-environment-using-nix">This blog post</a> details how it can be used as a virtualenv
esq package manager.</p>