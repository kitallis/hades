+++
title = "A comparison of regex engines"
date = "2017-03-28T00:00:00+00:00"

[extra]
author = "Rust Leipzig"
link = "https://rust-leipzig.github.io/regex/2017/03/28/comparison-of-regex-engines/"
+++
<script src="https://cdnjs.cloudflare.com/ajax/libs/Chart.js/2.5.0/Chart.bundle.js"></script>

<h1 id="introduction">Introduction</h1>
<p>Regular expressions (or just regex) are commonly used in pattern search algorithms. There are many different
regex engines available with different support of expressions, performance constraints and language bindings.
Based on the previous work of John Maddock (See his own <a href="http://www.boost.org/doc/libs/1_41_0/libs/regex/doc/gcc-performance.html">regex comparison</a>)
and the sljit project (See their <a href="http://sljit.sourceforge.net/regex_perf.html">regex comparison</a>)
I want to give an overview of actively developed engines regarding their performance.</p>

<h1 id="test-setup">Test setup</h1>
<h2 id="hardware">Hardware</h2>
<p>The performance is measured only on my Dell notebook. It’s not the newest one, but it doesn’t matter because
I used the same hardware for all engines and I’m interested in the performance results compared to each other.
But here are some hardware information:</p>
<ul>
  <li>Chassis: Dell Latitude E7450</li>
  <li>CPU: Intel® Core™ i5-5300U</li>
  <li>RAM: 16GB</li>
  <li>SSD: Samsung PM85 256GB</li>
</ul>

<h2 id="software">Software</h2>
<p>The used software is also not the latest, but newer than the default packages of my installed Ubuntu 16.04:</p>
<ul>
  <li>GCC 6.2.0</li>
  <li>Rustc 1.16.0 and 1.17.0-nightly</li>
</ul>

<p>I want to get the execution time to find a match of each engine for the following regular expression:</p>
<ul>
  <li><code class="highlighter-rouge">Twain</code></li>
  <li><code class="highlighter-rouge">(?i)Twain</code></li>
  <li><code class="highlighter-rouge">[a-z]shing</code></li>
  <li><code class="highlighter-rouge">Huck[a-zA-Z]+|Saw[a-zA-Z]+</code></li>
  <li><code class="highlighter-rouge">\b\w+nn\b</code></li>
  <li><code class="highlighter-rouge">[a-q][^u-z]{13}x</code></li>
  <li><code class="highlighter-rouge">Tom|Sawyer|Huckleberry|Finn</code></li>
  <li><code class="highlighter-rouge">(?i)Tom|Sawyer|Huckleberry|Finn</code></li>
  <li><code class="highlighter-rouge">.{0,2}(Tom|Sawyer|Huckleberry|Finn)</code></li>
  <li><code class="highlighter-rouge">.{2,4}(Tom|Sawyer|Huckleberry|Finn)</code></li>
  <li><code class="highlighter-rouge">Tom.{10,25}river|river.{10,25}Tom</code></li>
  <li><code class="highlighter-rouge">[a-zA-Z]+ing</code></li>
  <li><code class="highlighter-rouge">\s[a-zA-Z]{0,12}ing\s</code></li>
  <li><code class="highlighter-rouge">([A-Za-z]awyer|[A-Za-z]inn)\s</code></li>
  <li><code class="highlighter-rouge">["'][^"']{0,30}[?!\.][\"']</code></li>
  <li><code class="highlighter-rouge">\u221E|\u2713</code></li>
  <li><code class="highlighter-rouge">\p{Sm}</code></li>
</ul>

<p>The set of expressions is not representative, but is sufficient to get an overview.</p>

<p>To measure the performance I modified the existing benchmark tool of the sljit project.
The tool is available on github: <a href="https://github.com/rust-leipzig/regex-performance">regex-performance</a>.
The base tool of the sljit project supported already the following regex engines:</p>
<ul>
  <li><a href="https://github.com/kkos/oniguruma">Oniguruma</a>, v6.1.3</li>
  <li><a href="https://github.com/google/re2">RE2</a></li>
  <li><a href="https://github.com/laurikari/tre">Tre</a></li>
  <li><a href="http://www.pcre.oashrg">PCRE2</a>, v10.23</li>
