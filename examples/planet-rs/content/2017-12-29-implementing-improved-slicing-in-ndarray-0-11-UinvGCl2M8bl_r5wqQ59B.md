+++
title = "Implementing Improved Slicing in `ndarray` 0.11"
date = "2017-12-29T00:00:00+00:00"

[extra]
author = "Rust — Jim Turner"
link = "https://jim.turner.link/pages/impl-slicing-subviews-ndarray/"
+++
<p><a href="https://crates.io/crates/ndarray"><code>ndarray</code></a> 0.11 has a convenient new feature: combined slicing and subviews
as a single operation. <a href="https://crates.io/crates/ndarray"><code>ndarray</code></a> is a <a href="https://www.rust-lang.org/">Rust</a> crate that provides an
<em>n</em>-dimensional array type and associated functionality for general elements
and for numerics.</p>

<p>This article discusses the implementation of the new feature. It&rsquo;s a story of
type hacking: zero-sized types, associated types, traits for compile-time type
manipulation, and pointer casting.</p>

<h2 id="introduction">Introduction</h2>

<p>The primary type in <code>ndarray</code> is the <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html"><code>ArrayBase&lt;S, D&gt;</code></a> container type,
which is represented internally as:</p>

<ul>
<li>the data representation (e.g. a contiguous memory block for owned arrays),</li>
<li>a pointer to the first element,</li>
<li>a sequence of <em>n</em> axis lengths, and</li>
<li>a sequence of <em>n</em> axis strides.</li>
</ul>

<p>The two generic type parameters in <code>ArrayBase&lt;S, D&gt;</code> are:</p>

<ul>
<li><code>S</code>: the data ownership (owned arrays own the block of data; array views only
have a pointer to it)</li>
<li><code>D</code>: the dimensionality of the array (e.g. <code>Ix1</code> for 1-D arrays, <code>Ix2</code> for
2-D arrays, etc.)</li>
</ul>

<p><code>ArrayBase</code> is used for both owned arrays and views of arrays. Note that the
number of dimensions of the array is typically fixed at compile time, although
there is a dynamic dimension type <code>IxDyn</code> if needed.</p>

<h3 id="slicing">Slicing</h3>

<p>Slicing methods such as <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice"><code>.slice()</code></a> create a view a subset of the data in an
array. With a step other than 1, it’s also possible to do things such as view
every third index along an axis or reverse the order of an axis.</p>

