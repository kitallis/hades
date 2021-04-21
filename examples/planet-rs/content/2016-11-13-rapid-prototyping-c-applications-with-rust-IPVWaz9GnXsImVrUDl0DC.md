+++
title = "Rapid prototyping C applications with Rust"
date = "2016-11-13T00:00:00+00:00"

[extra]
author = "Rust Leipzig"
link = "https://rust-leipzig.github.io/cargo/2016/11/13/rapid-prototyping-c-applications/"
+++
<p><a href="https://github.com/rust-lang/cargo">Cargo</a> is a gorgeous tool when it comes to building, packaging and shipping your
own Rust applications. Some of you guys are living in a C/C++ world where developing Rust applications is just a hobby.
A reason could be that Rust is currently not that highly accepted in the world of <em>applications in production</em>. This is
sad, but even within your day to day development it could be possible to use Rust for a lots of tools or prototyping
scenarios!</p>

<p>One example could be to use the possibilities of Rust and Cargo to do prototyping of C based applications. So how could
we do that?</p>

<p>For the first try we simply build a new cargo project:</p>

<div class="highlighter-rouge"><pre class="highlight"><code>&gt; cargo new --bin rapidc
     Created binary (application) `rapidc` project
</code></pre>
</div>

<p>The target is now to come as fast as possible back into the C world. For this we add the two dependencies <code class="highlighter-rouge">libc</code> and
<code class="highlighter-rouge">gcc</code>, as well as a custom build script <code class="highlighter-rouge">build.rs</code> to our <code class="highlighter-rouge">Cargo.toml</code> file:</p>

<div class="language-toml highlighter-rouge"><pre class="highlight"><code><span class="nn">[package]</span>
<span class="py">name</span> <span class="p">=</span> <span class="s">"rapidc"</span>
<span class="py">version</span> <span class="p">=</span> <span class="s">"0.1.0"</span>
<span class="py">build</span> <span class="p">=</span> <span class="s">"build.rs"</span>

<span class="nn">[dependencies]</span>
<span class="py">libc</span> <span class="p">=</span> <span class="s">"0"</span>

<span class="nn">[build-dependencies]</span>
<span class="py">gcc</span> <span class="p">=</span> <span class="s">"0"</span>
</code></pre>
</div>

<p>What else? For sure we need an entry point for our C application, which we name <code class="highlighter-rouge">src/main.c</code> and looks like:</p>

<div class="language-c highlighter-rouge"><pre class="highlight"><code><span class="cp">#include &lt;stdio.h&gt;
</span>
<span class="kt">int</span> <span class="nf">main_c</span><span class="p">(</span><span class="kt">int</span> <span class="n">argc</span><span class="p">,</span> <span class="kt">char</span> <span class="o">*</span><span class="n">argv</span><span class="p">[])</span>
<span class="p">{</span>
    <span class="n">printf</span><span class="p">(</span><span class="s">"Found %d args:</span><span class="se">\n</span><span class="s">"</span><span class="p">,</span> <span class="n">argc</span><span class="p">);</span>
    <span class="k">for</span> <span class="p">(</span><span class="kt">int</span> <span class="n">i</span> <span class="o">=</span> <span class="mi">0</span><span class="p">;</span> <span class="n">i</span> <span class="o">&lt;</span> <span class="n">argc</span><span class="p">;</span> <span class="o">++</span><span class="n">i</span><span class="p">)</span> <span class="p">{</span>
        <span class="n">printf</span><span class="p">(</span><span class="s">"</span><span class="se">\t</span><span class="s">- %s</span><span class="se">\n</span><span class="s">"</span><span class="p">,</span> <span class="n">argv</span><span class="p">[</span><span class="n">i</span><span class="p">]);</span>
    <span class="p">}</span>
    <span class="k">return</span> <span class="mi">0</span><span class="p">;</span>
<span class="p">}</span>
</code></pre>
</div>