</ul>

<p>I added support for two more engines:</p>
<ul>
  <li><a href="https://github.com/01org/hyperscan">Hyperscan</a>, v4.4.1</li>
  <li><a href="https://doc.rust-lang.org/regex/regex/index.html">Rust regex crate</a>, v0.2.1</li>
</ul>

<h3 id="pcre2">PCRE2</h3>
<blockquote>
  <p>Perl Compatible Regular Expressions (PCRE) is a regular expression C library inspired by the regular expression
capabilities in the Perl programming language. PCRE2 is the name used for a revised API for the PCRE library.</p>
</blockquote>

<p>Beside the standard matching algorithm PCRE2 is shipped with an alternative algorithm based on a
deterministic finite automate (<em>DFA</em>) which operates in a different way and is not Perl-compatible.
A detailed description is provided within the <a href="http://www.pcre.org/current/doc/html/pcre2matching.html">man pages</a>.</p>

<p>A heavyweight optimized variant is shipped too: the just-in-time (<em>JIT</em>) compiling can greatly speed up pattern matching.</p>

<p>To get comparable results, the Unicode support has to be enabled with the configuration option <code class="highlighter-rouge">--enable-unicode</code>.
The JIT feature is optional and has to be enabled with the option <code class="highlighter-rouge">--enable-jit</code>.</p>

<h3 id="hyperscan">Hyperscan</h3>
<p>Hyperscan is a <a href="https://01.org/hyperscan">01.org</a> open source project:</p>
<blockquote>
  <p>Hyperscan is a high-performance multiple regex matching library.
It follows the regular expression syntax of the commonly-used libpcre library, yet functions as a standalone library
with its own API written in C. Hyperscan uses hybrid automata techniques to allow simultaneous matching of large numbers 
of regular expressions, as well as matching of regular expressions across streams of data.</p>
</blockquote>

<p>Hyperscan is a mature library with over 10 years of development. The focus of Hyperscan is the x86 platform and the library
uses hardware accelerators such as AVX to optimize the throughput.</p>

<p>By default, Hyperscan does not care about the start of a match. Getting the start of a match requires the set flag
<code class="highlighter-rouge">HS_FLAG_SOM_LEFTMOST</code> for compiling a pattern. The flag costs some performance but is required to get
comparable results.</p>

<h3 id="rust-regex-crate">Rust regex crate</h3>
<p>A Rust crate is a synonymous for a ‘library’ or ‘package’. The <a href="https://doc.rust-lang.org/regex/regex/index.html">Rust regex crate</a>
provides functions for parsing, compiling, and executing regular expressions:</p>
<blockquote>
  <p>Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and back-references.
In exchange, all searches execute in linear time with respect to the size of the regular expression and search text.</p>
</blockquote>