<p>For example, let’s say you have a 3-dimensional array of shape 2 × 2 × 3 (i.e.
2 submatrices of 2 rows and 3 columns):</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="kd">let</span><span class="w"> </span><span class="n">arr</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">array</span><span class="o">!</span><span class="p">[[[</span><span class="w"> </span><span class="mi">1</span><span class="p">,</span><span class="w">  </span><span class="mi">2</span><span class="p">,</span><span class="w">  </span><span class="mi">3</span><span class="p">],</span><span class="w"></span>
<span class="w">                  </span><span class="p">[</span><span class="w"> </span><span class="mi">4</span><span class="p">,</span><span class="w">  </span><span class="mi">5</span><span class="p">,</span><span class="w">  </span><span class="mi">6</span><span class="p">]],</span><span class="w"></span>
<span class="w">                 </span><span class="p">[[</span><span class="w"> </span><span class="mi">7</span><span class="p">,</span><span class="w">  </span><span class="mi">8</span><span class="p">,</span><span class="w">  </span><span class="mi">9</span><span class="p">],</span><span class="w"></span>
<span class="w">                  </span><span class="p">[</span><span class="mi">10</span><span class="p">,</span><span class="w"> </span><span class="mi">11</span><span class="p">,</span><span class="w"> </span><span class="mi">12</span><span class="p">]]];</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="n">arr</span><span class="p">.</span><span class="n">shape</span><span class="p">(),</span><span class="w"> </span><span class="o">&amp;</span><span class="p">[</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">]);</span><span class="w"></span>
</code></pre></div>

<p>You could create a slice with</p>

<ul>
<li>Both of the submatrices, but in reverse order: <code>0..2;-1</code></li>
<li>The first row in each submatrix: <code>..1</code></li>
<li>The even-index columns in each submatrix: <code>..;2</code></li>
</ul>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="kd">let</span><span class="w"> </span><span class="n">slice</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="mi">0</span><span class="o">..</span><span class="mi">2</span><span class="p">;</span><span class="o">-</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">0</span><span class="o">..</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">]);</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="w"></span>
<span class="w">    </span><span class="n">slice</span><span class="p">,</span><span class="w"></span>
<span class="w">    </span><span class="n">array</span><span class="o">!</span><span class="p">[[[</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="mi">9</span><span class="p">]],</span><span class="w"></span>
<span class="w">           </span><span class="p">[[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">]]]</span><span class="w"></span>
<span class="p">);</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="n">slice</span><span class="p">.</span><span class="n">shape</span><span class="p">(),</span><span class="w"> </span><span class="o">&amp;</span><span class="p">[</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">]);</span><span class="w"></span>
</code></pre></div>

<p>Here, <code>slice</code> is a view of a subset of <code>arr</code> (without copying).</p>

<h3 id="subviews">Subviews</h3>

<p>Subview methods such as <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.subview"><code>.subview()</code></a> create a view restricted to a single
index along an axis, and then remove that axis from the view. This produces a
view with one dimension less than the input array.</p>

<p>For example, let’s select the first (index <code>0</code>) row in each submatrix (axis
<code>1</code>) of <code>arr</code>. The new shape is <code>[2, 3]</code> because axis <code>1</code> was removed from the
view.</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="kd">let</span><span class="w"> </span><span class="n">subview</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">1</span><span class="p">),</span><span class="w"> </span><span class="mi">0</span><span class="p">);</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="w"></span>
<span class="w">    </span><span class="n">subview</span><span class="p">,</span><span class="w"></span>
<span class="w">    </span><span class="n">array</span><span class="o">!</span><span class="p">[[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">],</span><span class="w"></span>
<span class="w">           </span><span class="p">[</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="mi">8</span><span class="p">,</span><span class="w"> </span><span class="mi">9</span><span class="p">]]</span><span class="w"></span>
<span class="p">);</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="n">subview</span><span class="p">.</span><span class="n">shape</span><span class="p">(),</span><span class="w"> </span><span class="o">&amp;</span><span class="p">[</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">]);</span><span class="w"></span>
</code></pre></div>

<h3 id="combining-slicing-and-subviews">Combining slicing and subviews</h3>

<p><code>ndarray</code> 0.11 makes it possible to combine slicing and subviews in a single
operation. To get the same result in earlier versions of <code>ndarray</code>, you’d have
to chain together multiple <code>.slice()</code> and <code>.into_subview()</code> calls while being
careful about their order. For example,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="kd">let</span><span class="w"> </span><span class="n">arr</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Array4</span>::<span class="o">&lt;</span><span class="kt">f64</span><span class="o">&gt;</span>::<span class="n">zeros</span><span class="p">((</span><span class="mi">12</span><span class="p">,</span><span class="w"> </span><span class="mi">10</span><span class="p">,</span><span class="w"> </span><span class="mi">6</span><span class="p">,</span><span class="w"> </span><span class="mi">9</span><span class="p">));</span><span class="w"></span>

<span class="c1">// old</span>
<span class="kd">let</span><span class="w"> </span><span class="n">slice</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="mi">3</span><span class="o">..</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">]).</span><span class="n">into_subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">3</span><span class="p">),</span><span class="w"> </span><span class="mi">5</span><span class="p">).</span><span class="n">into_subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">1</span><span class="p">),</span><span class="w"> </span><span class="mi">4</span><span class="p">);</span><span class="w"></span>

<span class="c1">// new</span>
<span class="kd">let</span><span class="w"> </span><span class="n">slice</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="mi">3</span><span class="o">..</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">5</span><span class="p">]);</span><span class="w"></span>
</code></pre></div>

<p>In this example, <code>3..7</code> and <code>..;2</code> indicate slices of axes 0 and 2, and <code>4</code> and
<code>5</code> indicate subviews of axes 1 and 3. The new version is more concise and is
easier to use correctly than a series of <code>.slice()</code> and <code>.into_subview()</code>
calls.</p>

<h2 id="discussion">Discussion</h2>

<p>Implementing this feature was interesting because so much needed to be handled
at compile time. The goals were:</p>

<ol>
<li>Allow a combination of slicing and subviews in a single operation.</li>
<li>Provide compile-time checking of the number of dimensions in the slicing
argument. For example, it should be a compile-time error to call
<code>Array1::zeros(5).slice(s![.., ..])</code>.</li>
<li>Determine at compile time the number of dimensions of the output array/view
after slicing. (Otherwise, we’d always have to return a dynamic-dimensional
array.)</li>
<li>Provide a representation of slicing information that can be passed around or
stored in a variable.</li>
<li>Minimize the use of generic type parameters.</li>
</ol>

<p>One option was to create a macro that expands to a series of <code>.slice()</code> and
<code>.into_subview()</code> calls, like this:</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="n">slice</span><span class="o">!</span><span class="p">(</span><span class="n">arr</span><span class="p">,</span><span class="w"> </span><span class="p">[</span><span class="mi">3</span><span class="o">..</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">5</span><span class="p">])</span><span class="w"></span>
<span class="o">=&gt;</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="mi">3</span><span class="o">..</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">]).</span><span class="n">into_subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">3</span><span class="p">),</span><span class="w"> </span><span class="mi">5</span><span class="p">).</span><span class="n">into_subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">1</span><span class="p">),</span><span class="w"> </span><span class="mi">4</span><span class="p">)</span><span class="w"></span>
</code></pre></div>

<p>This approach was backwards-compatible, but it didn’t address goal 4 (providing
an object that could represent the slicing information). Additionally,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="n">slice_move</span><span class="o">!</span><span class="p">(</span><span class="mi">5</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="o">&amp;</span><span class="n">slice</span><span class="o">!</span><span class="p">(</span><span class="n">arr</span><span class="p">,</span><span class="w"> </span><span class="p">[</span><span class="mi">3</span><span class="o">..</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">])</span><span class="w"> </span><span class="o">-</span><span class="w"> </span><span class="mi">9</span><span class="p">,</span><span class="w"> </span><span class="p">[</span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">])</span><span class="w"></span>
</code></pre></div>

<p>wasn’t quite as easy to read as</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="p">(</span><span class="mi">5</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="o">&amp;</span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="mi">3</span><span class="o">..</span><span class="mi">7</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">])</span><span class="w"> </span><span class="o">-</span><span class="w"> </span><span class="mi">9</span><span class="p">).</span><span class="n">slice_move</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">])</span><span class="w"></span>
</code></pre></div>

<p>So, I decided to focus on modifying the <code>s![]</code> macro to support subviews.</p>

<h3 id="representing-slicing-information">Representing slicing information</h3>

<p>The first step was to define a representation of the information necessary to
perform a slice.</p>

<p>In earlier versions of <code>ndarray</code>, this was <code>&amp;[Si; n]</code>, where each instance of
<code>Si</code></p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">struct</span> <span class="nc">Si</span><span class="p">(</span><span class="k">pub</span><span class="w"> </span><span class="n">Ixs</span><span class="p">,</span><span class="w"> </span><span class="k">pub</span><span class="w"> </span><span class="nb">Option</span><span class="o">&lt;</span><span class="n">Ixs</span><span class="o">&gt;</span><span class="p">,</span><span class="w"> </span><span class="k">pub</span><span class="w"> </span><span class="n">Ixs</span><span class="p">);</span><span class="w"></span>
</code></pre></div>

<p>represented a slice of one axis. The signature of <code>.slice()</code> was</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">impl</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">S</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="w"> </span><span class="n">ArrayBase</span><span class="o">&lt;</span><span class="n">S</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="w"></span>
<span class="k">where</span><span class="w"></span>
<span class="w">    </span><span class="n">S</span>: <span class="nc">Data</span><span class="o">&lt;</span><span class="n">Elem</span><span class="o">=</span><span class="n">A</span><span class="o">&gt;</span><span class="p">,</span><span class="w"></span>
<span class="w">    </span><span class="n">D</span>: <span class="nc">Dimension</span><span class="p">,</span><span class="w"></span>
<span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="k">pub</span><span class="w"> </span><span class="k">fn</span> <span class="nf">slice</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">indexes</span>: <span class="kp">&amp;</span><span class="nc">D</span>::<span class="n">SliceArg</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="p">;</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>where <code>D::SliceArg</code> was <code>[Si; n]</code> for fixed-dimension arrays and <code>[Si]</code> for
dynamic-dimension arrays. The <code>s![]</code> macro simply provided a compact way to
construct a <code>&amp;[Si; n]</code> instance.</p>

<p>In <code>ndarray</code> 0.11, the replacement for <code>Si</code> had to be able to represent either
a range with step (slice) or an index (subview). Rust made this easy with an
enum:</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nc">SliceOrIndex</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="n">Slice</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">        </span><span class="n">start</span>: <span class="kt">isize</span><span class="p">,</span><span class="w"></span>
<span class="w">        </span><span class="n">end</span>: <span class="nb">Option</span><span class="o">&lt;</span><span class="kt">isize</span><span class="o">&gt;</span><span class="p">,</span><span class="w"></span>
<span class="w">        </span><span class="n">step</span>: <span class="kt">isize</span><span class="p">,</span><span class="w"></span>
<span class="w">    </span><span class="p">},</span><span class="w"></span>
<span class="w">    </span><span class="n">Index</span><span class="p">(</span><span class="kt">isize</span><span class="p">),</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>Now that <code>.slice()</code> could take subviews in addition to slicing axes, the
dimension of the output array/view could be different from the dimension of
<code>self</code>. As a result, the function signature needed to have a generic parameter
<code>Do</code>:</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">fn</span> <span class="nf">slice</span><span class="o">&lt;</span><span class="n">Do</span><span class="o">&gt;</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">info</span>: <span class="kp">&amp;</span><span class="nc">SOMETHING_HERE</span><span class="o">&lt;</span><span class="n">Do</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">Do</span><span class="o">&gt;</span><span class="w"></span>
<span class="k">where</span><span class="w"></span>
<span class="w">    </span><span class="n">Do</span>: <span class="nc">Dimension</span><span class="p">;</span><span class="w"></span>
</code></pre></div>

<p>So, <code>s![]</code> couldn’t just return <code>&amp;[SliceOrIndex; n]</code>; it needed to include
<code>Do</code>. The slicing information had two parts: the output dimension and the
actual sequence of <code>SliceOrIndex</code>. I combined these in a struct</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">struct</span> <span class="nc">SliceInfo</span><span class="o">&lt;</span><span class="n">T</span>: <span class="o">?</span><span class="nb">Sized</span><span class="p">,</span><span class="w"> </span><span class="n">D</span>: <span class="nc">Dimension</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="n">out_dim</span>: <span class="nc">PhantomData</span><span class="o">&lt;</span><span class="n">D</span><span class="o">&gt;</span><span class="p">,</span><span class="w"></span>
<span class="w">    </span><span class="n">indices</span>: <span class="nc">T</span><span class="p">,</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>where <code>T</code> was <code>[SliceOrIndex; n]</code> when the <code>SliceInfo</code> was produced by <code>s![]</code>.</p>

<h3 id="constructing-sliceinfo">Constructing <code>SliceInfo</code></h3>

<p>In order to construct a <code>SliceInfo</code> instance, the <code>s![]</code> macro had to build an
array <code>[SliceOrIndex; n]</code> and determine the output dimension type after
slicing. Both problems were solved with traits:</p>

<ul>
<li>Use <code>Into&lt;SliceOrIndex&gt;</code> to convert each argument to <code>SliceOrIndex</code>.</li>
<li>Use a new trait <code>SliceNextDim</code> to determine the output dimension type.</li>
</ul>

<p>The <code>SliceNextDim</code> trait provided a way to update the output dimension while
iterating over the arguments.</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">SliceNextDim</span><span class="o">&lt;</span><span class="n">D1</span><span class="p">,</span><span class="w"> </span><span class="n">D2</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="k">fn</span> <span class="nf">next_dim</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">PhantomData</span><span class="o">&lt;</span><span class="n">D1</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">PhantomData</span><span class="o">&lt;</span><span class="n">D2</span><span class="o">&gt;</span><span class="p">;</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>Starting with <code>$dim = PhantomData&lt;Ix0&gt;</code> (0-dimensional) and calling
<code>SliceNextDim(&amp;$arg, $dim)</code> on each argument <code>$arg</code> to update <code>$dim</code> determined
the output dimension. The implementation of <code>SliceNextDim</code> for index (subview)
arguments specified that <code>D2 = D1</code> since the corresponding axes <em>were not</em>
included in the output, while the implementation for range (slice) arguments
specified that <code>D2 = D1::Larger</code> since the corresponding axes <em>were</em> included
in the output.</p>

<p>This was great because all the types are determined automatically at compile
time! The return type from <code>s![]</code> was <code>&amp;SliceInfo&lt;[SliceOrIndex; n], D&gt;</code>.</p>

<h3 id="argument-coercions">Argument coercions</h3>

<p>The signature for <code>.slice()</code> was now</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">fn</span> <span class="nf">slice</span><span class="o">&lt;</span><span class="n">Do</span><span class="o">&gt;</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">info</span>: <span class="kp">&amp;</span><span class="nc">SliceInfo</span><span class="o">&lt;</span><span class="n">D</span>::<span class="n">SliceArg</span><span class="p">,</span><span class="w"> </span><span class="n">Do</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">Do</span><span class="o">&gt;</span><span class="w"></span>
<span class="k">where</span><span class="w"></span>
<span class="w">    </span><span class="n">Do</span>: <span class="nc">Dimension</span><span class="p">;</span><span class="w"></span>
</code></pre></div>

<p>where <code>D::SliceArg</code> was <code>[SliceOrIndex; n]</code> for fixed dimensions and
<code>[SliceOrIndex]</code> for the dynamic dimension (<code>IxDyn</code>). Automatic coercion of
<code>&amp;SliceInfo&lt;[SliceOrIndex; n], Do&gt;</code> to <code>&amp;SliceInfo&lt;[SliceOrIndex], Do&gt;</code> made
the output of <code>s![]</code> work for both fixed and dynamic dimensions.</p>

<p>The <code>.slice_inplace()</code> method didn’t change the dimensionality of the
array/view, so it didn’t need the <code>Do</code> type argument. It’s signature remained</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">pub</span><span class="w"> </span><span class="k">fn</span> <span class="nf">slice_inplace</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">indices</span>: <span class="kp">&amp;</span><span class="nc">D</span>::<span class="n">SliceArg</span><span class="p">);</span><span class="w"></span>
</code></pre></div>

