+++
title = "Sketch of overloaded short-circuiting operators"
date = "2016-09-25T11:30:00-07:00"

[extra]
author = "Platymuus"
link = "https://www.platymuus.com/posts/2016/overloaded-short-circuit/"
+++
<p>Rust’s <a href="https://doc.rust-lang.org/std/ops/index.html"><code class="highlighter-rouge">std::ops</code></a> module provides a variety of traits for overloading Rust’s operators, but conspicuously skips the <code class="highlighter-rouge">&amp;&amp;</code> and <code class="highlighter-rouge">||</code> operators. While I don’t believe these operators are that important to overload, I wanted to attempt to address one of the barriers to that functionality, namely short-circuiting, and provide a starting point for someone who actually cares to write an RFC.</p>

<p>As a refresher, short-circuiting is the behavior that these operators sometimes don’t evaluate their right-hand sides at all, depending on the value of their left-hand sides (<code class="highlighter-rouge">true || x</code> is always <code class="highlighter-rouge">true</code>, <code class="highlighter-rouge">false &amp;&amp; x</code> is always <code class="highlighter-rouge">false</code>). This prevents representing these operators with function calls, because a functions’s arguments must be evaluated before it can be called. Even a closure isn’t enough - because an expression like <code class="highlighter-rouge">lhs &amp;&amp; return</code> is valid, but cannot be transformed to <code class="highlighter-rouge">and(lhs, || return)</code>, which has different semantics.</p>