<p>Except the Rust crate all engines are written in C or C++ including the test tool. It was a requirement to have
C-binding of the used engine. So an interface was necessary to call the Rust functions within C.
The solution uses the Rusts FFI (foreign function interface) to build a static
library which just counts the matches for a given expression. The complete library consists of 3 functions with less
than 50 lines of code. The main Rust function to get the matches is:</p>
<div class="language-rust highlighter-rouge"><pre class="highlight"><code><span class="err">#</span><span class="p">[</span><span class="n">no_mangle</span><span class="p">]</span>
<span class="k">pub</span> <span class="k">extern</span> <span class="k">fn</span> <span class="nf">regex_matches</span><span class="p">(</span><span class="n">raw_exp</span><span class="p">:</span> <span class="o">*</span><span class="k">mut</span> <span class="n">Regex</span><span class="p">,</span> <span class="n">p</span><span class="p">:</span> <span class="o">*</span><span class="k">const</span> <span class="nb">u8</span><span class="p">,</span> <span class="n">len</span><span class="p">:</span> <span class="nb">u64</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">u64</span> <span class="p">{</span>
    <span class="k">let</span> <span class="n">exp</span> <span class="o">=</span> <span class="k">unsafe</span> <span class="p">{</span> <span class="nn">Box</span><span class="p">::</span><span class="nf">from_raw</span><span class="p">(</span><span class="n">raw_exp</span><span class="p">)</span> <span class="p">};</span>
    <span class="k">let</span> <span class="n">s</span> <span class="o">=</span> <span class="k">unsafe</span> <span class="p">{</span> <span class="nn">slice</span><span class="p">::</span><span class="nf">from_raw_parts</span><span class="p">(</span><span class="n">p</span><span class="p">,</span> <span class="n">len</span> <span class="k">as</span> <span class="nb">usize</span><span class="p">)</span> <span class="p">};</span>

    <span class="k">let</span> <span class="n">findings</span> <span class="o">=</span> <span class="n">exp</span><span class="nf">.find_iter</span><span class="p">(</span><span class="n">s</span><span class="p">)</span><span class="nf">.count</span><span class="p">();</span>
    <span class="nn">Box</span><span class="p">::</span><span class="nf">into_raw</span><span class="p">(</span><span class="n">exp</span><span class="p">);</span>
    <span class="n">findings</span> <span class="k">as</span> <span class="nb">u64</span>
<span class="p">}</span>
</code></pre>
</div>
<p>The function gets a raw pointer to a previously compiled expression (<code class="highlighter-rouge">raw_exp</code>), a raw pointer to the input c-string
(<code class="highlighter-rouge">p</code>) and the length of the input string (<code class="highlighter-rouge">len</code>). At the first the function gets the compiled expression from the
corresponding raw pointer and the input string. The converting from a raw pointer into a type is unsafe, that’s why
the code parts have to be wrapped with <code class="highlighter-rouge">unsafe {}</code>. After all the number of matches is gathered with a call of
<code class="highlighter-rouge">exp.find_iter(s).count()</code>. To use the compiled expression in following function calls, the raw
pointer from the expression is gathered again. This effects that the lifetime of the expression still remains after returning.
At the end the function returns the number of matches as a 64bit value to the caller.</p>

<p>The corresping C function prototype is:</p>
<div class="language-c highlighter-rouge"><pre class="highlight"><code><span class="k">struct</span> <span class="n">Regex</span><span class="p">;</span>       <span class="c1">// anonymous declaration
</span>
<span class="k">extern</span> <span class="kt">uint64_t</span> <span class="n">regex_matches</span><span class="p">(</span><span class="k">struct</span> <span class="n">Regex</span> <span class="k">const</span> <span class="o">*</span> <span class="k">const</span> <span class="n">exp</span><span class="p">,</span> <span class="kt">uint8_t</span> <span class="o">*</span> <span class="k">const</span> <span class="n">str</span><span class="p">,</span> <span class="kt">uint64_t</span> <span class="n">str_len</span><span class="p">);</span>
</code></pre>
</div>

<h1 id="results">Results</h1>
<p>To get the test results of the test tool call within the build directory of the tool:</p>
<div class="language-bash highlighter-rouge"><pre class="highlight"><code>./src/regex_perf -f ../3200.txt -o results.csv
</code></pre>
</div>
<p>The tool prints the details and writes the results per test and engine to <code class="highlighter-rouge">results.csv</code>.
At the end a short summary of the results is printed:</p>
<div class="highlighter-rouge"><pre class="highlight"><code>Total Results:
[      pcre] time:  12626.7 ms, score:      8 points,
[  pcre-dfa] time:  14135.2 ms, score:      0 points,
[  pcre-jit] time:   1050.6 ms, score:     47 points,
[       re2] time:    946.1 ms, score:     26 points,
[      onig] time:   2475.8 ms, score:      4 points,
[       tre] time:  10508.4 ms, score:      0 points,
[     hscan] time:    299.7 ms, score:     72 points,
[rust_regex] time:   3681.5 ms, score:     47 points,
</code></pre>
</div>

<h2 id="timings">Timings</h2>
<p>On the basis of a CSV-file I do some analytics. At first I summarize the overall execution time per engine.
The following chart shows the details:</p>