<p>This worked nicely when <code>self</code> had a fixed dimension because <code>SliceInfo&lt;T, D&gt;</code>
implemented <code>Deref { type Target = T; ... }</code> so that <code>&amp;SliceInfo&lt;[SliceOrIndex;
n], Do&gt;</code> coerced to <code>&amp;[SliceOrIndex; n]</code>. Unfortunately, the compiler was not
able to combine the <code>deref</code> coercion and an automatic coercion to coerce
<code>&amp;SliceInfo&lt;[SliceOrIndex; n], Do&gt;</code> to <code>&amp;[SliceOrIndex]</code>. However, the caller
could manually call <code>.as_ref()</code> to perform the conversion.</p>

<p>It would be possible to work around coercion limitations by using a generic
type argument <code>I</code> that implemented <code>Into&lt;&amp;D::SliceArg&gt;</code>, but a design goal was
to minimize the use of generics.</p>

<h3 id="other-type-conversions">Other type conversions</h3>

<p>I also wanted to support <code>SliceInfo&lt;Vec&lt;SliceOrIndex&gt;, D&gt;</code> because older
versions of <code>ndarray</code> cleanly handled <code>Vec&lt;Si&gt;</code>. To make this convenient, an
implementation of <code>AsRef</code> was provided to convert
<code>&amp;SliceInfo&lt;Vec&lt;SliceOrIndex&gt;, D&gt;</code> to <code>&amp;SliceInfo&lt;[SliceOrIndex], D&gt;</code>.
Specifically,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">impl</span><span class="o">&lt;</span><span class="n">T</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="w"> </span><span class="nb">AsRef</span><span class="o">&lt;</span><span class="n">SliceInfo</span><span class="o">&lt;</span><span class="p">[</span><span class="n">SliceOrIndex</span><span class="p">],</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;&gt;</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">SliceInfo</span><span class="o">&lt;</span><span class="n">T</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="w"></span>
<span class="k">where</span><span class="w"></span>
<span class="w">    </span><span class="n">T</span>: <span class="nb">AsRef</span><span class="o">&lt;</span><span class="p">[</span><span class="n">SliceOrIndex</span><span class="p">]</span><span class="o">&gt;</span><span class="p">,</span><span class="w"></span>
<span class="w">    </span><span class="n">D</span>: <span class="nc">Dimension</span><span class="p">,</span><span class="w"></span>
<span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="k">fn</span> <span class="nf">as_ref</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="nc">SliceInfo</span><span class="o">&lt;</span><span class="p">[</span><span class="n">SliceOrIndex</span><span class="p">],</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="p">;</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>At first glance, this appeared to be impossible to implement. It seemed like
the body of the method needed to create a new <code>SliceInfo</code> instance. However, if
the function returned a reference to the new <code>SliceInfo</code>, the reference would
be dangling because the <code>SliceInfo</code> would be dropped at the end of the function
body.</p>

