+++
title = "Wrapping callbacks without userdata"
date = "2016-07-29T08:08:00-07:00"

[extra]
author = "Platymuus"
link = "https://www.platymuus.com/posts/2016/callbacks-without-userdata/"
+++
<p>It’s easy to find articles describing how to wrap Rust closures to pass to C libraries that will carry a <code class="highlighter-rouge">void*</code> to your callback, but what can you do when no userdata value is provided? This post is based on work I’ve done for the <a href="https://github.com/jcmoyer/rust-lua53"><code class="highlighter-rouge">lua</code> crate</a>.</p>

<h2 id="the-situation">The Situation</h2>

<p>Let’s imagine that the C library provides an API roughly analogous to the following:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">type</span> <span class="n">ffi_Object</span> <span class="o">=</span> <span class="o">...</span><span class="p">;</span> <span class="c">// opaque type representing foreign Object</span>
<span class="k">type</span> <span class="n">ffi_Callback</span> <span class="o">=</span> <span class="k">unsafe</span> <span class="k">extern</span> <span class="k">fn</span><span class="p">(</span><span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="p">;</span>
<span class="k">extern</span> <span class="p">{</span>
	<span class="k">fn</span> <span class="nf">ffi_push_callback</span><span class="p">(</span><span class="n">_</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">,</span> <span class="n">_</span><span class="p">:</span> <span class="n">ffi_Callback</span><span class="p">);</span>
<span class="p">}</span>
</code></pre></div></div>

<p>We currently have a safe wrapper type named <code class="highlighter-rouge">Object</code> and we want to enable consumers of our library to push callbacks onto it without writing any <code class="highlighter-rouge">unsafe</code> code, but how can we do that without a userdata pointer? While the lack of a userdata pointer prevents us from carrying any state with the callback (ruling out closures), it’s still possible to wrap ordinary functions.</p>

<h2 id="the-solution">The Solution</h2>

<p>First, let’s think about the signature we want the users of our wrapper to conform to. The FFI demands <code class="highlighter-rouge">unsafe extern fn(*mut ffi_Object) -&gt; u32</code>, but we want to allow such callbacks to be totally safe. This means no <code class="highlighter-rouge">unsafe</code> or <code class="highlighter-rouge">extern</code>, and we should replace <code class="highlighter-rouge">*mut ffi_Object</code> with a safe analogue. Since the callback does not take ownership, this is <code class="highlighter-rouge">&amp;mut Object</code>.</p>

<p>Now we’re going to need to define a function that converts from our safe signature to the unsafe signature. At first, you might try to write:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">fn</span> <span class="nf">wrap_callback</span><span class="p">(</span><span class="n">callback</span><span class="p">:</span> <span class="k">fn</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">ffi_Callback</span> <span class="p">{</span> <span class="o">...</span> <span class="p">}</span>
</code></pre></div></div>

<p>but this leads to a dead end. A <code class="highlighter-rouge">fn(&amp;mut Object) -&gt; u32</code> is a function pointer, a runtime value, state! Exactly what we can’t pack into an <code class="highlighter-rouge">ffi_Callback</code>. We’re going to need to build, at compile time, a different <code class="highlighter-rouge">ffi_Callback</code> for each function we’re ever passed. Luckily, Rust has the useful property that each function gets its own unique type, so we can use a type parameter - our <code class="highlighter-rouge">wrap_callback</code> function is going to be generic. These unique types can’t be written directly, so let’s add a dummy parameter <code class="highlighter-rouge">_: F</code> to allow inference to fill them in for us.</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">fn</span> <span class="n">wrap_callback</span><span class="o">&lt;</span><span class="n">F</span><span class="p">:</span> <span class="nf">Fn</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="o">&gt;</span><span class="p">(</span><span class="n">_</span><span class="p">:</span> <span class="n">F</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">ffi_Callback</span> <span class="p">{</span>
</code></pre></div></div>

<p>Now that we know the function’s signature, what about its contents? First, let’s not forget to reintroduce a restriction we just accidentally loosened: this signature accepts closures with state, and it’s not possible for us to support that. Let’s be sure we’re being passed ordinary functions (or equally-valid stateless closures):</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code>	<span class="k">assert</span><span class="o">!</span><span class="p">(</span><span class="nn">mem</span><span class="p">::</span><span class="nn">size_of</span><span class="p">::</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span><span class="p">()</span> <span class="o">==</span> <span class="mi">0</span><span class="p">);</span>
</code></pre></div></div>

<p>It’s a little unfortunate that we have to runtime assert this and can’t check it at compile time, but it’s the best we can do in current Rust. Make sure this requirement is displayed in your library’s documentation.</p>

<p>Alright, now what? Well, we need to define a function with the signature demanded by <code class="highlighter-rouge">ffi_Callback</code>. Unlike <code class="highlighter-rouge">wrap_callback</code>, which accepts a function as a type parameter and returns an <code class="highlighter-rouge">ffi_Callback</code>, this function will accept the same and <em>be</em> an <code class="highlighter-rouge">ffi_Callback</code>.</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code>	<span class="k">unsafe</span> <span class="k">extern</span> <span class="k">fn</span> <span class="n">wrapped</span><span class="o">&lt;</span><span class="n">F</span><span class="p">:</span> <span class="nf">Fn</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="o">&gt;</span><span class="p">(</span><span class="n">ptr</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span> <span class="p">{</span>
</code></pre></div></div>

<p>First, let’s convert our <code class="highlighter-rouge">*mut ffi_Object</code> into a safe <code class="highlighter-rouge">Object</code> we can use:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code>		<span class="k">let</span> <span class="k">mut</span> <span class="n">object</span> <span class="o">=</span> <span class="nn">Object</span><span class="p">::</span><span class="nf">from_ptr</span><span class="p">(</span><span class="n">ptr</span><span class="p">);</span>
</code></pre></div></div>

<p>Now we need to obtain a value of type <code class="highlighter-rouge">F</code> in order to call it. We just asserted that <code class="highlighter-rouge">F</code> is zero-sized, so there’s exactly one possible value, but the compiler doesn’t know that. We’ll have to unsafely construct that value somehow - probably the simplest way is <a href="https://doc.rust-lang.org/std/mem/fn.zeroed.html"><code class="highlighter-rouge">mem::zeroed</code></a>. Then, we’re going to call that function with a reference to the <code class="highlighter-rouge">Object</code>, and save the result:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code>		<span class="c">//           | generate value ||  call it  |</span>
		<span class="k">let</span> <span class="n">result</span> <span class="o">=</span> <span class="nn">mem</span><span class="p">::</span><span class="nn">zeroed</span><span class="p">::</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span><span class="p">()(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">object</span><span class="p">);</span>
</code></pre></div></div>

<p>Lastly, since our <code class="highlighter-rouge">Object</code> is an owned type, we need to avoid dropping it, then return the result we just generated. A future post will cover how to handle this in a less risky way.</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code>		<span class="nn">mem</span><span class="p">::</span><span class="nf">forget</span><span class="p">(</span><span class="n">object</span><span class="p">);</span>
		<span class="n">result</span>
	<span class="p">}</span>
</code></pre></div></div>

<p>Now we’re back in <code class="highlighter-rouge">wrap_callback</code>, ready to return the function satisfying <code class="highlighter-rouge">ffi_Closure</code> we just built:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code>	<span class="nn">wrapped</span><span class="p">::</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span>
<span class="p">}</span>
</code></pre></div></div>

<p>And that’s it! Using it is simple, and as we hoped, contains no use of <code class="highlighter-rouge">unsafe</code>:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">fn</span> <span class="nf">our_callback</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span> <span class="p">{</span> <span class="o">...</span> <span class="p">}</span>
<span class="k">fn</span> <span class="nf">push_our_callback</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="p">{</span>
	<span class="k">let</span> <span class="n">wrapped</span><span class="p">:</span> <span class="n">ffi_Callback</span> <span class="o">=</span> <span class="nf">wrap_callback</span><span class="p">(</span><span class="n">our_callback</span><span class="p">);</span>
	<span class="n">object</span><span class="nf">.push_callback</span><span class="p">(</span><span class="n">wrapped</span><span class="p">);</span>
<span class="p">}</span>
</code></pre></div></div>

<p>Depending on your needs, you can make <code class="highlighter-rouge">wrap_callback</code> public or use it internally.</p>

<h2 id="full-example">Full Example</h2>

<p>Here’s the code in this article all put together, including a dummy version of <code class="highlighter-rouge">Object</code> which I omitted for brevity. If you want to experiment, you can use this code on the <a href="https://play.rust-lang.org/?gist=62b88ad266c9264f4dcf362a98033e18&amp;version=stable">Rust Playground</a>. I’m interested in feedback and can accept it by email or the comments section of wherever you found this post.</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="err">#</span><span class="o">!</span><span class="p">[</span><span class="nf">allow</span><span class="p">(</span><span class="n">non_camel_case_types</span><span class="p">)]</span>
<span class="k">use</span> <span class="nn">std</span><span class="p">::</span><span class="n">mem</span><span class="p">;</span>

<span class="c">// The Situation</span>
<span class="k">struct</span> <span class="n">ffi_Object</span><span class="p">;</span>
<span class="k">type</span> <span class="n">ffi_Callback</span> <span class="o">=</span> <span class="k">unsafe</span> <span class="k">extern</span> <span class="k">fn</span><span class="p">(</span><span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="p">;</span>
<span class="k">extern</span> <span class="k">fn</span> <span class="nf">ffi_push_callback</span><span class="p">(</span><span class="n">_</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">,</span> <span class="n">_</span><span class="p">:</span> <span class="n">ffi_Callback</span><span class="p">)</span> <span class="p">{}</span>

<span class="c">// Dummy wrapper</span>
<span class="k">struct</span> <span class="n">Object</span> <span class="p">{</span>
	<span class="n">ptr</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span>
<span class="p">}</span>

<span class="k">impl</span> <span class="n">Object</span> <span class="p">{</span>
	<span class="k">fn</span> <span class="nf">new</span><span class="p">()</span> <span class="k">-&gt;</span> <span class="n">Object</span> <span class="p">{</span>
		<span class="c">// initialize ffi_Object here</span>
		<span class="n">Object</span> <span class="p">{</span> <span class="n">ptr</span><span class="p">:</span> <span class="nn">std</span><span class="p">::</span><span class="nn">ptr</span><span class="p">::</span><span class="nf">null_mut</span><span class="p">()</span> <span class="p">}</span>
	<span class="p">}</span>
	<span class="k">fn</span> <span class="nf">from_ptr</span><span class="p">(</span><span class="n">ptr</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">Object</span> <span class="p">{</span>
		<span class="n">Object</span> <span class="p">{</span> <span class="n">ptr</span><span class="p">:</span> <span class="n">ptr</span> <span class="p">}</span>
	<span class="p">}</span>
	<span class="k">fn</span> <span class="nf">push_callback</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="k">self</span><span class="p">,</span> <span class="n">cb</span><span class="p">:</span> <span class="n">ffi_Callback</span><span class="p">)</span> <span class="p">{</span>
		<span class="nf">ffi_push_callback</span><span class="p">(</span><span class="k">self</span><span class="py">.ptr</span><span class="p">,</span> <span class="n">cb</span><span class="p">);</span>
	<span class="p">}</span>
<span class="p">}</span>

<span class="k">impl</span> <span class="n">Drop</span> <span class="k">for</span> <span class="n">Object</span> <span class="p">{</span>
	<span class="k">fn</span> <span class="k">drop</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="k">self</span><span class="p">)</span> <span class="p">{</span>
		<span class="c">// destroy ffi_Object here</span>
	<span class="p">}</span>
<span class="p">}</span>

<span class="c">// The Solution</span>
<span class="k">fn</span> <span class="n">wrap_callback</span><span class="o">&lt;</span><span class="n">F</span><span class="p">:</span> <span class="nf">Fn</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="o">&gt;</span><span class="p">(</span><span class="n">_</span><span class="p">:</span> <span class="n">F</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">ffi_Callback</span> <span class="p">{</span>
	<span class="k">assert</span><span class="o">!</span><span class="p">(</span><span class="nn">mem</span><span class="p">::</span><span class="nn">size_of</span><span class="p">::</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span><span class="p">()</span> <span class="o">==</span> <span class="mi">0</span><span class="p">);</span>

	<span class="k">unsafe</span> <span class="k">extern</span> <span class="k">fn</span> <span class="n">wrapped</span><span class="o">&lt;</span><span class="n">F</span><span class="p">:</span> <span class="nf">Fn</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span><span class="o">&gt;</span><span class="p">(</span><span class="n">ptr</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span> <span class="p">{</span>
		<span class="k">let</span> <span class="k">mut</span> <span class="n">object</span> <span class="o">=</span> <span class="nn">Object</span><span class="p">::</span><span class="nf">from_ptr</span><span class="p">(</span><span class="n">ptr</span><span class="p">);</span>
		<span class="k">let</span> <span class="n">result</span> <span class="o">=</span> <span class="nn">mem</span><span class="p">::</span><span class="nn">transmute</span><span class="p">::</span><span class="o">&lt;</span><span class="n">_</span><span class="p">,</span> <span class="o">&amp;</span><span class="n">F</span><span class="o">&gt;</span><span class="p">(</span><span class="o">&amp;</span><span class="p">())(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">object</span><span class="p">);</span>
		<span class="nn">mem</span><span class="p">::</span><span class="nf">forget</span><span class="p">(</span><span class="n">object</span><span class="p">);</span>
		<span class="n">result</span>
	<span class="p">}</span>

	<span class="nn">wrapped</span><span class="p">::</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span>
<span class="p">}</span>

<span class="c">// Usage</span>
<span class="k">fn</span> <span class="nf">our_callback</span><span class="p">(</span><span class="n">_</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span> <span class="p">{</span>
	<span class="mi">0</span>
<span class="p">}</span>
<span class="k">fn</span> <span class="nf">main</span><span class="p">()</span> <span class="p">{</span>
	<span class="k">let</span> <span class="k">mut</span> <span class="n">object</span> <span class="o">=</span> <span class="nn">Object</span><span class="p">::</span><span class="nf">new</span><span class="p">();</span>
	<span class="k">let</span> <span class="n">wrapped</span><span class="p">:</span> <span class="n">ffi_Callback</span> <span class="o">=</span> <span class="nf">wrap_callback</span><span class="p">(</span><span class="n">our_callback</span><span class="p">);</span>
	<span class="n">object</span><span class="nf">.push_callback</span><span class="p">(</span><span class="n">wrapped</span><span class="p">);</span>
<span class="p">}</span>
</code></pre></div></div>

<h2 id="addendum-safer-less-usable">Addendum: Safer, Less Usable</h2>

<p>If you’re not satisfied with the runtime assert, it can be eliminated if you’re willing to sacrifice usability:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="c">// Definition</span>
<span class="k">trait</span> <span class="n">Callback</span> <span class="p">{</span>
	<span class="k">fn</span> <span class="nf">call</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">i32</span><span class="p">;</span>
<span class="p">}</span>

<span class="k">fn</span> <span class="n">wrap_callback</span><span class="o">&lt;</span><span class="n">F</span><span class="p">:</span> <span class="n">Callback</span><span class="o">&gt;</span><span class="p">()</span> <span class="k">-&gt;</span> <span class="n">ffi_Callback</span> <span class="p">{</span>
	<span class="k">unsafe</span> <span class="k">extern</span> <span class="k">fn</span> <span class="n">wrapped</span><span class="o">&lt;</span><span class="n">F</span><span class="p">:</span> <span class="n">Callback</span><span class="o">&gt;</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">ffi_Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">i32</span> <span class="p">{</span>
		<span class="k">let</span> <span class="k">mut</span> <span class="n">object</span> <span class="o">=</span> <span class="nn">Object</span><span class="p">::</span><span class="nf">from_ptr</span><span class="p">(</span><span class="n">ptr</span><span class="p">);</span>
		<span class="k">let</span> <span class="n">result</span> <span class="o">=</span> <span class="nn">F</span><span class="p">::</span><span class="nf">call</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="n">object</span><span class="p">);</span>
		<span class="nn">mem</span><span class="p">::</span><span class="nf">forget</span><span class="p">(</span><span class="n">object</span><span class="p">);</span>
		<span class="n">result</span>
	<span class="p">}</span>
	<span class="nn">wrapped</span><span class="p">::</span><span class="o">&lt;</span><span class="n">F</span><span class="o">&gt;</span>
<span class="p">}</span>

<span class="c">// Usage</span>
<span class="k">struct</span> <span class="n">OurCallback</span><span class="p">;</span>
<span class="k">impl</span> <span class="n">Callback</span> <span class="k">for</span> <span class="n">OurCallback</span> <span class="p">{</span>
	<span class="k">fn</span> <span class="nf">call</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u32</span> <span class="p">{</span>
		<span class="o">...</span>
	<span class="p">}</span>
<span class="p">}</span>

<span class="k">fn</span> <span class="nf">push_our_callback</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="p">{</span>
	<span class="n">object</span><span class="nf">.push_callback</span><span class="p">(</span><span class="nn">wrap_callback</span><span class="p">::</span><span class="o">&lt;</span><span class="n">OurCallback</span><span class="o">&gt;</span><span class="p">());</span>
<span class="p">}</span>
</code></pre></div></div>

<p><em>Thanks to Lalaland on <code class="highlighter-rouge">#rust</code> for this suggestion.</em></p>

<h2 id="addendum-the-past">Addendum: The Past</h2>

<p>There was once a time when those unique per-function types we made use of were not zero-sized! This was just an implementation flaw and has since been resolved, but it was possible to work around it using closures that captured no state, and <em>were</em> zero-sized:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">fn</span> <span class="nf">push_our_callback</span><span class="p">(</span><span class="n">object</span><span class="p">:</span> <span class="o">&amp;</span><span class="k">mut</span> <span class="n">Object</span><span class="p">)</span> <span class="p">{</span>
	<span class="n">object</span><span class="nf">.push_callback</span><span class="p">(</span><span class="nf">wrap_callback</span><span class="p">(|</span><span class="n">s</span><span class="p">|</span> <span class="nf">our_callback</span><span class="p">(</span><span class="n">s</span><span class="p">)));</span>
<span class="p">}</span>
</code></pre></div></div>