<div><canvas id="timing_chart" height="400"></canvas></div>
<script>
    var ctx = document.getElementById("timing_chart");
    var myChart = new Chart(ctx, {
        type: 'horizontalBar',
        data: {
            labels: ["pcre-dfa", "pcre", "tre", "rust_regex", "onig", "pcre-jit", "re2", "hscan"],
            datasets: [{
                label: 'Timing',
                data: [14135.1, 12626.6, 10508.5, 3681.5, 2475.9, 1050.6, 946.0, 299.5],
                backgroundColor: [
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(54, 162, 235, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)'
                ],
                borderColor: [
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(54, 162, 235, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)'
                ],
                borderWidth: 1
            }]
        },
        options: {
            maintainAspectRatio: false,
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }],
                xAxes: [{
                    scaleLabel: {
                        display: true,
                        labelString: 'ms'
                    }
                }]
            }
        }
    });
</script>

<p>Hyperscan is the fastest engine with a total execution time of ~300ms (~3x less time than 2nd) and the Rust regex crate
gets the 5th place with ~3700ms. It seems that the Rust regex crate is not the fastest solution in place.</p>

<p>But: what happens if one expression is really slow? This test distorts the overall result of the engine.
Therefor I implement a simple result scoring. For each test the fastest engine can earn 5 points, the 2nd 4 points
and so on. This limits the impact of a single slow expression. The following chart shows the score points per engine:</p>

<div><canvas id="score_chart" height="400"></canvas></div>
<script>
    var ctx = document.getElementById("score_chart");
    var myChart = new Chart(ctx, {
        type: 'horizontalBar',
        data: {
            labels: ["tre", "pcre-dfa", "onig", "pcre", "re2", "rust_regex", "pcre-jit", "hscan"],
            datasets: [{
                label: 'Score',
                data: [0, 0, 4, 8, 26, 47, 47, 72],
                backgroundColor: [
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(54, 162, 235, 0.2)',
                    'rgba(64, 64, 64, 0.2)',
                    'rgba(64, 64, 64, 0.2)'
                ],
                borderColor: [
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(54, 162, 235, 1)',
                    'rgba(64, 64, 64, 1)',
                    'rgba(64, 64, 64, 1)'
                ],
                borderWidth: 1
            }]
        },
        options: {
            maintainAspectRatio: false,
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }],
                xAxes: [{
                    scaleLabel: {
                        display: true,
                        labelString: 'points'
                    }
                }]
            }
        }
    });
</script>

<p>Hyperscan is still the number one, but the Rust regex crate gets the 2nd place together with PCRE2-JIT.
The results look better than the absolute timings, but it seems that one or more expressions are slow in their execution time.</p>

<p>So it’s time to take a look into the results for each expression. The following chart compares the average timings
of all engines per expression against the measured value of the Rust regex crate.
The secondary y-axis displays the ratio of the Rust value and the average value in percentage.</p>