<p><a href="https://github.com/bluss">bluss</a> made the astute observation that <code>&amp;SliceInfo&lt;T, D&gt;</code> had the same
bitwise representation as <code>&amp;T</code> because the <code>out_dim: PhantomData&lt;D&gt;</code> field in
<code>SliceInfo</code> was a zero-sized type, and the only remaining field was <code>indices:
T</code>. This meant that the conversion could be implemented with pointer casts</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="k">unsafe</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="o">&amp;*</span><span class="p">(</span><span class="bp">self</span><span class="p">.</span><span class="n">indices</span><span class="p">.</span><span class="n">as_ref</span><span class="p">()</span><span class="w"> </span><span class="k">as</span><span class="w"> </span><span class="o">*</span><span class="k">const</span><span class="w"> </span><span class="p">[</span><span class="n">SliceOrIndex</span><span class="p">]</span><span class="w"></span>
<span class="w">        </span><span class="k">as</span><span class="w"> </span><span class="o">*</span><span class="k">const</span><span class="w"> </span><span class="n">SliceInfo</span><span class="o">&lt;</span><span class="p">[</span><span class="n">SliceOrIndex</span><span class="p">],</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="p">)</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>We also added a <code>#[repr(C)]</code> attribute to <code>SliceInfo</code> for extra safety.</p>

