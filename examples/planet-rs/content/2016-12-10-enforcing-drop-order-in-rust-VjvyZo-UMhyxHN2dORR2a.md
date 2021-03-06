+++
title = "Enforcing drop order in Rust"
date = "2016-12-10T14:14:51+01:00"

[extra]
author = "aochagavia&apos;s blog"
link = "https://aochagavia.github.io/blog/enforcing-drop-order-in-rust/"
+++
<p>Some days ago I stumbled upon an old <a href="https://github.com/rust-lang/rfcs/issues/744">issue</a> on Rust&rsquo;s GitHub
repository. While the title of the issue is <em>&ldquo;should struct fields and array
elements be dropped in reverse declaration order (a la C++)&rdquo;</em>, the discussion
also extends to whether the current drop order should be stabilized.</p>

<p>Surprising as it may seem, drop order in Rust is unspecified and could
theoretically be changed in the future. However, there are use cases that
require enforcing a given drop order (see, for instance, <a href="http://stackoverflow.com/questions/41053542/forcing-the-order-in-which-struct-fields-are-dropped">this SO
question</a>).</p>

<p>In the current version of Rust (1.13), as well as in the nightlies, there is no
mechanism to statically control the drop order of the fields of a struct.
This leaves a programmer with two options:</p>

<ul>
<li>Wrapping fields in an <code>Option</code>-like enum.</li>
<li>Relying on the current (unspecified) drop order.</li>
</ul>

<p>This post will explore how both alternatives can be used. By experimenting
with a simple problem, we will try to explain the advantages and disadvantages
of both approaches. At the end, we will also share some final thoughts on
stabilizing drop order.</p>

<h2 id="formulating-the-problem">Formulating the problem</h2>

<p>Before defining the problem, we need a way to monitor the drop order of
struct fields. Here, we define a struct that prints a string. It will
be used in the code examples that are to follow.</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #75715e">// A struct that prints the contained `str` upon being dropped</span>
<span style="color: #66d9ef">struct</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">&#39;static </span><span style="color: #66d9ef">str</span><span style="color: #f8f8f2">);</span>

<span style="color: #66d9ef">impl</span><span style="color: #f8f8f2"> Drop </span><span style="color: #66d9ef">for</span><span style="color: #f8f8f2"> PrintDrop {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> drop(</span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> self) {</span>
<span style="color: #f8f8f2">        println</span><span style="color: #f92672">!</span><span style="color: #f8f8f2">(</span><span style="color: #e6db74">&quot;Dropping {}&quot;</span><span style="color: #f8f8f2">, self.</span><span style="color: #ae81ff">0</span><span style="color: #f8f8f2">)</span>
<span style="color: #f8f8f2">    }</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>With that out of the way, here is a minimal problem to consider. Suppose you
have the following struct, and would like <code>baz</code> to be dropped before <code>bar</code>.
If you fail to do so, Something Bad will happen.</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">struct</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">    bar</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">    baz</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<h2 id="alternative-one-wrapping-fields-in-an-option-like-enum">Alternative one: wrapping fields in an <code>Option</code>-like enum</h2>

<p>Wrapping your fields is pretty straightforward if you have seen the pattern
before. For the sake of simplicity, we just use an <code>Option</code>, though it would
be possible to write your own enum to make things a bit more ergonomic.</p>

<p>After introducing an <code>Option</code>, the struct looks like this:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">struct</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">    bar</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">    baz</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> Option</span><span style="color: #f92672">&lt;</span><span style="color: #f8f8f2">PrintDrop</span><span style="color: #f92672">&gt;</span><span style="color: #f8f8f2">,</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>With this new struct, you can write a <code>Drop</code> implementation that takes the
value out of the <code>Option</code> and drops it:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">impl</span><span style="color: #f8f8f2"> Drop </span><span style="color: #66d9ef">for</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> drop(</span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> self) {</span>
<span style="color: #f8f8f2">        </span><span style="color: #75715e">// Drop baz by replacing it</span>
<span style="color: #f8f8f2">        self.baz </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> None;</span>
<span style="color: #f8f8f2">    }</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>You can test the code by running it in a program with the following main
function:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> foo </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">        bar</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;bar&quot;</span><span style="color: #f8f8f2">),</span>
<span style="color: #f8f8f2">        baz</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> Some(PrintDrop(</span><span style="color: #e6db74">&quot;baz&quot;</span><span style="color: #f8f8f2">)),</span>
<span style="color: #f8f8f2">    };</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>The output below shows that <code>baz</code> is dropped first and <code>bar</code> second, which
was exactly our intention!</p>

<pre><code>Dropping baz
Dropping bar
</code></pre>

<h2 id="alternative-two-relying-on-the-current-unspecified-drop-order">Alternative two: relying on the current (unspecified) drop order</h2>

<p>Of course, it is also possible to find out in which order the fields are
dropped in the current version of Rust! It turns out that the fields are
dropped in the same order as they are declared.</p>

<p>In the case of <code>Foo</code>, this means that flipping the declaration of <code>baz</code> and
<code>bar</code> is exactly what we need:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">struct</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">    baz</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">    bar</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>After this change, we can verify that everything works correctly by running
the code with a main function similar to our previous one. In fact, we get
the following output:</p>

<pre><code>Dropping baz
Dropping bar
</code></pre>

<h2 id="which-one-should-you-use">Which one should you use?</h2>

<p>In my opinion, the wrapper type is the Right Way To Go &trade;, just because
you are not supposed to rely on unspecified behavior. The disadvantage of the latter approach seems
clear: a future version of the compiler implementing a different drop order
would break your program. This becomes even worse if you are authoring a
library, since a program that relies on it could potentially break just by
using a new version of the compiler. Even if you published a fix, it would
require <em>everyone</em> to update their dependencies.</p>

<p>On the other hand, it is undeniable that the wrapper approach has disadvantages
as well:</p>

<ul>
<li>It is clearly less ergonomic, because you can no longer use a normal type.</li>
<li>Each time you access the field you need to unwrap it, which means an extra
branch in your code unless the optimizer is smart enough.</li>
</ul>

<h2 id="final-thoughts">Final thoughts</h2>

<p>Given the drawbacks of having to use a runtime construct to enforce
a certain drop order, it would make sense to stabilize it. While there is
clearly consensus about the need for stabilization, it is not at all clear
whether the currently implemented drop order should be changed before it is
stabilized. The discussion is still open, as summarized by nrc in a <a href="https://github.com/rust-lang/rfcs/issues/744#issuecomment-231237499">comment</a>
on Rust&rsquo;s issue tracker.</p>