<div><canvas id="rust_chart" height="600"></canvas></div>
<script>
    var ctx = document.getElementById("rust_chart");
    var myChart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: [
                "Twain", "(?i)Twain", "[a-z]shing", "Huck[a-zA-Z]+|Saw[a-zA-Z]+",
                "\b\w+nn\b", "[a-q][^u-z]{13}x", "Tom|Sawyer|Huckleberry|Finn", "(?i)Tom|Sawyer|Huckleberry|Finn",
                ".{0,2}(Tom|Sawyer|Huckleberry|Finn)",".{2,4}(Tom|Sawyer|Huckleberry|Finn)",
                "Tom.{10,25}river|river.{10,25}Tom", "[a-zA-Z]+ing", "\s[a-zA-Z]{0,12}ing\s",
                "([A-Za-z]awyer|[A-Za-z]inn)\s", "[\"'][^\"']{0,30}[?!\.][\"']", "∞|✓", "\p{Sm}"
            ],
            datasets: [
                {
                    type: 'bar',
                    label: 'AVG',
                    yAxisID: 'timing',
                    data: [39.2, 90.4, 219.4, 69.3, 424.9, 833.5, 95.0, 253.4, 1129.5, 1252.4, 81.5, 427.0, 213.9, 334.9, 73.2, 45.2, 132.9],
                    backgroundColor: 'rgba(64, 64, 64, 0.2)',
                    borderColor: 'rgba(64, 64, 64, 1)',
                    borderWidth: 1
                },
                {
                    type: 'bar',
                    label: 'Rust',
                    yAxisID: 'timing',
                    data: [2.2, 41.9, 7.0, 5.5, 193.7, 3077.9, 39.9, 40.2, 34.2, 34.4, 28.6, 15.8, 40.5, 34.2, 12.5, 39.1, 33.9],
                    backgroundColor: 'rgba(54, 162, 235, 0.2)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                },
                {
                    type: 'line',
                    label: 'Rust/AVG',
                    yAxisID: 'ratio',
                    data: [5.6, 46.3, 3.2, 7.9, 45.6, 369.3, 42.0, 15.9, 3.0, 2.7, 35.1, 3.7, 18.9, 10.2, 17.1, 86.6, 25.5],
                    borderColor: 'rgba(255,99,132,1)',
                    backgroundColor: 'white',
                    borderWidth: 2,
                    fill: false
                }
            ]
        },
        options: {
            maintainAspectRatio: false,
            scales: {
                yAxes: [
                    {
                        id: 'timing',
                        scaleLabel: {
                            display: true,
                            labelString: 'ms'
                        },
                        type: 'logarithmic',
                        position: 'left',
                        ticks: {
                            beginAtZero:true
                        }
                    }, {
                        id: 'ratio',
                        scaleLabel: {
                            display: true,
                            labelString: '%'
                        },
                        type: 'linear',
                        position: 'right',
                        ticks: {
                            beginAtZero:true
                        }
                    }
                ]
            }
        }
    });
</script>

<p>The red curve has 3 major peeks of expressions for which the regex crate does not perform well.
The expressions are:</p>
<ol>
  <li><code class="highlighter-rouge">[a-q][^u-z]{13}x</code></li>
  <li><code class="highlighter-rouge">∞|✓</code></li>
  <li><code class="highlighter-rouge">(?i)Twain</code></li>
</ol>

<p>Especially the execution of the first of those three expression is done very slow.</p>

<h2 id="improvements">Improvements</h2>
<p>With the first results of the benchmark I opened the ticket <a href="https://github.com/rust-lang/regex/issues/350">rust-lang/regex/350</a>
to get feedback regarding my findings. Andrew Gallant alias <a href="https://github.com/BurntSushi">BurntSushi</a>
gave me great feedback with some improvement proposals.</p>

<p>One improvement is to use the SIMD feature of the regex crate. This feature is currently available in the Rust nightly
built, so I have to install the nightly toolchain too. I adjust the projects cmake scripts to detect whether a nightly
compiler is used and the SIMD feature is supported. So the rust toolchain can be switched with
<code class="highlighter-rouge">rustup default nightly-x86_64-unknown-linux-gnu</code> and the tool has to be reconfigured and build again to get
the new results.</p>