<h3 id="evaluation-of-macro-arguments">Evaluation of macro arguments</h3>

<p>My first implementation of the <code>s![]</code> macro had a bug. Try to spot the bug in
this piece of the implementation. The <code>$r:expr;$s:expr</code> pattern correctly
matched a <code>range;step</code> argument.</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="n">macro_rules</span><span class="o">!</span><span class="w"> </span><span class="n">s</span><span class="p">(</span><span class="w"></span>
<span class="w">    </span><span class="c1">// ...</span>
<span class="w">    </span><span class="p">(</span><span class="o">@</span><span class="n">parse</span><span class="w"> </span><span class="cp">$dim</span>:<span class="nc">expr</span><span class="p">,</span><span class="w"> </span><span class="p">[</span><span class="cp">$($stack</span>:<span class="nc">tt</span><span class="p">)</span><span class="o">*</span><span class="p">]</span><span class="w"> </span><span class="cp">$r</span>:<span class="nc">expr</span><span class="p">;</span><span class="cp">$s</span>:<span class="nc">expr</span><span class="p">,</span><span class="w"> </span><span class="cp">$($t</span>:<span class="nc">tt</span><span class="p">)</span><span class="o">*</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">        </span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="o">@</span><span class="n">parse</span><span class="w"></span>
<span class="w">            </span><span class="cp">$crate</span>::<span class="n">SliceNextDim</span>::<span class="n">next_dim</span><span class="p">(</span><span class="o">&amp;</span><span class="cp">$r</span><span class="p">,</span><span class="w"> </span><span class="cp">$dim</span><span class="p">),</span><span class="w"></span>
<span class="w">            </span><span class="p">[</span><span class="cp">$($stack</span><span class="p">)</span><span class="o">*</span><span class="w"> </span><span class="n">s</span><span class="o">!</span><span class="p">(</span><span class="o">@</span><span class="n">convert</span><span class="w"> </span><span class="cp">$r</span><span class="p">,</span><span class="w"> </span><span class="cp">$s</span><span class="p">),]</span><span class="w"></span>
<span class="w">            </span><span class="cp">$($t</span><span class="p">)</span><span class="o">*</span><span class="w"></span>
<span class="w">        </span><span class="p">]</span><span class="w"></span>
<span class="w">    </span><span class="p">};</span><span class="w"></span>
<span class="w">    </span><span class="c1">// ...</span>
<span class="p">);</span><span class="w"></span>
</code></pre></div>

