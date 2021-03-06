+++
title = "Exploring Rust's unspecified drop order"
date = "2016-12-12T14:14:51+01:00"

[extra]
author = "aochagavia&apos;s blog"
link = "https://aochagavia.github.io/blog/exploring-rusts-unspecified-drop-order/"
+++
<p>After my previous <a href="https://aochagavia.github.io/blog/enforcing-drop-order-in-rust/">post</a>, I thought it would be
interesting to run some experiments to determine the unspecified drop order
within different constructs of Rust. After you read this, I guarantee you will
understand why there is so much <a href="https://github.com/rust-lang/rfcs/issues/744">discussion</a> about changing
the current drop order before stabilizing it :)</p>

<p><strong>TLDR:</strong> the current drop order is really weird!</p>

<p>In this post we are going to look at:</p>

<ul>
<li>Local variables</li>
<li>Tuples</li>
<li>Structs and enums</li>
<li>Slices</li>
<li>Closure captures</li>
</ul>

<p>We will be reusing the <code>PrintDrop</code> struct, so here is the definition
in case you forgot it:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">struct</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">&#39;static </span><span style="color: #66d9ef">str</span><span style="color: #f8f8f2">);</span>

<span style="color: #66d9ef">impl</span><span style="color: #f8f8f2"> Drop </span><span style="color: #66d9ef">for</span><span style="color: #f8f8f2"> PrintDrop {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> drop(</span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> self) {</span>
<span style="color: #f8f8f2">        println</span><span style="color: #f92672">!</span><span style="color: #f8f8f2">(</span><span style="color: #e6db74">&quot;Dropping {}&quot;</span><span style="color: #f8f8f2">, self.</span><span style="color: #ae81ff">0</span><span style="color: #f8f8f2">)</span>
<span style="color: #f8f8f2">    }</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Now we are ready to go!</p>

<h2 id="local-variables">Local variables</h2>

<p>Let&rsquo;s start with the following piece of code, testing the drop order of local
variables:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> x </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> y </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">);</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>And the output is&hellip;</p>

<pre><code>Dropping y
Dropping x
</code></pre>

<p>As you can see, local variables are dropped in the reverse order of their
declaration. This should come as no surprise, since new objects can store
references to previously declared ones. Therefore a different drop
order would result in dangling pointers.</p>

<p>The drop order of function parameters is similar, so the first parameter
in the list is the last one to be dropped. The code is omitted for the sake
of brevity, but this behavior can be trivially reproduced by writing a
function with two by-value parameters.</p>

<p>If you think about drop order from the perspective of data structures, the
behavior of local variables resembles the way a stack works. You
could say that they are pushed onto the stack and popped at the
end of the scope.</p>

<h2 id="tuples">Tuples</h2>

<p>After seeing the stack-like behavior of local variables, one would expect to
see something similar in other constructs. However, tuples have a little
surprise for us&hellip;</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> tup </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> (PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">), PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">), PrintDrop(</span><span style="color: #e6db74">&quot;z&quot;</span><span style="color: #f8f8f2">));</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>And the output is&hellip;</p>

<pre><code>Dropping x
Dropping y
Dropping z
</code></pre>

<p>Wait! Are you telling me that the variables are dropped in the same order as
they are declared? So it seems! To continue with the data structures story,
we could say that tuples behave like a queue, in which elements are enqueued
in their order of appearance and dequeued at the end of the scope.</p>

<p>But this is not all! There is a subtle surprise lurking around
the corner&hellip; If there is a <code>panic</code> during construction of the tuple, the drop
order is reversed! If you don&rsquo;t believe me, just run the code below:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> tup </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> (PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">), PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">), panic</span><span style="color: #f92672">!</span><span style="color: #f8f8f2">());</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>As I told you, the output is:</p>

<pre><code>Dropping y
Dropping x
</code></pre>

<p>In other words, a tuple shows a queue-like drop order, unless one of the
expressions in the tuple constructor panics. In case
of a panic during construction, the drop order will be stack-like!</p>

<p><strong>EDIT:</strong> as pointed out by <a href="https://www.reddit.com/r/rust/comments/5hw00k/exploring_rusts_unspecified_drop_order/db3ejx0/">birkenfeld</a> on Reddit, the stack-like
drop order actually makes sense in case of a panic. There is at this stage no tuple!
Therefore, the expressions are dropped according to the rules of local variables.</p>

<h2 id="structs-and-enums">Structs and enums</h2>

<p>Structs present the same weird behavior as tuples. To a certain
extent this seems consistent, since a struct is arguably a tuple with named
fields instead of indices.
It seems logical that they share the same drop order. The same holds for enums.</p>

