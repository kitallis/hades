+++
title = "Rocket - A Rust game running on WASM"
date = "2017-12-03T15:14:51+01:00"

[extra]
author = "aochagavia&apos;s blog"
link = "https://aochagavia.github.io/blog/rocket---a-rust-game-running-on-wasm/"
+++
<p>Two weeks ago, Alex Crichton&rsquo;s <a href="https://github.com/rust-lang/rust/pull/45905">PR</a> adding a target for
WebAssembly to the Rust compiler was merged. There are many differences
between this target and the Emscripten one, but the important one for me is that
it doesn&rsquo;t depend on external stuff like the Emscripten SDK (which
IIRC used to be a pain to get working on Windows, but seems to be better
now).</p>

<p>After seeing the examples on <a href="https://www.hellorust.com">hellorust.com</a>, I thought it would
be interesting to try to adapt my game <a href="https://github.com/aochagavia/rocket">Rocket</a> to work
on the browser through the <code>wasm32-unknown-unknown</code> target. The project
was a great way to figure out how far you can go when porting a project that
is a bit more complex than a hello world. I was pleasantly surprised by the
fact that most of the code could be reused. Particularly, the game logic code
was barely touched at all.</p>

<h3 id="tldr">TLDR</h3>

<p>Here is the <a href="https://github.com/aochagavia/rocket_wasm">source code</a>. Also, you can play the game in the canvas
below or <a href="https://aochagavia.github.io/rocket_wasm/">on a dedicated tab</a>.</p>

<p>The controls are:</p>

<ul>
<li>Left and right arrows: turn left and right</li>
<li>Up arrow: boost</li>
<li>Space: shoot</li>
</ul>

<p><canvas id="canvas" height="300px" style="width: 100%;"></canvas>
<script src="js/embedded-rocket.js"></script></p>

<h2 id="an-mvp">An MVP</h2>

<h3 id="getting-things-to-compile-removing-piston">Getting things to compile: removing Piston</h3>

<p>Before I started, the little I knew about WebAssembly was that it doesn&rsquo;t allow
you to interface with the OS, graphics card or other stuff like that. Using
Emscripten seems to be a way around this problem, but I guess you still need
to adapt your programs to some extent&hellip; I have never used it, though, so take
my words with a grain of salt.</p>

<p>After cloning the Rocket repository I started removing stuff. The first thing to
go was the dependency on Piston. I didn&rsquo;t even try to compile Rocket to wasm
before this step, as it is obvious that Piston requires OS support.</p>

<p>At this point, we were left with:</p>

<ol>
<li>No game loop</li>
<li>No rendering</li>
<li>No player input</li>
</ol>

<h3 id="rebuilding-the-game-laying-down-the-basic-structure">Rebuilding the game: laying down the basic structure</h3>

<p>So here we are without even a main function. This means that the game loop should
be implemented in Javascript and call into our Rust functions. Therefore
we need a set of basic functions that are enough to drive the execution of the
game, draw something to the screen and process user input.</p>