<p>The issue was that the <code>$r</code> expression was expanded in two places, so it was
evaluated twice at runtime. Unfortunately, this issue wasn’t a problem in the
common case (literal range and index arguments) so it took me a while to notice
it.</p>

<p>The solution was to use <code>match</code> to evaluate <code>$r</code> once and create a binding <code>r</code>
that could be used in both places:</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="n">macro_rules</span><span class="o">!</span><span class="w"> </span><span class="n">s</span><span class="p">(</span><span class="w"></span>
<span class="w">    </span><span class="c1">// ...</span>
<span class="w">    </span><span class="p">(</span><span class="o">@</span><span class="n">parse</span><span class="w"> </span><span class="cp">$dim</span>:<span class="nc">expr</span><span class="p">,</span><span class="w"> </span><span class="p">[</span><span class="cp">$($stack</span>:<span class="nc">tt</span><span class="p">)</span><span class="o">*</span><span class="p">]</span><span class="w"> </span><span class="cp">$r</span>:<span class="nc">expr</span><span class="p">;</span><span class="cp">$s</span>:<span class="nc">expr</span><span class="p">,</span><span class="w"> </span><span class="cp">$($t</span>:<span class="nc">tt</span><span class="p">)</span><span class="o">*</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">        </span><span class="k">match</span><span class="w"> </span><span class="cp">$r</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">            </span><span class="n">r</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">                </span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="o">@</span><span class="n">parse</span><span class="w"></span>
<span class="w">                   </span><span class="cp">$crate</span>::<span class="n">SliceNextDim</span>::<span class="n">next_dim</span><span class="p">(</span><span class="o">&amp;</span><span class="n">r</span><span class="p">,</span><span class="w"> </span><span class="cp">$dim</span><span class="p">),</span><span class="w"></span>
<span class="w">                   </span><span class="p">[</span><span class="cp">$($stack</span><span class="p">)</span><span class="o">*</span><span class="w"> </span><span class="n">s</span><span class="o">!</span><span class="p">(</span><span class="o">@</span><span class="n">convert</span><span class="w"> </span><span class="n">r</span><span class="p">,</span><span class="w"> </span><span class="cp">$s</span><span class="p">),]</span><span class="w"></span>
<span class="w">                   </span><span class="cp">$($t</span><span class="p">)</span><span class="o">*</span><span class="w"></span>
<span class="w">                </span><span class="p">]</span><span class="w"></span>
<span class="w">            </span><span class="p">}</span><span class="w"></span>
<span class="w">        </span><span class="p">}</span><span class="w"></span>
<span class="w">    </span><span class="p">};</span><span class="w"></span>
<span class="w">    </span><span class="c1">// ...</span>
<span class="p">);</span><span class="w"></span>
</code></pre></div>