<p>For the sake of brevity, the code below only tests the drop order of a struct.
Of course, the same behavior is expected from tuple structs, tuple enum
variants and struct enum variants.</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">struct</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">    x</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">    y</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">    z</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop,</span>
<span style="color: #f8f8f2">}</span>

<span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> foo </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">        x</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">),</span>
<span style="color: #f8f8f2">        y</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">),</span>
<span style="color: #f8f8f2">        z</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;z&quot;</span><span style="color: #f8f8f2">),</span>
<span style="color: #f8f8f2">    };</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>And the output is&hellip;</p>

<pre><code>Dropping x
Dropping y
Dropping z
</code></pre>

<p>Again, the order is reversed when a panic occurs during construction:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> foo </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> Foo {</span>
<span style="color: #f8f8f2">        x</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">),</span>
<span style="color: #f8f8f2">        y</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">),</span>
<span style="color: #f8f8f2">        z</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> panic</span><span style="color: #f92672">!</span><span style="color: #f8f8f2">(),</span>
<span style="color: #f8f8f2">    };</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>As can be observed in the output below:</p>

<pre><code>Dropping y
Dropping x
</code></pre>

<p>Looking at things from the bright side, at least we can say that this
behavior is consistent across all data types. Still, it feels completely
arbitrary to use a queue-like drop order.</p>

<h2 id="slices">Slices</h2>

<p>Slices show the same queue-like behavior under normal circumstances and a
stack-like behavior in the presence of a panic during construction.</p>

<p>Panic-free construction:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> xs </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> [PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">), PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">), PrintDrop(</span><span style="color: #e6db74">&quot;z&quot;</span><span style="color: #f8f8f2">)];</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Results in queue-like drop order:</p>

<pre><code>Dropping x
Dropping y
Dropping z
</code></pre>

<p>Panic during construction:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> xs </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> [PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">), PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">), panic</span><span style="color: #f92672">!</span><span style="color: #f8f8f2">()];</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Results in stack-like drop order:</p>

<pre><code>Dropping y
Dropping x
</code></pre>

<p>Interestingly, <code>Vec&lt;T&gt;</code> shows the same drop order. As we are used to, a panic
in the <code>vec![]</code> macro will reverse the drop order. However, if you
panic after constructing the <code>Vec</code> by manually calling <code>push</code> a couple of
times, the drop order will be queue-like (from Rust&rsquo;s perspective
you are dropping a fully constructed <code>Vec</code>).</p>

<h2 id="closure-captures">Closure captures</h2>

<p>An intriguing construct to <em>close</em> this post is the case of <em>closure</em> captures.
We know that, under the hood, closures are actually structs that implement
the <code>Fn</code>, <code>FnMut</code> or <code>FnOnce</code> traits. This means that the drop order depends
on the order in which captures are declared in the generated struct.</p>

<p>Let&rsquo;s start with a simple code example. Note that the order in which the
variables are declared is different than the order in which they are used
in the closure.</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> main() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> z </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;z&quot;</span><span style="color: #f8f8f2">);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> x </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;x&quot;</span><span style="color: #f8f8f2">);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> y </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> PrintDrop(</span><span style="color: #e6db74">&quot;y&quot;</span><span style="color: #f8f8f2">);</span>

<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> closure </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> move </span><span style="color: #f92672">||</span><span style="color: #f8f8f2"> {</span>
<span style="color: #f8f8f2">        x; y; z;</span>
<span style="color: #f8f8f2">    };</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>And the output is&hellip;</p>

<pre><code>Dropping x
Dropping y
Dropping z
</code></pre>

<p>Based on the output it seems that the drop order is the same as the order in
which the variables appear in the closure. However, we should test what
happens in the scenario below:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> closure </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> move </span><span style="color: #f92672">||</span><span style="color: #f8f8f2"> {</span>
<span style="color: #f8f8f2">    {</span>
<span style="color: #f8f8f2">        </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> z_ref </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">z;</span>
<span style="color: #f8f8f2">    }</span>
<span style="color: #f8f8f2">    x; y; z;</span>
<span style="color: #f8f8f2">};</span>
</pre></div>

<p>Again, the output is:</p>

<pre><code>Dropping x
Dropping y
Dropping z
</code></pre>

<p>As you can see, even though <code>z</code> appears first as a reference, it is still the
last one to be dropped. Therefore we should reformulate our hypothesis and say
that the order in which captures are dropped is the same as the order in which
they are moved. This way we ignore any references that may appear before.</p>

<p>Of course, we could perform more experiments to see if there are any edge
cases to be aware about, but in the end the best approach would be to look at
the source code of the compiler. This will certainly be necessary when drop
order is stabilized.</p>