+++
title = "`ndarray` 0.11"
date = "2017-12-29T00:00:00+00:00"

[extra]
author = "Rust — Jim Turner"
link = "https://jim.turner.link/pages/ndarray-0.11/"
+++
<p><a href="https://crates.io/crates/ndarray"><code>ndarray</code></a> 0.11.0 was just released. <code>ndarray</code> is a <a href="https://www.rust-lang.org/">Rust</a> crate that
provides an <em>n</em>-dimensional array type <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html"><code>ArrayBase</code></a> for general elements and
for numerics. Arrays are <em>n</em>-dimensional, so they can represent vectors (1
axis), matrices (2 axes), etc., to <em>n</em> axes. <code>ndarray</code> provides methods for
<em>n</em>-dimensional slicing, iteration, view-taking, and some mathematical
operations.</p>

<p><code>ndarray</code> 0.11 has an exciting new feature: combined slicing and subviews!</p>

<h2 id="combined-slicing-and-subviews">Combined slicing and subviews</h2>

<p>The biggest new feature in <code>ndarray</code> 0.11 is the ability to combine slicing and
subviews in a single operation. To get the same result in earlier versions of
<code>ndarray</code>, you’d have to chain together multiple <a href="https://docs.rs/ndarray/0.10.13/ndarray/struct.ArrayBase.html#method.slice"><code>.slice()</code></a> and
<a href="https://docs.rs/ndarray/0.10.13/ndarray/struct.ArrayBase.html#method.into_subview"><code>.into_subview()</code></a> calls while being careful with the order
of the calls. For example,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="kd">let</span><span class="w"> </span><span class="n">arr</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">array</span><span class="o">!</span><span class="p">[[[</span><span class="w"> </span><span class="mi">1</span><span class="p">,</span><span class="w">  </span><span class="mi">2</span><span class="p">,</span><span class="w">  </span><span class="mi">3</span><span class="p">,</span><span class="w">  </span><span class="mi">4</span><span class="p">],</span><span class="w"></span>
<span class="w">                  </span><span class="p">[</span><span class="w"> </span><span class="mi">5</span><span class="p">,</span><span class="w">  </span><span class="mi">6</span><span class="p">,</span><span class="w">  </span><span class="mi">7</span><span class="p">,</span><span class="w">  </span><span class="mi">8</span><span class="p">]],</span><span class="w"></span>
<span class="w">                 </span><span class="p">[[</span><span class="w"> </span><span class="mi">9</span><span class="p">,</span><span class="w"> </span><span class="mi">10</span><span class="p">,</span><span class="w"> </span><span class="mi">11</span><span class="p">,</span><span class="w"> </span><span class="mi">12</span><span class="p">],</span><span class="w"></span>
<span class="w">                  </span><span class="p">[</span><span class="mi">13</span><span class="p">,</span><span class="w"> </span><span class="mi">14</span><span class="p">,</span><span class="w"> </span><span class="mi">15</span><span class="p">,</span><span class="w"> </span><span class="mi">16</span><span class="p">]]];</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="n">arr</span><span class="p">.</span><span class="n">shape</span><span class="p">(),</span><span class="w"> </span><span class="o">&amp;</span><span class="p">[</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">4</span><span class="p">]);</span><span class="w"></span>

<span class="c1">// old</span>
<span class="kd">let</span><span class="w"> </span><span class="n">slice</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="o">..</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">,</span><span class="w"> </span><span class="mi">1</span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">])</span><span class="w"></span>
<span class="w">    </span><span class="p">.</span><span class="n">into_subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">1</span><span class="p">),</span><span class="w"> </span><span class="mi">0</span><span class="p">)</span><span class="w"></span>
<span class="w">    </span><span class="p">.</span><span class="n">into_subview</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">0</span><span class="p">),</span><span class="w"> </span><span class="mi">1</span><span class="p">);</span><span class="w"></span>

<span class="c1">// new</span>
<span class="kd">let</span><span class="w"> </span><span class="n">slice</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">slice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">0</span><span class="p">,</span><span class="w"> </span><span class="mi">1</span><span class="o">..</span><span class="p">;</span><span class="mi">2</span><span class="p">]);</span><span class="w"></span>

<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="n">slice</span><span class="p">.</span><span class="n">shape</span><span class="p">(),</span><span class="w"> </span><span class="o">&amp;</span><span class="p">[</span><span class="mi">2</span><span class="p">]);</span><span class="w"></span>
<span class="n">assert_eq</span><span class="o">!</span><span class="p">(</span><span class="n">slice</span><span class="p">,</span><span class="w"> </span><span class="n">array</span><span class="o">!</span><span class="p">[</span><span class="mi">10</span><span class="p">,</span><span class="w"> </span><span class="mi">12</span><span class="p">]);</span><span class="w"></span>
</code></pre></div>