<div><canvas id="rust_simd_chart" height="600"></canvas></div>
<script>
    var ctx = document.getElementById("rust_simd_chart");
    var myChart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: [
                "Twain", "(?i)Twain", "[a-z]shing", "Huck[a-zA-Z]+|Saw[a-zA-Z]+",
                "\b\w+nn\b", "[a-q][^u-z]{13}x", "Tom|Sawyer|Huckleberry|Finn", "(?i)Tom|Sawyer|Huckleberry|Finn",
                ".{0,2}(Tom|Sawyer|Huckleberry|Finn)",".{2,4}(Tom|Sawyer|Huckleberry|Finn)",
                "Tom.{10,25}river|river.{10,25}Tom", "[a-zA-Z]+ing", "\s[a-zA-Z]{0,12}ing\s",
                "([A-Za-z]awyer|[A-Za-z]inn)\s", "[\"'][^\"']{0,30}[?!\.][\"']", "∞|✓", "\p{Sm}"
            ],
            datasets: [
                {
                    type: 'bar',
                    label: 'stable',
                    yAxisID: 'timing',
                    data: [2.2, 41.9, 7, 5.5, 193.7, 3077.9, 39.9, 40.2, 34.2, 34.4, 28.6, 15.8, 40.5, 34.2, 12.5, 39.1, 33.9],
                    backgroundColor: 'rgba(64, 64, 64, 0.2)',
                    borderColor: 'rgba(64, 64, 64, 1)',
                    borderWidth: 1
                },
                {
                    type: 'bar',
                    label: 'nightly',
                    yAxisID: 'timing',
                    data: [2, 3.7, 5.9, 3.9, 179.9, 3120.6, 4.3, 40.3, 35.2, 34.7, 5.2, 16.8, 41.8, 34.4, 11.6, 3.6, 34.8],
                    backgroundColor: 'rgba(54, 162, 235, 0.2)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                },
                {
                    type: 'line',
                    label: 'nightly/stable',
                    yAxisID: 'ratio',
                    data: [90.91, 8.83, 84.29, 70.91, 92.88, 101.39, 10.78, 100.25, 102.92, 100.87, 18.18, 106.33, 103.21, 100.58, 92.80, 9.21, 102.65],
                    borderColor: 'rgba(255,99,132,1)',
                    backgroundColor: 'white',
                    borderWidth: 2,
                    fill: false
                }
            ]
        },
        options: {
            maintainAspectRatio: false,
            scales: {
                yAxes: [
                    {
                        id: 'timing',
                        scaleLabel: {
                            display: true,
                            labelString: 'ms'
                        },
                        type: 'logarithmic',
                        position: 'left',
                        ticks: {
                            beginAtZero:true
                        }
                    }, {
                        id: 'ratio',
                        scaleLabel: {
                            display: true,
                            labelString: '%'
                        },
                        type: 'linear',
                        position: 'right',
                        ticks: {
                            beginAtZero:true
                        }
                    }
                ]
            }
        }
    });
</script>

<p>The chart shows that the expressions <code class="highlighter-rouge">∞|✓</code> and <code class="highlighter-rouge">(?i)Twain</code> benefit by using the SIMD feature,
but not the expression <code class="highlighter-rouge">[a-q][^u-z]{13}x</code>. This expression requires backtracking.
The Rust regex crates uses a finite state machine based algorithm, which lacks for back-references and backtracking.</p>

<h2 id="matches">Matches</h2>
<p>Regarding the found matches I found some deviations. At first, the libraries oniguruma and tre do not support
Unicode category expressions like <code class="highlighter-rouge">\p{Sm}</code>. This expression matches all mathematical symbols like <code class="highlighter-rouge">=</code> or <code class="highlighter-rouge">|</code>.
The Rust regex crate matches additionally the symbol <code class="highlighter-rouge">∞</code>.</p>

<p>Hyperscan returns more matches than other engines, e.g. 977 for the expression <code class="highlighter-rouge">Huck[a-zA-Z]+|Saw[a-zA-Z]+</code> whereas
all other engines are finding 262 matches. Hyperscan reports all matches. The expression <code class="highlighter-rouge">Saw[a-zA-Z]+</code> returns the
following matches for input <code class="highlighter-rouge">Sawyer</code>:</p>
<ul>
  <li>Sawy</li>
  <li>Sawye</li>
  <li>Sawyer</li>
</ul>

<p>All other engines report just one match: Sawy (non-greedy semantics) or Sawyer (greedy semantics).</p>

<h1 id="conclusion">Conclusion</h1>
<p>The Rust regex crate is now something about 2 years old, but tends to edge mature engines like PCRE2 and
Hyperscan. Depending on the used expressions the Rust regex crate is a good choice for pattern matchings.
Thanks to all contributors of the regex crate for their awesome work.</p>

<h1 id="related-work">Related work</h1>
<p>The regex crate contains it’s own benchmark harness with a lot expressions and support for:</p>
<ul>
  <li>PCRE</li>
  <li>PCRE2</li>
  <li>RE2</li>
  <li>Oniguruma</li>
  <li>TCL</li>
</ul>

<p>This benchmark can be used to get another view on the performance of the engines.
Please see the <a href="https://github.com/rust-lang/regex/tree/master/bench">bench subdirectory of the crates repository</a>.</p>