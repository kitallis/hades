+++
title = "Idiomatic tree and graph like structures in Rust"
date = "2016-12-20T00:00:00+00:00"

[extra]
author = "Rust Leipzig"
link = "https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/"
+++
<p>If you want to create data structures which can be modified during runtime, a possible solution could lead into tree or
graph like structures. Writing tree structures in Rust is no trivial problem. Nevertheless there are some common
idiomatic ways how to handle lifetime and borrowing issues. In other languages like C/C++ we would simply use pointers
to create graphs or trees. This is also possible in Rust, but the thing is that this would kill every benefit from the
borrow checker and could lead into the same pitfalls like in other languages.</p>

<p>I tried a lots of implementations of tree like structures, which should be dynamically modifiable during runtime. One of
the first implementations used interior mutability with data structures like:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="k">use</span> <span class="nn">std</span><span class="p">::</span><span class="nn">rc</span><span class="p">::</span><span class="nb">Rc</span><span class="p">;</span>
<span class="k">use</span> <span class="nn">std</span><span class="p">::</span><span class="nn">cell</span><span class="p">::</span><span class="n">RefCell</span><span class="p">;</span>

<span class="k">struct</span> <span class="n">Node</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span> <span class="p">{</span>
    <span class="n">previous</span><span class="p">:</span> <span class="nb">Rc</span><span class="o">&lt;</span><span class="n">RefCell</span><span class="o">&lt;</span><span class="nb">Box</span><span class="o">&lt;</span><span class="n">Node</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;&gt;&gt;&gt;</span><span class="p">,</span>
    <span class="c">//        ^  ^       ^   ^</span>
    <span class="c">//        |  |       |   |</span>
    <span class="c">//        |  |       |   - The next Node with generic `T`</span>
    <span class="c">//        |  |       |</span>
    <span class="c">//        |  |       - Heap allocated memory, needed</span>
    <span class="c">//        |  |         if `T` is a trait object.</span>
    <span class="c">//        |  |</span>
    <span class="c">//        |  - A mutable memory location with</span>
    <span class="c">//        |    dynamically checked borrow rules.</span>
    <span class="c">//        |    Needed because `Box` is immutable.</span>
    <span class="c">//        |</span>
    <span class="c">//        - Reference counted pointer, will be</span>
    <span class="c">//          dropped when every reference is gone.</span>
    <span class="c">//          Needed to create multiple node references.</span>

    <span class="n">next</span><span class="p">:</span> <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">Rc</span><span class="o">&lt;</span><span class="n">RefCell</span><span class="o">&lt;</span><span class="nb">Box</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;&gt;&gt;&gt;</span><span class="p">,</span>
    <span class="n">data</span><span class="p">:</span> <span class="n">T</span><span class="p">,</span>
    <span class="c">// ...</span>
<span class="p">}</span>
</code></pre>
</div>

<p>This is on one hand hard to understand and will on the other hand lead into runtime borrow checks, which are not that
idiomatic Rust. Furthermore back references are simply not possible with this approach, since cyclic references are not
allowed within Rust. The main reason is that cyclic references could lead very fast into memory leaks if we are not
really careful. Also, this approach is not multi threading capable, but we could use an <code class="highlighter-rouge">Arc</code> instead of <code class="highlighter-rouge">Rc</code>, which will
decrease the overall performance.</p>

<p>To reduce the complexity of the problem we need to reduce the lifetime complexity within the structures. A good
approach is to use a <a href="https://en.wikipedia.org/wiki/Region-based_memory_management">memory arena</a> for the Nodes, because
this implies that every element within the arena has the same lifetime. How would such an arena look like? What about a
simple vector:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="k">pub</span> <span class="k">struct</span> <span class="n">Arena</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span> <span class="p">{</span>
    <span class="n">nodes</span><span class="p">:</span> <span class="nb">Vec</span><span class="o">&lt;</span><span class="n">Node</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;&gt;</span><span class="p">,</span>
<span class="p">}</span>
</code></pre>
</div>

<p>A node could then look like this:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="k">pub</span> <span class="k">struct</span> <span class="n">Node</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span> <span class="p">{</span>
    <span class="n">parent</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">NodeId</span><span class="o">&gt;</span><span class="p">,</span>
    <span class="n">previous_sibling</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">NodeId</span><span class="o">&gt;</span><span class="p">,</span>
    <span class="n">next_sibling</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">NodeId</span><span class="o">&gt;</span><span class="p">,</span>
    <span class="n">first_child</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">NodeId</span><span class="o">&gt;</span><span class="p">,</span>
    <span class="n">last_child</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="n">NodeId</span><span class="o">&gt;</span><span class="p">,</span>

    <span class="c">/// The actual data which will be stored within the tree</span>
    <span class="k">pub</span> <span class="n">data</span><span class="p">:</span> <span class="n">T</span><span class="p">,</span>
<span class="p">}</span>

<span class="k">pub</span> <span class="k">struct</span> <span class="n">NodeId</span> <span class="p">{</span>
    <span class="n">index</span><span class="p">:</span> <span class="nb">usize</span><span class="p">,</span>
<span class="p">}</span>
</code></pre>
</div>

<p>This means we store the actual <code class="highlighter-rouge">Node</code> within the <code class="highlighter-rouge">Vec</code>, but for creating the tree, we simply use the indexes from the
vector. To create a new node we can use this method:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="k">pub</span> <span class="k">fn</span> <span class="nf">new_node</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span> <span class="k">self</span><span class="p">,</span> <span class="n">data</span><span class="p">:</span> <span class="n">T</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">NodeId</span> <span class="p">{</span>
    <span class="c">// Get the next free index</span>
    <span class="k">let</span> <span class="n">next_index</span> <span class="o">=</span> <span class="k">self</span><span class="py">.nodes</span><span class="nf">.len</span><span class="p">();</span>

    <span class="c">// Push the node into the arena</span>
    <span class="k">self</span><span class="py">.nodes</span><span class="nf">.push</span><span class="p">(</span><span class="n">Node</span> <span class="p">{</span>
        <span class="n">parent</span><span class="p">:</span> <span class="nb">None</span><span class="p">,</span>
        <span class="n">first_child</span><span class="p">:</span> <span class="nb">None</span><span class="p">,</span>
        <span class="n">last_child</span><span class="p">:</span> <span class="nb">None</span><span class="p">,</span>
        <span class="n">previous_sibling</span><span class="p">:</span> <span class="nb">None</span><span class="p">,</span>
        <span class="n">next_sibling</span><span class="p">:</span> <span class="nb">None</span><span class="p">,</span>
        <span class="n">data</span><span class="p">:</span> <span class="n">data</span><span class="p">,</span>
    <span class="p">});</span>

    <span class="c">// Return the node identifier</span>
    <span class="n">NodeId</span> <span class="p">{</span> <span class="n">index</span><span class="p">:</span> <span class="n">next_index</span> <span class="p">}</span>
<span class="p">}</span>
</code></pre>
</div>

<p>This approach makes creating graph like structures very easy, where they can contain any data. A general multi
processing is also possible since parts of a vector can shared across threads safely.</p>

<p>To try it out, simple use the <a href="https://github.com/saschagrunert/indextree">indextree crate</a> and create graph like
structures for nearly everything.</p>