<p>In this example, the <code>1..;2</code> indicates a slice of axis 2 (starting at index 1
and stepping by 2), and the <code>1</code> and <code>0</code> indicate subviews of axes 0 and 1,
respectively. The resulting <code>slice</code> is a view of a subset of <code>arr</code> (without
copying). The new version is more concise and is easier to use correctly than a
series of <code>.slice()</code> and <code>.into_subview()</code> calls.</p>

<p>You’ll still get a compile-time error if the dimensionality of the array/view
being sliced doesn’t match the number of arguments in the <a href="https://docs.rs/ndarray/0.11.0/ndarray/macro.s.html"><code>s![]</code></a>
macro, and the dimensionality of the sliced array/view is still determined at
compile-time.</p>

<p>You may be interested in <a href="https://jim.turner.link/pages/impl-slicing-subviews-ndarray/">how this feature was implemented</a>. It&rsquo;s a story of type hacking:
zero-sized types, associated types, traits for compile-time type manipulation,
and pointer casting.</p>

<h2 id="other-new-features">Other new features</h2>

<p><a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html"><code>ArrayBase</code></a> has new <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice_axis"><code>.slice_axis()</code></a>, <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice_axis_mut"><code>.slice_axis_mut()</code></a>, and
<a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice_axis_inplace"><code>.slice_axis_inplace()</code></a> methods to slice individual axes. These methods
simplify functions that slice generic-dimensional arrays/views. For example,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="c1">// old</span>
<span class="k">fn</span> <span class="nf">reverse_first_axis</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span>: <span class="nc">Dimension</span><span class="o">&gt;</span><span class="p">(</span><span class="n">view</span>: <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">slice_info</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">vec</span><span class="o">!</span><span class="p">[</span><span class="n">Si</span><span class="p">(</span><span class="mi">0</span><span class="p">,</span><span class="w"> </span><span class="nb">None</span><span class="p">,</span><span class="w"> </span><span class="mi">1</span><span class="p">);</span><span class="w"> </span><span class="n">view</span><span class="p">.</span><span class="n">ndim</span><span class="p">()];</span><span class="w"></span>
<span class="w">    </span><span class="n">slice_info</span><span class="p">[</span><span class="mi">0</span><span class="p">].</span><span class="mi">2</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="o">-</span><span class="mi">1</span><span class="p">;</span><span class="w"></span>
<span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">view_dyn</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">view</span><span class="p">.</span><span class="n">into_dyn</span><span class="p">();</span><span class="w"></span>
<span class="w">    </span><span class="n">view_dyn</span><span class="p">.</span><span class="n">islice</span><span class="p">(</span><span class="o">&amp;</span><span class="n">slice_info</span><span class="p">);</span><span class="w"></span>
<span class="w">    </span><span class="n">view_dyn</span><span class="p">.</span><span class="n">into_dimensionality</span><span class="p">().</span><span class="n">unwrap</span><span class="p">()</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>

<span class="c1">// new</span>
<span class="k">fn</span> <span class="nf">reverse_first_axis</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span>: <span class="nc">Dimension</span><span class="o">&gt;</span><span class="p">(</span><span class="k">mut</span><span class="w"> </span><span class="n">view</span>: <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView</span><span class="o">&lt;</span><span class="n">A</span><span class="p">,</span><span class="w"> </span><span class="n">D</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="n">view</span><span class="p">.</span><span class="n">slice_axis_inplace</span><span class="p">(</span><span class="n">Axis</span><span class="p">(</span><span class="mi">0</span><span class="p">),</span><span class="w"> </span><span class="n">Slice</span>::<span class="n">new</span><span class="p">(</span><span class="mi">0</span><span class="p">,</span><span class="w"> </span><span class="nb">None</span><span class="p">,</span><span class="w"> </span><span class="o">-</span><span class="mi">1</span><span class="p">));</span><span class="w"></span>
<span class="w">    </span><span class="n">view</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p><code>ArrayBase</code> has a new <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice_move"><code>.slice_move()</code></a> method which consumes the array/view
and returns a new one. This method is necessary to be able to change the number
of dimensions when slicing an owned array into a smaller owned array. (The
<a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice_inplace"><code>.slice_inplace()</code></a> method (called <a href="https://docs.rs/ndarray/0.10.13/ndarray/struct.ArrayBase.html#method.islice"><code>.islice()</code></a> in 0.10) can’t change the
number of dimensions because it can’t change the type of its argument.) Adding
<code>.slice_move()</code> also has the nice side-effect that some uses of slicing are now
a little more ergonomic, especially when taking ownership of <code>ArrayView</code>s. For
example,</p>
<div class="highlight"><pre><code class="language-rust" data-lang="rust"><span></span><span class="c1">// old</span>
<span class="k">fn</span> <span class="nf">reverse_view</span><span class="o">&lt;</span><span class="n">A</span><span class="o">&gt;</span><span class="p">(</span><span class="k">mut</span><span class="w"> </span><span class="n">view</span>: <span class="nc">ArrayView1</span><span class="o">&lt;</span><span class="n">A</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView1</span><span class="o">&lt;</span><span class="n">A</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="n">view</span><span class="p">.</span><span class="n">islice</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="o">..</span><span class="p">;</span><span class="o">-</span><span class="mi">1</span><span class="p">]);</span><span class="w"></span>
<span class="w">    </span><span class="n">view</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>

<span class="c1">// new</span>
<span class="k">fn</span> <span class="nf">reverse_view</span><span class="o">&lt;</span><span class="n">A</span><span class="o">&gt;</span><span class="p">(</span><span class="n">view</span>: <span class="nc">ArrayView1</span><span class="o">&lt;</span><span class="n">A</span><span class="o">&gt;</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">ArrayView1</span><span class="o">&lt;</span><span class="n">A</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w"></span>
<span class="w">    </span><span class="n">view</span><span class="p">.</span><span class="n">slice_move</span><span class="p">(</span><span class="n">s</span><span class="o">!</span><span class="p">[</span><span class="o">..</span><span class="p">;</span><span class="o">-</span><span class="mi">1</span><span class="p">])</span><span class="w"></span>
<span class="p">}</span><span class="w"></span>
</code></pre></div>

<p>The <a href="https://docs.rs/ndarray/0.11.0/ndarray/macro.s.html"><code>s![]</code></a> macro now supports more index types, such as <code>usize</code>, so
you no longer have to manually cast <code>usize</code> indices to <code>isize</code>.</p>

<p>We’ve added many other new features since the <a href="https://bluss.github.io//rust/2017/04/09/ndarray-0.9/">last release
announcement</a>. See the
<a href="https://github.com/bluss/rust-ndarray/blob/master/README.rst#recent-changes-ndarray">full release notes</a> for more!</p>

<h2 id="updating-existing-code">Updating existing code</h2>

<p>There are breaking changes in <code>ndarray</code> 0.11, but updating most code written
for 0.10 is fairly straightforward. The most common updates will be:</p>

<ul>
<li>Rename <a href="https://docs.rs/ndarray/0.10.13/ndarray/struct.ArrayBase.html#method.islice"><code>.islice()</code></a> to <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.slice_inplace"><code>.slice_inplace()</code></a>.</li>
<li>Rename <a href="https://docs.rs/ndarray/0.10.13/ndarray/struct.ArrayBase.html#method.isubview"><code>.isubview()</code></a> to <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.ArrayBase.html#method.subview_inplace"><code>.subview_inplace()</code></a>.</li>
</ul>

<p>If you explicitly used the old <a href="https://docs.rs/ndarray/0.10.13/ndarray/struct.Si.html"><code>Si</code></a> type, switch to using the
<a href="https://docs.rs/ndarray/0.11.0/ndarray/macro.s.html"><code>s![]</code></a> macro if possible. If you need to pass around slicing
information for a single axis, you can use the new <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.SliceInfo.html"><code>Slice</code></a> type, which is
very similar to the old <code>Si</code> type and can be used as an argument to the <code>s![]</code>
macro and the <code>.slice_axis*()</code> methods. If <code>s![]</code> doesn’t work for your
use-case, <a href="https://docs.rs/ndarray/0.11.0/ndarray/struct.SliceInfo.html"><code>SliceInfo</code></a> is analogous to the old <code>[Si; n]</code> type and can be
constructed manually from an array/slice/<code>Vec</code> of <a href="https://docs.rs/ndarray/0.11.0/ndarray/enum.SliceOrIndex.html"><code>SliceOrIndex</code></a>.</p>

<p>The output of <code>s![]</code> may not automatically coerce into the parameter type for
<code>.slice*()</code> in all of the places it did in earlier versions of <code>ndarray</code>
(primarily when dealing with dynamic-dimensional arrays). You can call
<code>.as_ref()</code> on the output of <code>s![]</code> to perform the necessary conversion.</p>

<p>See the <a href="https://github.com/bluss/rust-ndarray/blob/master/README.rst#recent-changes-ndarray">full release notes</a> for more.</p>

<h2 id="thanks">Thanks</h2>

<p>Many thanks to <a href="https://github.com/bluss">bluss</a> for maintaining <code>ndarray</code> and helping me (<a href="https://github.com/jturner314">jturner314</a>)
refine the design of the new features, and, of course, for creating <code>ndarray</code>
in the first place! Thanks also to various people on IRC for answering my Rust
questions.</p>