<p>This program does not do really special at all, but as a demonstration we print out all available command line
arguments. The interesting part happens within the Rust entry function in <code class="highlighter-rouge">main.rs</code>:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="k">extern</span> <span class="n">crate</span> <span class="n">libc</span><span class="p">;</span>

<span class="k">use</span> <span class="nn">std</span><span class="p">::</span><span class="n">env</span><span class="p">;</span>
<span class="k">use</span> <span class="nn">std</span><span class="p">::</span><span class="nn">ffi</span><span class="p">::</span><span class="n">CString</span><span class="p">;</span>
<span class="k">use</span> <span class="nn">libc</span><span class="p">::{</span><span class="nb">c_int</span><span class="p">,</span> <span class="nb">c_char</span><span class="p">};</span>

<span class="k">extern</span> <span class="s">"C"</span> <span class="p">{</span>
    <span class="k">fn</span> <span class="nf">main_c</span><span class="p">(</span><span class="n">argc</span><span class="p">:</span> <span class="nb">c_int</span><span class="p">,</span> <span class="n">argv</span><span class="p">:</span> <span class="o">*</span><span class="k">const</span> <span class="o">*</span><span class="k">const</span> <span class="nb">c_char</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">c_int</span><span class="p">;</span>
<span class="p">}</span>

<span class="k">fn</span> <span class="nf">main</span><span class="p">()</span> <span class="p">{</span>
    <span class="c">// Get the current args and map them to a vector of zero</span>
    <span class="c">// terminated c strings:</span>
    <span class="k">let</span> <span class="n">args</span><span class="p">:</span> <span class="nb">Vec</span><span class="o">&lt;</span><span class="n">CString</span><span class="o">&gt;</span> <span class="o">=</span> <span class="nn">env</span><span class="p">::</span><span class="nf">args</span><span class="p">()</span><span class="nf">.filter_map</span><span class="p">(|</span><span class="n">arg</span><span class="p">|</span> <span class="p">{</span>
        <span class="nn">CString</span><span class="p">::</span><span class="nf">new</span><span class="p">(</span><span class="n">arg</span><span class="p">)</span><span class="nf">.ok</span><span class="p">()</span>
    <span class="p">})</span><span class="nf">.collect</span><span class="p">();</span>

    <span class="c">// Convert these c strings to raw pointers</span>
    <span class="k">let</span> <span class="n">c_args</span><span class="p">:</span> <span class="nb">Vec</span><span class="o">&lt;*</span><span class="k">const</span> <span class="nb">c_char</span><span class="o">&gt;</span> <span class="o">=</span> <span class="n">args</span><span class="nf">.iter</span><span class="p">()</span><span class="nf">.map</span><span class="p">(|</span><span class="n">arg</span><span class="p">|</span> <span class="p">{</span>
        <span class="n">arg</span><span class="nf">.as_ptr</span><span class="p">()</span>
    <span class="p">})</span><span class="nf">.collect</span><span class="p">();</span>

    <span class="c">// Call the main function within the created c library</span>
    <span class="k">unsafe</span> <span class="p">{</span>
        <span class="nf">main_c</span><span class="p">(</span><span class="n">c_args</span><span class="nf">.len</span><span class="p">()</span> <span class="k">as</span> <span class="nb">c_int</span><span class="p">,</span> <span class="n">c_args</span><span class="nf">.as_ptr</span><span class="p">());</span>
    <span class="p">};</span>
<span class="p">}</span>
</code></pre>
</div>

<p>The Foreign Function Interface (FFI) of Rust is pretty straight forward and Cargo even supports us in writing those
applications. We simply need to declare our C function within the scope of <code class="highlighter-rouge">extern "C" { ... }</code>. Since we cannot use the
basic C types within Rust we have to use the ones of the <code class="highlighter-rouge">libc</code> crate (like <code class="highlighter-rouge">c_int</code> or <code class="highlighter-rouge">c_char</code>). The <code class="highlighter-rouge">main</code> function
creates a Vector of C pointers to our command line arguments and passes them within the <code class="highlighter-rouge">unsafe {...}</code> block to our C
entry function.</p>

<p>Do we forgot something? Yes, what about the <code class="highlighter-rouge">build.rs</code> script? How can we tell Cargo to first build our C library
and then link it to the Rust application? The build script looks like:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="k">extern</span> <span class="n">crate</span> <span class="n">gcc</span><span class="p">;</span>

<span class="k">fn</span> <span class="nf">main</span><span class="p">()</span> <span class="p">{</span>
    <span class="nn">gcc</span><span class="p">::</span><span class="nn">Config</span><span class="p">::</span><span class="nf">new</span><span class="p">()</span><span class="nf">.file</span><span class="p">(</span><span class="s">"src/main.c"</span><span class="p">)</span>
                      <span class="nf">.include</span><span class="p">(</span><span class="s">"src"</span><span class="p">)</span>
                      <span class="nf">.compile</span><span class="p">(</span><span class="s">"libmain.a"</span><span class="p">);</span>
<span class="p">}</span>
</code></pre>
</div>

<p>Pretty easy right? The <code class="highlighter-rouge">gcc</code> crate helps us to easily build the <code class="highlighter-rouge">main.c</code> to the library <code class="highlighter-rouge">libmain.a</code>. Now we are able to
try out our work by executing:</p>

<div class="highlighter-rouge"><pre class="highlight"><code>&gt; cargo run
cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/rapidc`
Found 1 args:
        - target/debug/rapidc
</code></pre>
</div>

<p>Or what about:</p>

<div class="highlighter-rouge"><pre class="highlight"><code>&gt; cargo run arg1 arg2 arg3
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/rapidc arg1 arg2 arg3`
Found 4 args:
        - target/debug/rapidc
        - arg1
        - arg2
        - arg3
</code></pre>
</div>

<p>Great, everything works as expected, …but stop! What happened here? We did not tell Cargo to link against a library
called <code class="highlighter-rouge">libmain.a</code>. How should Cargo know what to do with the library? That is one of the very important features of
such a great build environment: Strong defaults will lead into a better world. Cargo will simply assume that you call
your library <code class="highlighter-rouge">libmain.a</code> for the file <code class="highlighter-rouge">main.c</code>.</p>

<p>If we want to link another library to our C application we can simply use the <code class="highlighter-rouge">link</code> directive:</p>

<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="err">#</span><span class="p">[</span><span class="nf">link</span><span class="p">(</span><span class="n">name</span> <span class="o">=</span> <span class="s">"pthread"</span><span class="p">)]</span>
<span class="k">extern</span> <span class="s">"C"</span> <span class="p">{</span>
    <span class="k">fn</span> <span class="nf">main_c</span><span class="p">(</span><span class="n">argc</span><span class="p">:</span> <span class="nb">c_int</span><span class="p">,</span> <span class="n">argv</span><span class="p">:</span> <span class="o">*</span><span class="k">const</span> <span class="o">*</span><span class="k">const</span> <span class="nb">c_char</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">c_int</span><span class="p">;</span>
<span class="p">}</span>
</code></pre>
</div>

<p>It is pretty brilliant: On one hand we have the full power of Rust and Cargo via the language itself within the
<code class="highlighter-rouge">build.rs</code> script. This means we have the full flexibility like in CMake by the usage of the complete programming
language set of Rust. On the other hand the strong defaults of Cargo will make it very easy to build release versions,
debug our code via <code class="highlighter-rouge">gdb</code> or <code class="highlighter-rouge">lldb</code>, package the application and write benchmarks or unit tests for your C functions.
This is all possible without the need of a cruel Makefile or a bloated CMake environment. How great is that?! 😀</p>

<p>Currently I am working on some <a href="https://github.com/saschagrunert/craft">Cargo port called ‘Craft’</a> which should combine
both worlds out of the box. Replacing the compiler facade is a lots of work and Cargo is even evolving very fast which
makes me think about writing just an cargo extension to do faster prototyping in C. What do you think?</p>

<p>The full source code of the small example project is available <a href="https://github.com/saschagrunert/rapidc">on GitHub</a>.</p>