<p>A more involved approach is needed. Fundamentally, these short-circuiting operators evaluate the left-hand side and depending on that value either short-circuit to a result or evaluate the right-hand side and combine it with the left-hand side in an operation. To represent these two steps, our trait will need two methods, one to represent the short-circuit check and one for the actual operation. For the rest of this post, I will be speaking about the <code class="highlighter-rouge">&amp;&amp;</code> operator; the <code class="highlighter-rouge">||</code> operator works pretty much the same.</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">trait</span> <span class="n">And</span><span class="o">&lt;</span><span class="n">Rhs</span><span class="o">=</span><span class="n">Self</span><span class="o">&gt;</span> <span class="p">{</span>
	<span class="k">type</span> <span class="n">Output</span><span class="p">;</span>

	<span class="k">fn</span> <span class="nf">and_short</span><span class="p">(</span><span class="o">&amp;</span><span class="k">self</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="nn">Self</span><span class="p">::</span><span class="n">Output</span><span class="o">&gt;</span><span class="p">;</span>
	<span class="c">// alternate:</span>
	<span class="k">fn</span> <span class="nf">and_short</span><span class="p">(</span><span class="k">self</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">Result</span><span class="o">&lt;</span><span class="nn">Self</span><span class="p">::</span><span class="n">Output</span><span class="p">,</span> <span class="n">Self</span><span class="o">&gt;</span><span class="p">;</span>

	<span class="k">fn</span> <span class="nf">and</span><span class="p">(</span><span class="k">self</span><span class="p">,</span> <span class="n">rhs</span><span class="p">:</span> <span class="n">Rhs</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nn">Self</span><span class="p">::</span><span class="n">Output</span><span class="p">;</span>
<span class="p">}</span>
</code></pre></div></div>

<p>In this sketch, <code class="highlighter-rouge">and_short</code> takes <code class="highlighter-rouge">&amp;self</code> because if it returns None, we need to pass that same <code class="highlighter-rouge">self</code> to <code class="highlighter-rouge">and</code>. If we wanted to allow moving in <code class="highlighter-rouge">and_short</code>, its signature could instead be changed to the listed alternate, where <code class="highlighter-rouge">Ok(v)</code> is a short-circuited result and <code class="highlighter-rouge">Err(s)</code> returns the value to be used as the self value of <code class="highlighter-rouge">and</code>.</p>

<p>The desugaring of <code class="highlighter-rouge">lhs &amp;&amp; rhs</code> then becomes fairly straightforward. To eliminate ambiguity, I’ve represented it as a macro:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="nd">macro_rules!</span> <span class="n">and</span> <span class="p">{</span>
	<span class="p">(</span><span class="nv">$lhs:expr</span><span class="p">,</span> <span class="nv">$rhs:expr</span><span class="p">)</span> <span class="k">=&gt;</span> <span class="p">{{</span>
		<span class="k">let</span> <span class="n">lhs</span> <span class="o">=</span> <span class="nv">$lhs</span><span class="p">;</span>
		<span class="k">match</span> <span class="nn">And</span><span class="p">::</span><span class="nf">and_short</span><span class="p">(</span><span class="o">&amp;</span><span class="n">lhs</span><span class="p">)</span> <span class="p">{</span>
			<span class="nf">Some</span><span class="p">(</span><span class="n">value</span><span class="p">)</span> <span class="k">=&gt;</span> <span class="n">value</span><span class="p">,</span>
			<span class="nb">None</span> <span class="k">=&gt;</span> <span class="nn">And</span><span class="p">::</span><span class="nf">and</span><span class="p">(</span><span class="n">lhs</span><span class="p">,</span> <span class="nv">$rhs</span><span class="p">),</span>
		<span class="p">}</span>
	<span class="p">}}</span>
<span class="p">}</span>
</code></pre></div></div>

<p>The left-hand operand is always evaluated immediately, then <code class="highlighter-rouge">and_short</code> is called. If it returns <code class="highlighter-rouge">Some</code>, the right-hand operand is not evaluted; if it returns <code class="highlighter-rouge">None</code>, the right-hand side is evaluated and <code class="highlighter-rouge">and</code> is called. Here’s how an implementation for <code class="highlighter-rouge">bool</code> might look, if we needed to implement it in Rust:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">impl</span> <span class="n">And</span> <span class="k">for</span> <span class="nb">bool</span> <span class="p">{</span>
	<span class="k">type</span> <span class="n">Output</span> <span class="o">=</span> <span class="nb">bool</span><span class="p">;</span>
	<span class="k">fn</span> <span class="nf">and_short</span><span class="p">(</span><span class="o">&amp;</span><span class="k">self</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="nb">bool</span><span class="o">&gt;</span> <span class="p">{</span>
		<span class="k">match</span> <span class="o">*</span><span class="k">self</span> <span class="p">{</span>
			<span class="k">false</span> <span class="k">=&gt;</span> <span class="nf">Some</span><span class="p">(</span><span class="k">false</span><span class="p">),</span>
			<span class="k">true</span> <span class="k">=&gt;</span> <span class="nb">None</span><span class="p">,</span>
		<span class="p">}</span>
	<span class="p">}</span>
	<span class="k">fn</span> <span class="nf">and</span><span class="p">(</span><span class="k">self</span><span class="p">,</span> <span class="n">rhs</span><span class="p">:</span> <span class="nb">bool</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">bool</span> <span class="p">{</span>
		<span class="k">match</span> <span class="p">(</span><span class="k">self</span><span class="p">,</span> <span class="n">rhs</span><span class="p">)</span> <span class="p">{</span>
			<span class="p">(</span><span class="k">true</span><span class="p">,</span> <span class="k">true</span><span class="p">)</span> <span class="k">=&gt;</span> <span class="k">true</span><span class="p">,</span>
			<span class="n">_</span> <span class="k">=&gt;</span> <span class="k">false</span><span class="p">,</span>
		<span class="p">}</span>
	<span class="p">}</span>
<span class="p">}</span>
</code></pre></div></div>

<p>And here’s what an implementation for a hypothetical <a href="https://en.wikipedia.org/wiki/Three-valued_logic#Logics">ternary logic value</a> might look like:</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">enum</span> <span class="n">Tri</span> <span class="p">{</span>
	<span class="n">False</span> <span class="o">=</span> <span class="err">-</span><span class="mi">1</span><span class="p">,</span>
	<span class="nb">None</span> <span class="o">=</span> <span class="mi">0</span><span class="p">,</span>
	<span class="n">True</span> <span class="o">=</span> <span class="mi">1</span><span class="p">,</span>
<span class="p">}</span>

<span class="k">impl</span> <span class="n">And</span> <span class="k">for</span> <span class="n">Tri</span> <span class="p">{</span>
	<span class="k">type</span> <span class="n">Output</span> <span class="o">=</span> <span class="n">Tri</span><span class="p">;</span>
	<span class="k">fn</span> <span class="nf">and_short</span><span class="p">(</span><span class="o">&amp;</span><span class="k">self</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">Tri</span><span class="o">&gt;</span> <span class="p">{</span>
		<span class="k">match</span> <span class="o">*</span><span class="k">self</span> <span class="p">{</span>
			<span class="nn">Tri</span><span class="p">::</span><span class="n">False</span> <span class="k">=&gt;</span> <span class="nf">Some</span><span class="p">(</span><span class="nn">Tri</span><span class="p">::</span><span class="n">False</span><span class="p">),</span>
			<span class="n">_</span> <span class="k">=&gt;</span> <span class="nb">None</span><span class="p">,</span>
		<span class="p">}</span>
	<span class="p">}</span>
	<span class="k">fn</span> <span class="nf">and</span><span class="p">(</span><span class="k">self</span><span class="p">,</span> <span class="n">rhs</span><span class="p">:</span> <span class="n">Tri</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">Tri</span> <span class="p">{</span>
		<span class="k">match</span> <span class="p">(</span><span class="k">self</span><span class="p">,</span> <span class="n">rhs</span><span class="p">)</span> <span class="p">{</span>
			<span class="p">(</span><span class="nn">Tri</span><span class="p">::</span><span class="n">False</span><span class="p">,</span> <span class="n">_</span><span class="p">)</span> <span class="p">|</span>
			<span class="p">(</span><span class="n">_</span><span class="p">,</span> <span class="nn">Tri</span><span class="p">::</span><span class="n">False</span><span class="p">)</span> <span class="k">=&gt;</span> <span class="nn">Tri</span><span class="p">::</span><span class="n">False</span><span class="p">,</span>
			<span class="p">(</span><span class="nn">Tri</span><span class="p">::</span><span class="nb">None</span><span class="p">,</span> <span class="n">_</span><span class="p">)</span> <span class="p">|</span>
			<span class="p">(</span><span class="n">_</span><span class="p">,</span> <span class="nn">Tri</span><span class="p">::</span><span class="nb">None</span><span class="p">)</span> <span class="k">=&gt;</span> <span class="nn">Tri</span><span class="p">::</span><span class="nb">None</span><span class="p">,</span>
			<span class="p">(</span><span class="nn">Tri</span><span class="p">::</span><span class="n">True</span><span class="p">,</span> <span class="nn">Tri</span><span class="p">::</span><span class="n">True</span><span class="p">)</span> <span class="k">=&gt;</span> <span class="nn">Tri</span><span class="p">::</span><span class="n">True</span><span class="p">,</span>
		<span class="p">}</span>
	<span class="p">}</span>
<span class="p">}</span>
</code></pre></div></div>

<p>Note that it’s important for consistency of behavior that for values of <code class="highlighter-rouge">self</code> where <code class="highlighter-rouge">and_short</code> returns <code class="highlighter-rouge">Some(v)</code>, <code class="highlighter-rouge">and</code> returns <code class="highlighter-rouge">v</code> for any value of <code class="highlighter-rouge">rhs</code>.</p>

<p>A full example, including an <code class="highlighter-rouge">Or</code> trait is available as <a href="https://gist.github.com/SpaceManiac/35565f1516ed1adc03ade2c9461398a1">a gist</a> and can be run on <a href="https://play.rust-lang.org/?gist=35565f1516ed1adc03ade2c9461398a1">the playground</a>. Thanks to those on the <code class="highlighter-rouge">#rust</code> IRC who inspired me to want to write this post and poked holes in my earlier ideas.</p>