<p>This illustrates an important difference between function arguments and macro
expression arguments. If you need to use the value of an expression in multiple
places in a macro, you have to be careful not to accidentally evaluate the
expression multiple times.</p>

<p>When using a macro that acts like a function (e.g. <code>s![]</code> or <code>vec![]</code>), it’s
easy to ignore that the macro call is really an expansion into an expression,
not a function call. This is usually okay when calling macros, but it’s
important to keep the difference in mind when writing macros.</p>

<p>Note that the solution uses <code>match $r { r =&gt; expr }</code> instead of <code>{ let r = $r;
expr }</code> because <code>match</code> preserves temporary objects created in <code>$r</code> for the
entire body. Thanks to <a href="https://github.com/bluss">bluss</a> for pointing this out. For example,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="c1">// error</span>
<span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">r</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">vec</span><span class="o">!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">].</span><span class="n">iter</span><span class="p">();</span><span class="w"></span>
<span class="w">    </span><span class="n">r</span><span class="p">.</span><span class="n">count</span><span class="p">()</span><span class="w"></span>
<span class="p">};</span><span class="w"></span>

<span class="c1">// ok</span>
<span class="k">match</span><span class="w"> </span><span class="n">vec</span><span class="o">!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">].</span><span class="n">iter</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="n">r</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">        </span><span class="n">r</span><span class="p">.</span><span class="n">count</span><span class="p">()</span><span class="w"></span>
<span class="w">    </span><span class="p">}</span><span class="w"></span>
<span class="p">};</span><span class="w"></span>
</code></pre></div>

<h2 id="conclusion">Conclusion</h2>

<p>Implementing the combined slicing and subviews feature in <code>ndarray</code> 0.11 was a
lot of fun and a great learning experience. There were a few important lessons:</p>

<ul>
<li><p>Macros and traits are a surprisingly powerful combination for doing complex
things at compile time. For example, the <code>s![]</code> macro is able to determine at
compile time the output dimension of the sliced array/view from the number
and types of its arguments.</p></li>

<li><p>Keep in mind low-level solutions to problems. Rust is a safe, high-level
language, so it’s easy to forget what’s actually happening under-the-hood. In
some cases, low-level solutions such as pointer casts are a simple way to
work around high-level limitations.</p></li>

<li><p>Be careful not to treat macro arguments the same as function arguments, even
if the macro looks a lot like a function. If you&rsquo;re using an expression in
two places and don&rsquo;t want it to be evaluated twice, use <code>match</code> to evaluate
it and create a binding, and then use that binding.</p></li>
</ul>