<p>Since rendering and processing player input are more involved than just updating
the game state, I chose the latter as a first function to implement. I was able
to reuse the code for the game logic without any change, so the function ended
up looking as follows:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #75715e">#[no_mangle]</span><span style="color: #f8f8f2"></span>
<span style="color: #66d9ef">pub</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> update(time</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double) {</span>
<span style="color: #f8f8f2">  </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> data</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> GameData </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> DATA.lock().unwrap();</span>
<span style="color: #f8f8f2">  data.time_controller.update_seconds(time, </span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">data.actions, </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> data.state);</span>
<span style="color: #f8f8f2">  CollisionsController</span><span style="color: #f92672">::</span><span style="color: #f8f8f2">handle_collisions(</span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> data.state);</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Surprisingly, the <a href="https://github.com/aochagavia/rocket/blob/c13232b074da14662cc24ee075f7ef66521e5d27/src/main.rs#L71-L74">update function on the original game</a> is <em>exactly the same</em>,
with the exception of the use of <code>DATA</code>. By the way, we use <code>DATA</code> to
store state instead of passing it between Javascript and Rust every time we call
a function. The definition is quite simple:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #f8f8f2">lazy_static</span><span style="color: #f92672">!</span><span style="color: #f8f8f2"> {</span>
<span style="color: #f8f8f2">  </span><span style="color: #66d9ef">static</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">ref</span><span style="color: #f8f8f2"> DATA</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> Mutex</span><span style="color: #f92672">&lt;</span><span style="color: #f8f8f2">GameData</span><span style="color: #f92672">&gt;</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> Mutex</span><span style="color: #f92672">::</span><span style="color: #f8f8f2">new(new_game_data(</span><span style="color: #ae81ff">1024.0</span><span style="color: #f8f8f2">, </span><span style="color: #ae81ff">600.0</span><span style="color: #f8f8f2">));</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Since <code>DATA</code> is accessible from anywhere in the program, Rust forces us to use
a <code>Mutex</code> to ensure thread safety. Technically, this isn&rsquo;t necessary in the case
of Javascript, since there will only be one thread. Still, the type system knows
nothing about that&hellip; Hence the mutex.</p>

<h3 id="getting-things-to-compile-take-two">Getting things to compile, take two</h3>

<p>With Piston out of the way, I set out to get the rest of the code to compile and
to run it in the browser as a simulation without any visual output. This is the
moment where difficulties started to pop out.</p>

<p>The first problem I encountered was caused by the dependency on <code>rand</code>. Generating
random numbers doesn&rsquo;t necessarily require OS support, but you need to generate a
seed some way or another. For this reason, <code>rand</code> relies on an <code>OsRng</code> struct that
is platform-dependent. Guess what&hellip; WebAssembly didn&rsquo;t had such a struct, so the
crate could not be compiled.</p>

<p>Fortunately, the problem was easily solved by <a href="https://github.com/rust-lang-nursery/rand/pull/197">adding such a struct</a>.
After patching the crate, the code finally compiled&hellip; but it didn&rsquo;t run in the
browser.</p>

<p>By the way, you are probably wondering about the seeding problem. If there is no way
to communicate with the outside world from your WebAssembly programs, how can you
get a seed? Below I will describe how you can call Javascript functions from Rust,
which could be a solution to the problem. However, I decided to use a constant seed,
which is clearly not optimal, but is good enough for a playable demo.</p>

<h3 id="getting-things-to-run-link-errors">Getting things to run: link errors</h3>

<p>I mentioned in the paragraph above that the resulting program didn&rsquo;t run on the browser.
Concretely, after following the instructions on <a href="https://www.hellorust.com">hellorust.com</a>, I got the
following error:</p>

<pre><code>TypeError: import object field 'env' is not an Object
</code></pre>

<p>After looking around for a while, this turned out to be a linking problem. In other words,
the generated Rust code contained calls to functions that didn&rsquo;t exist. Therefore, the
browser expected me to pass an import object containing said functions. It seems that
some <code>f64</code> functions I used in the physics part of the game have no analogous on
WebAssembly, so I had to pass them explicitly from Javascript through the following object:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">let</span> <span style="color: #a6e22e">imports</span> <span style="color: #f92672">=</span> <span style="color: #f8f8f2">{</span>
  <span style="color: #a6e22e">env</span><span style="color: #f92672">:</span> <span style="color: #f8f8f2">{</span>
    <span style="color: #a6e22e">Math_atan</span><span style="color: #f92672">:</span> <span style="color: #f8f8f2">Math.</span><span style="color: #a6e22e">atan</span><span style="color: #f8f8f2">,</span>
    <span style="color: #a6e22e">sin</span><span style="color: #f92672">:</span> <span style="color: #f8f8f2">Math.</span><span style="color: #a6e22e">sin</span><span style="color: #f8f8f2">,</span>
    <span style="color: #a6e22e">cos</span><span style="color: #f92672">:</span> <span style="color: #f8f8f2">Math.</span><span style="color: #a6e22e">cos</span>
  <span style="color: #f8f8f2">}</span>
<span style="color: #f8f8f2">};</span>
</pre></div>

<p>After this, the code compiled and could be loaded on the browser, though without
any kind of visual feedback. Rust running on the browser! Finally.</p>

<h2 id="making-the-game-actually-playable">Making the game actually playable</h2>

<h3 id="rendering">Rendering</h3>

<p>At this point I discovered that you could call Javascript functions from within the
Rust program. This follows the same principle as using C functions from a library.
On the Rust side, you need to declare the function as <code>extern</code>. On the Javascript
side, you need to add the function to the imports, so it can be linked.</p>

<p>This means we can define drawing functions on the Javascript side and call them from
Rust. Even though WebAssembly itself cannot interact with the outside world, it can
still call Javascript functions you explicitly pass through the <code>imports</code> object.
This will be our escape hatch to render the game to a canvas</p>

<p>Rendering things to the screen was as easy as adding a bunch of functions to my program:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> clear_screen();</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> draw_player(_</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double, _</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double, _</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> draw_enemy(_</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double, _</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> draw_bullet(_</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double, _</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> draw_particle(_</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double, _</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double, _</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double);</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> draw_score(_</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> c_double);</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Of course, these functions had to be implemented on the Javascript side. You can find
them on the source code of the demo. You won&rsquo;t find any surprises there, as the only
thing they do is drawing to a canvas.</p>

<p>With these extern functions in place, I could implement the rest of the drawing code
in Rust as shown below:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #75715e">#[no_mangle]</span><span style="color: #f8f8f2"></span>
<span style="color: #66d9ef">pub</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">unsafe</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> draw() {</span>
<span style="color: #f8f8f2">    </span><span style="color: #f92672">use</span><span style="color: #f8f8f2"> geometry</span><span style="color: #f92672">::</span><span style="color: #f8f8f2">{Advance, Position};</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> data </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> DATA.lock().unwrap();</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> world </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">data.state.world;</span>

<span style="color: #f8f8f2">    clear_screen();</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">for</span><span style="color: #f8f8f2"> particle </span><span style="color: #66d9ef">in</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">world.particles {</span>
<span style="color: #f8f8f2">        draw_particle(particle.x(), particle.y(), </span><span style="color: #ae81ff">5.0</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">*</span><span style="color: #f8f8f2"> particle.ttl);</span>
<span style="color: #f8f8f2">    }</span>

<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">for</span><span style="color: #f8f8f2"> bullet </span><span style="color: #66d9ef">in</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">world.bullets {</span>
<span style="color: #f8f8f2">        draw_bullet(bullet.x(), bullet.y());</span>
<span style="color: #f8f8f2">    }</span>

<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">for</span><span style="color: #f8f8f2"> enemy </span><span style="color: #66d9ef">in</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #f8f8f2">world.enemies {</span>
<span style="color: #f8f8f2">        draw_enemy(enemy.x(), enemy.y());</span>
<span style="color: #f8f8f2">    }</span>

<span style="color: #f8f8f2">    draw_player(world.player.x(), world.player.y(), world.player.direction());</span>
<span style="color: #f8f8f2">    draw_score(data.state.score </span><span style="color: #66d9ef">as</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">f64</span><span style="color: #f8f8f2">);</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>Again, if you compare this code to the <a href="https://github.com/aochagavia/rocket/blob/c13232b074da14662cc24ee075f7ef66521e5d27/src/view.rs#L31-L45">original version</a>,
you will see that they are strikingly similar.</p>

<h3 id="processing-user-input">Processing user input</h3>

<p>With simulation and rendering in place, enabling user input was almost trivial.
First of all, I added a bunch of functions to toggle user actions on and off.
Note that I am using a Rust type as a parameter of each function. This is
technically incorrect, but I am not sure about which type I should use instead.
If you do, please open a PR so it can be fixed.</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #75715e">#[no_mangle]</span><span style="color: #f8f8f2"></span>
<span style="color: #66d9ef">pub</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> toggle_shoot(b</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">bool</span><span style="color: #f8f8f2">) {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> data </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> DATA.lock().unwrap();</span>
<span style="color: #f8f8f2">    data.actions.shoot </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> b;</span>
<span style="color: #f8f8f2">}</span>

<span style="color: #75715e">#[no_mangle]</span><span style="color: #f8f8f2"></span>
<span style="color: #66d9ef">pub</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> toggle_boost(b</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">bool</span><span style="color: #f8f8f2">) {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> data </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> DATA.lock().unwrap();</span>
<span style="color: #f8f8f2">    data.actions.boost </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> b;</span>
<span style="color: #f8f8f2">}</span>

<span style="color: #75715e">#[no_mangle]</span><span style="color: #f8f8f2"></span>
<span style="color: #66d9ef">pub</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> toggle_turn_left(b</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">bool</span><span style="color: #f8f8f2">) {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> data </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> DATA.lock().unwrap();</span>
<span style="color: #f8f8f2">    data.actions.rotate_left </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> b;</span>
<span style="color: #f8f8f2">}</span>

<span style="color: #75715e">#[no_mangle]</span><span style="color: #f8f8f2"></span>
<span style="color: #66d9ef">pub</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">extern</span><span style="color: #f8f8f2"> </span><span style="color: #e6db74">&quot;C&quot;</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">fn</span><span style="color: #f8f8f2"> toggle_turn_right(b</span><span style="color: #f92672">:</span><span style="color: #f8f8f2"> </span><span style="color: #66d9ef">bool</span><span style="color: #f8f8f2">) {</span>
<span style="color: #f8f8f2">    </span><span style="color: #66d9ef">let</span><span style="color: #f8f8f2"> data </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> </span><span style="color: #f92672">&amp;</span><span style="color: #66d9ef">mut</span><span style="color: #f8f8f2"> DATA.lock().unwrap();</span>
<span style="color: #f8f8f2">    data.actions.rotate_right </span><span style="color: #f92672">=</span><span style="color: #f8f8f2"> b;</span>
<span style="color: #f8f8f2">}</span>
</pre></div>

<p>In this case, the code did differ considerably from the <a href="https://github.com/aochagavia/rocket/blob/c13232b074da14662cc24ee075f7ef66521e5d27/src/controllers/input.rs#L39-L47">original version</a>,
since the latter relies on the <code>piston_window::Key</code> struct, which no longer exists. In the wasm
version, I moved the key matching logic to Javascript, since I didn&rsquo;t want to pass strings between
Javascript and Rust. The resulting code is straightforward:</p>
<div class="highlight" style="background: #272822"><pre style="line-height: 125%"><span></span><span style="color: #75715e">// Input processing</span>
<span style="color: #66d9ef">function</span> <span style="color: #a6e22e">processKey</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">key</span><span style="color: #f8f8f2">,</span> <span style="color: #a6e22e">b</span><span style="color: #f8f8f2">)</span> <span style="color: #f8f8f2">{</span>
  <span style="color: #66d9ef">switch</span> <span style="color: #f8f8f2">(</span><span style="color: #a6e22e">key</span><span style="color: #f8f8f2">)</span> <span style="color: #f8f8f2">{</span>
    <span style="color: #66d9ef">case</span> <span style="color: #e6db74">&quot;ArrowLeft&quot;</span><span style="color: #f92672">:</span>
      <span style="color: #a6e22e">module</span><span style="color: #f8f8f2">.</span><span style="color: #a6e22e">toggle_turn_left</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">b</span><span style="color: #f8f8f2">);</span>
      <span style="color: #66d9ef">break</span><span style="color: #f8f8f2">;</span>
    <span style="color: #66d9ef">case</span> <span style="color: #e6db74">&quot;ArrowRight&quot;</span><span style="color: #f92672">:</span>
      <span style="color: #a6e22e">module</span><span style="color: #f8f8f2">.</span><span style="color: #a6e22e">toggle_turn_right</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">b</span><span style="color: #f8f8f2">);</span>
      <span style="color: #66d9ef">break</span><span style="color: #f8f8f2">;</span>
    <span style="color: #66d9ef">case</span> <span style="color: #e6db74">&quot;ArrowUp&quot;</span><span style="color: #f92672">:</span>
      <span style="color: #a6e22e">module</span><span style="color: #f8f8f2">.</span><span style="color: #a6e22e">toggle_boost</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">b</span><span style="color: #f8f8f2">);</span>
      <span style="color: #66d9ef">break</span><span style="color: #f8f8f2">;</span>
    <span style="color: #66d9ef">case</span> <span style="color: #e6db74">&quot; &quot;</span><span style="color: #f92672">:</span>
      <span style="color: #a6e22e">module</span><span style="color: #f8f8f2">.</span><span style="color: #a6e22e">toggle_shoot</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">b</span><span style="color: #f8f8f2">);</span>
      <span style="color: #66d9ef">break</span><span style="color: #f8f8f2">;</span>
  <span style="color: #f8f8f2">}</span>
<span style="color: #f8f8f2">}</span>
<span style="color: #f8f8f2">document.</span><span style="color: #a6e22e">addEventListener</span><span style="color: #f8f8f2">(</span><span style="color: #e6db74">&#39;keydown&#39;</span><span style="color: #f8f8f2">,</span> <span style="color: #a6e22e">e</span> <span style="color: #f92672">=&gt;</span> <span style="color: #a6e22e">processKey</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">e</span><span style="color: #f8f8f2">.</span><span style="color: #a6e22e">key</span><span style="color: #f8f8f2">,</span> <span style="color: #66d9ef">true</span><span style="color: #f8f8f2">));</span>
<span style="color: #f8f8f2">document.</span><span style="color: #a6e22e">addEventListener</span><span style="color: #f8f8f2">(</span><span style="color: #e6db74">&#39;keyup&#39;</span><span style="color: #f8f8f2">,</span> <span style="color: #a6e22e">e</span> <span style="color: #f92672">=&gt;</span> <span style="color: #a6e22e">processKey</span><span style="color: #f8f8f2">(</span><span style="color: #a6e22e">e</span><span style="color: #f8f8f2">.</span><span style="color: #a6e22e">key</span><span style="color: #f8f8f2">,</span> <span style="color: #66d9ef">false</span><span style="color: #f8f8f2">));</span>
</pre></div>

<h1 id="conclusion">Conclusion</h1>

<p>Even though the <code>wasm32-unknown-unknown</code> target is quite new, it clearly has a
lot of potential. I am impressed by the fact that I was able to port Rocket
with almost no modifications to the game logic code. In the end, I ended up
spending most of the time dealing with rendering and figuring out how to correctly
set up the integration between Javascript and Rust.</p>

<h1 id="discussion">Discussion</h1>

<p>Comment on <a href="https://www.reddit.com/r/rust/comments/7ha3gj/rocket_a_rust_game_running_on_wasm/">reddit</a> or <a href="https://news.ycombinator.com/item?id=15843064">HN</a>!</p>