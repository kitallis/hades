+++
title = "LaTeX Tips"
date = "2017-03-08T23:31:54-05:00"

[extra]
author = "Rust — Jim Turner"
link = "https://jim.turner.link/pages/latex-tips/"
+++
<p>LaTeX is a great system for typesetting reports, articles, letters, and even
posters and presentations, but it takes a while to find a set of useful
packages and tune it to your personal style. This page is a collection of
useful tips, tricks, and advice that I’ve learned while working with LaTeX. I’m
assuming that you’re familiar with the basics of LaTeX; if you’re not, I
recommend starting with the <a href="https://en.wikibooks.org/wiki/LaTeX">LaTeX Wikibook</a>.</p>

<p>All LaTeX output images on this page link to the LaTeX source used to generate
them.</p>

<h2 id="getting-help">Getting Help</h2>

<p>For common problems, doing a simple web search is often the quickest way to
find a solution. For problems related to a specific package, the documentation
of most packages is very comprehensive and provides usage instructions and
solutions to common problems (especially conflicts with other packages).
These are some other good resources:</p>

<ul>
<li>The <a href="http://www.tex.ac.uk/index.html">UK list of TeX FAQs</a> is a great list of
common questions. For troubleshooting, see especially the sections <a href="http://www.tex.ac.uk/index.html#ThingsareGoingWronghellip">“Things
are Going Wrong…”</a>, <a href="http://www.tex.ac.uk/index.html#Whydoesitemdoemthat">“Why does it
<em>do</em> that?”</a>, and <a href="http://www.tex.ac.uk/index.html#ThejoyofTeXerrors">“The
joy of TeX errors”</a>.</li>
<li>The <a href="https://en.wikibooks.org/wiki/LaTeX">LaTeX Wikibook</a> has nicely-organized pages and lots of examples for
common topics.</li>
<li>The <a href="https://tex.stackexchange.com/">LaTeX Stack Exchange</a> is a good place to
ask questions or search questions others have asked.</li>
<li><a href="https://martin-ueding.de/posts/common-latex-mistakes/">Martin Ueding has some additional tips</a>, especially how to
avoid common mistakes.</li>
</ul>

<h2 id="project-structure">Project Structure</h2>

<p>For small to mid-size documents, I recommend the following file/directory
structure:</p>

<pre><code>project/
├── .git/
├── .gitignore
├── fig/
│   ├── tikz_figure.tex
│   ├── svg_figure.svg
│   └── …
├── Makefile
├── mystyle.sty
├── document.tex
└── …
</code></pre>

<p>Notice, in particular, that figures are confined to their own directory,
<code>fig/</code>. Also, the style of the document (i.e. what would normally go in the
preamble) is in a separate <code>mystyle.sty</code> file. This keeps from cluttering up
the <code>.tex</code> document and makes it easy to copy your standard style between
projects.</p>

<h2 id="reference-management">Reference Management</h2>

<p>I recommend <a href="http://www.jabref.org/">JabRef</a> for reference management because
it’s easy to use, is quite comprehensive, and has nice features like group
management, advanced searching, linking to downloaded full-text articles (which
is really convenient!), integrating with external tools, etc. It is compatible
with both <a href="http://www.bibtex.org/">BibTeX</a> and <a href="https://ctan.org/pkg/biblatex">BibLaTeX</a>.</p>

<p>It’s especially convenient to <a href="https://tex.stackexchange.com/questions/79251/improving-a-workflow-for-importing-bibtex-citations/112236#112236">set up automatic import of <code>.bib</code> files from
your web browser</a>.
Then, you can click on the “download citation” link available on most journal
article pages and directly import the citation information into JabRef.</p>

<h2 id="compilation-and-synctex">Compilation and SyncTeX</h2>

<p>I typically use <a href="https://www.gnu.org/software/emacs/">GNU Emacs</a> with <a href="https://www.gnu.org/software/auctex/">AUCTeX</a> for compilation, but if you use
a <a href="https://www.gnu.org/software/make/manual/make.html">Makefile</a>, then <a href="http://www.ctan.org/pkg/latexmk">latexmk</a> makes it easy to run all of the appropriate
commands to compile the document.</p>

<p>It’s possible to synchronize <a href="https://www.gnu.org/software/auctex/">AUCTeX</a> and the <a href="https://okular.kde.org/">Okular</a> document viewer such
that when you type <code>C-c C-v</code> (<code>TeX-view</code>) in Emacs, Okular will open the
document centered at the cursor location, and if you Shift-click in Okular,
Emacs will open the LaTeX source file with the cursor at the corresponding
location. To set up the Emacs → Okular link, assuming AUCTeX is already
installed and configured, add the following to your <code>.emacs</code>:</p>
<div class="highlight"><pre><code class="language-elisp" data-lang="elisp"><span></span><span class="c1">;; Enable SyncTeX correlation.</span>
<span class="p">(</span><span class="k">setq</span> <span class="nv">TeX-source-correlate-mode</span> <span class="no">t</span><span class="p">)</span>
<span class="p">(</span><span class="k">setq</span> <span class="nv">TeX-source-correlate-start-server</span> <span class="no">t</span><span class="p">)</span>
<span class="p">(</span><span class="k">setq</span> <span class="nv">TeX-source-correlate-method</span> <span class="ss">&#39;synctex</span><span class="p">)</span>

<span class="c1">;; Use Okular as the PDF viewer.</span>
<span class="p">(</span><span class="k">setq</span> <span class="nv">TeX-view-program-selection</span> <span class="o">&#39;</span><span class="p">((</span><span class="nv">output-pdf</span> <span class="s">&quot;PDF Viewer&quot;</span><span class="p">)))</span>
<span class="c1">;; Define %(dir) expansion for use in `TeX-view-program-list&#39; because the expanded</span>
<span class="c1">;; path needs to match exactly the path listed in the *.synctex.gz file.</span>
<span class="p">(</span><span class="k">setq</span> <span class="nv">TeX-expand-list</span> <span class="o">&#39;</span><span class="p">((</span><span class="s">&quot;%(dir)&quot;</span> <span class="p">(</span><span class="nb">lambda</span> <span class="p">(</span><span class="nv">arg</span><span class="p">)</span> <span class="nv">default-directory</span><span class="p">)</span> <span class="no">nil</span><span class="p">)))</span>
<span class="p">(</span><span class="k">setq</span> <span class="nv">TeX-view-program-list</span> <span class="o">&#39;</span><span class="p">((</span><span class="s">&quot;PDF Viewer&quot;</span> <span class="p">(</span><span class="s">&quot;okular --unique file://%(dir)%o#src:%n%(dir)./%b&quot;</span><span class="p">))))</span>
</code></pre></div>

<p>To set up the Okular → Emacs link, open Okular&rsquo;s configuration dialog with
<code>Settings</code> → <code>Configure Okular…</code>, click on the <code>Editor</code> tab, and then select
<code>Emacs client</code> as your <code>Editor</code>. The corresponding <code>Command</code> is:</p>
<div class="highlight"><pre><code class="language-shell" data-lang="shell"><span></span>emacsclient -a emacs --no-wait +%l %f
</code></pre></div>

<p>If you’re using <a href="http://www.ctan.org/pkg/latexmk">latexmk</a> to build your document, make sure you tell your
LaTeX engine to generate the SyncTeX file. For example, if you’re using
a <a href="https://www.gnu.org/software/make/manual/make.html">Makefile</a> with <a href="http://www.ctan.org/pkg/latexmk">latexmk</a> and the <a href="https://www.tug.org/xetex/">XeLaTeX</a> engine, you can use the
following rule (note the <code>-synctex=1</code> option):</p>
<div class="highlight"><pre><code class="language-makefile" data-lang="makefile"><span></span><span class="nf">%.pdf</span><span class="o">:</span> %.<span class="n">tex</span>
	latexmk -pdf -pdflatex<span class="o">=</span><span class="s2">&quot;xelatex -synctex=1 %O %S&quot;</span> <span class="s2">&quot;</span>$<span class="s2">&lt;&quot;</span>
</code></pre></div>

<h2 id="computation-results">Computation Results</h2>

<p>Martin Ueding developed a great technique to automatically <a href="https://martin-ueding.de/posts/computation-results-in-latex/">insert results from
Python computations into LaTeX documents</a>. Basically, you
generate the results with your script, (optionally) save the results to a file
to cache them, use <a href="http://jinja.pocoo.org/">Jinja</a> to insert the results into
your LaTeX template, and then compile the resulting LaTeX source file.</p>

<h2 id="managing-floats">Managing Floats</h2>

<p>In LaTeX, some environments, such as figures and tables, are floating. This
means that LaTeX will position them in what it thinks is the optimal location
based on some heuristics. Unfortunately, these heuristics aren’t always ideal.</p>

<p>Often, it is useful to force LaTeX to place a float immediately, such as before
starting a new section. The <code>\clearpage</code> command forces LaTeX to place any
remaining floats and starts a new page. I like using <code>\clearpage</code> before each
major section of my document. If you’re tight on space, the <a href="http://ctan.org/pkg/placeins"><code>placeins</code></a>
package provides a <code>\FloatBarrier</code> command that places the remaining floats
without starting a new page.</p>

<p>LaTeX’s defaults can be overly restrictive, refusing to place many floats on a
page or leaving large amounts of white space below floats. This seems to be
particularly common in two-column layouts. In this case, some of the following
settings may be useful for you:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="c">% For all pages:</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\topfraction</span><span class="nb">}{</span>0.9<span class="nb">}</span>     <span class="c">% max fraction of page filled by floats at top</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\bottomfraction</span><span class="nb">}{</span>0.8<span class="nb">}</span>  <span class="c">% max fraction of page filled by floats at bottom</span>

<span class="c">% For non-float pages:</span>
<span class="k">\setcounter</span><span class="nb">{</span>topnumber<span class="nb">}{</span>10<span class="nb">}</span>           <span class="c">% max number of floats at top of page</span>
<span class="k">\setcounter</span><span class="nb">{</span>bottomnumber<span class="nb">}{</span>10<span class="nb">}</span>        <span class="c">% max number of floats at bottom of page</span>
<span class="k">\setcounter</span><span class="nb">{</span>totalnumber<span class="nb">}{</span>10<span class="nb">}</span>         <span class="c">% max number of floats on page</span>
<span class="k">\setcounter</span><span class="nb">{</span>dbltopnumber<span class="nb">}{</span>10<span class="nb">}</span>        <span class="c">% max number of floats on 2-column page</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\dbltopfraction</span><span class="nb">}{</span>0.9<span class="nb">}</span>  <span class="c">% max fraction of 2-column page taken up by column-spanning floats at top</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\textfraction</span><span class="nb">}{</span>0.1<span class="nb">}</span>    <span class="c">% min fraction of page taken up by text</span>

<span class="c">% For float pages:</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\floatpagefraction</span><span class="nb">}{</span>0.7<span class="nb">}</span>	  <span class="c">% min fraction of page taken up by floats (must be less than \topfraction)</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\dblfloatpagefraction</span><span class="nb">}{</span>0.7<span class="nb">}</span> <span class="c">% min fraction of 2-column page taken up by floats</span>
</code></pre></div>

<p><cite>Modified version of settings
from
<a href="http://aty.sdsu.edu/~aty/bibliog/latex/floats.html#preamble">Controlling LaTeX floats</a> by
Andrew T. Young.</cite></p>

<p>See <a href="http://www.tex.ac.uk/FAQ-floats.html">Moving tables and figures in LaTeX</a>
for some other tips for wrangling floats.</p>

<h2 id="matching-matplotlib-style">Matching Matplotlib Style</h2>

<p>With a little bit of work, it&rsquo;s possible to make <a href="http://matplotlib.org/">Matplotlib</a> (Python
plotting library) use LaTeX for typesetting text with the same style as the
rest of your document. The following technique seems to work most of the time:</p>

<ol>
<li><p>Place your LaTeX config for typesetting math in <code>mymath.sty</code>.</p></li>

<li><p>Create a file <code>plt.py</code>:</p>
<div class="highlight"><pre><code class="language-python" data-lang="python"><span></span><span class="kn">from</span> <span class="nn">cycler</span> <span class="kn">import</span> <span class="n">cycler</span>
<span class="kn">import</span> <span class="nn">matplotlib</span>
<span class="kn">from</span> <span class="nn">matplotlib.backends.backend_pgf</span> <span class="kn">import</span> <span class="n">FigureCanvasPgf</span>
<span class="kn">from</span> <span class="nn">matplotlib.pyplot</span> <span class="kn">import</span> <span class="o">*</span>


<span class="k">def</span> <span class="nf">get_preamble</span><span class="p">(</span><span class="n">style_path</span><span class="p">):</span>
    <span class="n">preamble</span> <span class="o">=</span> <span class="p">[]</span>
    <span class="k">with</span> <span class="nb">open</span><span class="p">(</span><span class="n">style_path</span><span class="p">)</span> <span class="k">as</span> <span class="n">style_file</span><span class="p">:</span>
        <span class="k">for</span> <span class="n">line</span> <span class="ow">in</span> <span class="n">style_file</span><span class="p">:</span>
            <span class="k">if</span> <span class="ow">not</span> <span class="n">line</span><span class="o">.</span><span class="n">strip</span><span class="p">()</span><span class="o">.</span><span class="n">startswith</span><span class="p">(</span><span class="sa">r</span><span class="s1">&#39;\ProvidesPackage&#39;</span><span class="p">):</span>
                <span class="n">preamble</span><span class="o">.</span><span class="n">append</span><span class="p">(</span><span class="n">line</span><span class="p">)</span>
    <span class="k">return</span> <span class="n">preamble</span>


<span class="k">def</span> <span class="nf">init</span><span class="p">(</span><span class="n">color</span><span class="o">=</span><span class="kc">True</span><span class="p">,</span> <span class="n">tex</span><span class="o">=</span><span class="kc">True</span><span class="p">,</span> <span class="n">tex_style</span><span class="o">=</span><span class="s1">&#39;mymath.sty&#39;</span><span class="p">,</span> <span class="n">tex_engine</span><span class="o">=</span><span class="s1">&#39;xelatex&#39;</span><span class="p">):</span>
    <span class="sd">&quot;&quot;&quot;Initialize the settings.&quot;&quot;&quot;</span>
    <span class="k">if</span> <span class="n">tex</span><span class="p">:</span>
        <span class="k">if</span> <span class="n">tex_engine</span> <span class="ow">in</span> <span class="p">{</span><span class="s1">&#39;xelatex&#39;</span><span class="p">,</span> <span class="s1">&#39;lualatex&#39;</span><span class="p">,</span> <span class="s1">&#39;pdflatex&#39;</span><span class="p">}:</span>
            <span class="n">matplotlib</span><span class="o">.</span><span class="n">backend_bases</span><span class="o">.</span><span class="n">register_backend</span><span class="p">(</span><span class="s1">&#39;pdf&#39;</span><span class="p">,</span> <span class="n">FigureCanvasPgf</span><span class="p">)</span>
            <span class="n">rc</span><span class="p">(</span><span class="s1">&#39;font&#39;</span><span class="p">,</span> <span class="n">family</span><span class="o">=</span><span class="s1">&#39;serif&#39;</span><span class="p">)</span>
            <span class="n">rc</span><span class="p">(</span><span class="s1">&#39;pgf&#39;</span><span class="p">,</span> <span class="n">preamble</span><span class="o">=</span><span class="n">get_preamble</span><span class="p">(</span><span class="n">tex_style</span><span class="p">),</span>
               <span class="n">rcfonts</span><span class="o">=</span><span class="kc">False</span><span class="p">,</span> <span class="n">texsystem</span><span class="o">=</span><span class="n">tex_engine</span><span class="p">)</span>
        <span class="k">elif</span> <span class="n">tex_engine</span> <span class="o">==</span> <span class="s1">&#39;latex&#39;</span><span class="p">:</span>
            <span class="n">rc</span><span class="p">(</span><span class="s1">&#39;text&#39;</span><span class="p">,</span> <span class="n">usetex</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
            <span class="n">rc</span><span class="p">(</span><span class="s1">&#39;text.latex&#39;</span><span class="p">,</span> <span class="n">unicode</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
            <span class="n">rc</span><span class="p">(</span><span class="s1">&#39;font&#39;</span><span class="p">,</span> <span class="n">family</span><span class="o">=</span><span class="s1">&#39;serif&#39;</span><span class="p">,</span> <span class="n">serif</span><span class="o">=</span><span class="p">[</span><span class="s1">&#39;Computer Modern&#39;</span><span class="p">])</span>
        <span class="k">else</span><span class="p">:</span>
            <span class="k">raise</span> <span class="ne">ValueError</span><span class="p">(</span><span class="s1">&#39;Unrecognized tex engine: </span><span class="si">{}</span><span class="s1">&#39;</span><span class="o">.</span><span class="n">format</span><span class="p">(</span><span class="n">tex_engine</span><span class="p">))</span>
    <span class="k">if</span> <span class="ow">not</span> <span class="n">color</span><span class="p">:</span>
        <span class="n">rc</span><span class="p">(</span><span class="s1">&#39;axes&#39;</span><span class="p">,</span> <span class="n">prop_cycle</span><span class="o">=</span><span class="p">(</span><span class="n">cycler</span><span class="p">(</span><span class="s1">&#39;color&#39;</span><span class="p">,</span> <span class="p">[</span><span class="s1">&#39;k&#39;</span><span class="p">,</span> <span class="s1">&#39;k&#39;</span><span class="p">,</span> <span class="s1">&#39;k&#39;</span><span class="p">,</span> <span class="s1">&#39;k&#39;</span><span class="p">])</span> <span class="o">+</span>
                               <span class="n">cycler</span><span class="p">(</span><span class="s1">&#39;linestyle&#39;</span><span class="p">,</span> <span class="p">[</span><span class="s1">&#39;-&#39;</span><span class="p">,</span> <span class="s1">&#39;--&#39;</span><span class="p">,</span> <span class="s1">&#39;:&#39;</span><span class="p">,</span> <span class="s1">&#39;-.&#39;</span><span class="p">])))</span>
</code></pre></div></li>

<li><p>In the plotting script, import <code>plt</code> and call <code>plt.init()</code> (passing in any
options that are different from the defaults) to set up Matplotlib.</p></li>
</ol>

<p>If you use this technique, you have to be careful about what you put in
<code>mymath.sty</code>, and Matplotlib still won&rsquo;t let you use custom commands that
you&rsquo;ve defined, but it generally works fairly well for simple cases. For more
information, see the LaTeX-related options in <a href="http://matplotlib.org/users/customizing.html">Matplotlib&rsquo;s customization
documentation</a> and see the
<a href="http://matplotlib.org/users/pgf.html">documentation on typesetting with XeLaTeX/LuaLaTeX</a>.</p>

<h2 id="mathematics">Mathematics</h2>

<p>One of LaTeX’s strengths is typesetting mathematics.</p>

<h3 id="math-environments">Math Environments</h3>

<p>Use the <a href="http://ctan.org/pkg/amsmath"><code>amsmath</code></a> and <a href="http://ctan.org/pkg/amsfonts"><code>amssymb</code></a> packages for more math environments and
commands. In particular, the <code>align</code>, <code>aligned</code>, and <code>gathered</code> environments
are very useful.</p>

<h3 id="math-symbols">Math Symbols</h3>

<p>I prefer typesetting mathematics in the style of <a href="https://en.wikipedia.org/wiki/ISO_80000-2">ISO 80000‑2</a> (with some
minor modifications), which unfortunately is somewhat different from the
default LaTeX style. To achieve this, I use two different sets of packages,
based on whether I’m using <a href="https://www.tug.org/xetex/">XeLaTeX</a> (which supports the <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a>
package) or <a href="https://www.tug.org/applications/pdftex/">pdfLaTeX</a> (which doesn’t).</p>

<p>If you’re using <a href="https://www.tug.org/xetex/">XeLaTeX</a>, the <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a> package is great because it
supports a wider variety of symbols and styles than the defaults, and as a nice
bonus, it lets you type your math in Unicode. This is how I like to include it
in my documents:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="na">[math-style=ISO,bold-style=ISO,nabla=upright,partial=upright]</span><span class="nb">{</span>unicode-math<span class="nb">}</span>
<span class="k">\setmathfont</span><span class="nb">{</span>latinmodern-math.otf<span class="nb">}</span>
<span class="k">\setmathfont</span><span class="na">[range=\mathbb]</span><span class="nb">{</span>texgyretermes-math.otf<span class="nb">}</span>
</code></pre></div>

<p>Then, you can define convenient commands for bold italic vectors (<code>\vec</code>), bold
italic matrices (<code>\mat</code>), bold sans-serif tensors (<code>\tens</code>), and bold italic
(with hat) unit vectors (<code>\uvec</code>) like the following:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\renewcommand*</span><span class="nb">{</span><span class="k">\vec</span><span class="nb">}</span>[1]<span class="nb">{</span><span class="k">\symbfit</span><span class="nb">{</span><span class="k">\symbf</span><span class="nb">{</span>#1<span class="nb">}}}</span> <span class="c">% Nested \symbfit and \symbf to handle \vec{0}</span>
<span class="k">\newcommand*</span><span class="nb">{</span><span class="k">\mat</span><span class="nb">}</span>[1]<span class="nb">{</span><span class="k">\symbfit</span><span class="nb">{</span>#1<span class="nb">}}</span>
<span class="k">\newcommand*</span><span class="nb">{</span><span class="k">\tens</span><span class="nb">}</span>[1]<span class="nb">{</span><span class="k">\symbfsf</span><span class="nb">{</span>#1<span class="nb">}}</span>
<span class="k">\newcommand*</span><span class="nb">{</span><span class="k">\uveci</span><span class="nb">}{</span><span class="k">\symbfit</span><span class="nb">{</span><span class="k">\hat</span><span class="nb">{</span><span class="k">\mkern</span>-1mu <span class="k">\textit</span><span class="nb">{</span><span class="k">\bfseries\i</span><span class="nb">}</span> <span class="k">\mkern</span>1.5mu<span class="nb">}}}</span>
<span class="k">\newcommand*</span><span class="nb">{</span><span class="k">\uvecj</span><span class="nb">}{</span><span class="k">\symbfit</span><span class="nb">{</span><span class="k">\hat</span><span class="nb">{</span><span class="k">\textit</span><span class="nb">{</span><span class="k">\bfseries\j</span><span class="nb">}</span> <span class="k">\mkern</span>1.5mu<span class="nb">}}}</span>
<span class="k">\DeclareRobustCommand</span><span class="nb">{</span><span class="k">\uvec</span><span class="nb">}</span>[1]<span class="nb">{{</span>
  <span class="k">\ifcsname</span> uvec<span class="k">\detokenize</span><span class="nb">{</span>#1<span class="nb">}</span><span class="k">\endcsname</span>
    <span class="k">\csname</span> uvec#1<span class="k">\endcsname</span>
  <span class="k">\else</span>
    <span class="k">\symbfit</span><span class="nb">{</span><span class="k">\symbf</span><span class="nb">{</span><span class="k">\hat</span><span class="nb">{</span>#1<span class="nb">}}}</span> <span class="c">% Nested \symbfit and \symbf to handle \uvec{0}</span>
  <span class="k">\fi</span>
<span class="nb">}}</span>
</code></pre></div>

<p>I haven&rsquo;t fully refined my configuration for <a href="https://www.tug.org/applications/pdftex/">pdfLaTeX</a>. That said,
the <a href="http://ctan.org/pkg/bm"><code>bm</code></a> package is the most reliable option I’ve found for making bold
italic characters. I also had some success with using the <a href="http://ctan.org/pkg/isomath"><code>isomath</code></a>
and <a href="http://ctan.org/pkg/mathdesign"><code>mathdesign</code></a> packages with the Utopia font, like this:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="na">[utopia]</span><span class="nb">{</span>mathdesign<span class="nb">}</span>
<span class="k">\usepackage</span><span class="nb">{</span>isomath<span class="nb">}</span>
<span class="k">\renewcommand</span><span class="nb">{</span><span class="k">\vec</span><span class="nb">}{</span><span class="k">\vectorsym</span><span class="nb">}</span>
<span class="k">\newcommand</span><span class="nb">{</span><span class="k">\uvec</span><span class="nb">}</span>[1]<span class="nb">{</span><span class="k">\vectorsym</span><span class="nb">{</span><span class="k">\hat</span><span class="nb">{</span>#1<span class="nb">}}}</span>
<span class="k">\newcommand</span><span class="nb">{</span><span class="k">\mat</span><span class="nb">}{</span><span class="k">\matrixsym</span><span class="nb">}</span>
<span class="k">\newcommand</span><span class="nb">{</span><span class="k">\tens</span><span class="nb">}{</span><span class="k">\tensorsym</span><span class="nb">}</span>
</code></pre></div>

<h3 id="delimiters">Delimiters</h3>

<p>The <a href="http://ctan.org/pkg/commath"><code>commath</code></a> package provides convenient commands to typeset delimiters.
Note that if you’re using <a href="http://ctan.org/pkg/commath"><code>commath</code></a> with the <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a> package,
you must include <a href="http://ctan.org/pkg/commath"><code>commath</code></a> in your preamble before <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a>.</p>

<p>Personally, I prefer redefining the <code>\del</code> command provided by <a href="http://ctan.org/pkg/commath"><code>commath</code></a> to
another name because I use <code>\del</code> to refer to the del operator <strong>∇</strong> (a nabla
vector). To redefine <code>\del</code> to e.g. <code>\prn</code>, use</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\let\prn\del</span>
</code></pre></div>

<p>I always use <code>\prn</code> and the delimiter commands provided by <a href="http://ctan.org/pkg/commath"><code>commath</code></a>
(<code>\sbr</code>, <code>\cbr</code>, etc.) for scaled delimiters instead of the standard LaTeX
<code>\left</code> and <code>\right</code> commands for scaling delimiters, because they reduce
typing and my editor (<a href="https://www.gnu.org/software/emacs/">GNU Emacs</a>) makes it very easy to manipulate
expressions surrounded by simple delimiter pairs (like curly braces). My editor
also makes it easy to identify matched/unmatched curly braces. In contrast,
trying to make sure the <code>\left</code> and <code>\right</code> commands are always properly
paired is a pain.</p>

<h3 id="derivatives">Derivatives</h3>

<p>The <a href="http://ctan.org/pkg/commath"><code>commath</code></a> package provides convenient commands to typeset derivatives
and differentials, in particular the <code>\od</code>, <code>\pd</code>, <code>\md</code>, and <code>\dif</code> commands
and their variants. Note that if you’re using <a href="http://ctan.org/pkg/commath"><code>commath</code></a> with
the <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a> package, you must include <a href="http://ctan.org/pkg/commath"><code>commath</code></a> in your preamble
before <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a>.</p>

<p>One problem is that, by default, LaTeX typesets the <code>\partial</code> character in
partial derivatives as slanted instead of upright. (The ISO convention is that
all operators, including differential operators, should be typeset upright.)</p>

<p>If you’re using <a href="https://www.tug.org/xetex/">XeLaTeX</a>, you can get an upright <code>\partial</code> character simply
by using the <a href="http://ctan.org/pkg/unicode-math"><code>unicode-math</code></a> package with the <code>partial=upright</code> option. This
gives output like the following:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/partial-derivative-upright-unicode-math-example.tex">
        <img src="https://jim.turner.link/media/teximg/partial-derivative-upright-unicode-math-example___default.png"
srcset="




/media/teximg/partial-derivative-upright-unicode-math-example___128w.png 128w,
/media/teximg/partial-derivative-upright-unicode-math-example___32w.png 32w,
/media/teximg/partial-derivative-upright-unicode-math-example___48w.png 48w,
/media/teximg/partial-derivative-upright-unicode-math-example___64w.png 64w,
/media/teximg/partial-derivative-upright-unicode-math-example___96w.png 96w"


sizes="(min-width: 4rem) 4rem, (min-width: 2rem) 100vw, 2rem"
style="width:4rem; min-width:2rem"

 alt="Partial derivative with upright ∂ characters using unicode-math">
    </a>
</figure>

<p>If you’re using <a href="https://www.tug.org/applications/pdftex/">pdfLaTeX</a>, you need to either take the upright <code>\partial</code>
character from another font or unslant the default <code>\partial</code> character. The
default output looks like this:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/partial-derivative-slanted-example.tex">
        <img src="https://jim.turner.link/media/teximg/partial-derivative-slanted-example___default.png"
srcset="




/media/teximg/partial-derivative-slanted-example___128w.png 128w,
/media/teximg/partial-derivative-slanted-example___32w.png 32w,
/media/teximg/partial-derivative-slanted-example___48w.png 48w,
/media/teximg/partial-derivative-slanted-example___64w.png 64w,
/media/teximg/partial-derivative-slanted-example___96w.png 96w"


sizes="(min-width: 4rem) 4rem, (min-width: 2rem) 100vw, 2rem"
style="width:4rem; min-width:2rem"

 alt="Partial derivative with the default slanted ∂ characters">
    </a>
</figure>

<p>To replace the default <code>\partial</code> character with the one from the <code>fourier</code>
font, you can include these commands in the preamble:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\DeclareFontEncoding</span><span class="nb">{</span>FML<span class="nb">}{}{}</span>
<span class="k">\DeclareFontSubstitution</span><span class="nb">{</span>FML<span class="nb">}{</span>futm<span class="nb">}{</span>m<span class="nb">}{</span>it<span class="nb">}</span>
<span class="k">\DeclareSymbolFont</span><span class="nb">{</span>fourier<span class="nb">}{</span>FML<span class="nb">}{</span>futm<span class="nb">}{</span>m<span class="nb">}{</span>it<span class="nb">}</span>
<span class="k">\DeclareMathSymbol</span><span class="nb">{</span><span class="k">\partialup</span><span class="nb">}{</span><span class="k">\mathord</span><span class="nb">}{</span>fourier<span class="nb">}{</span>130<span class="nb">}</span>
<span class="k">\renewcommand*</span><span class="nb">{</span><span class="k">\partial</span><span class="nb">}{</span><span class="k">\partialup</span><span class="nb">}</span>
</code></pre></div>

<p><cite><a href="https://tex.stackexchange.com/a/27533">LaTeX Stack Exchange answer</a>
by <a href="https://tex.stackexchange.com/users/1495/kahen">kahen</a>, © 2011 and licensed
under <a href="https://creativecommons.org/licenses/by-sa/3.0/">CC BY‑SA 3.0</a>.</cite></p>

<p>which results in output like this:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/partial-derivative-upright-fourier-example.tex">
        <img src="https://jim.turner.link/media/teximg/partial-derivative-upright-fourier-example___default.png"
srcset="




/media/teximg/partial-derivative-upright-fourier-example___125w.png 125w,
/media/teximg/partial-derivative-upright-fourier-example___31w.png 31w,
/media/teximg/partial-derivative-upright-fourier-example___47w.png 47w,
/media/teximg/partial-derivative-upright-fourier-example___63w.png 63w,
/media/teximg/partial-derivative-upright-fourier-example___94w.png 94w"


sizes="(min-width: 3.9375rem) 3.9375rem, (min-width: 1.96875rem) 100vw, 1.96875rem"
style="width:3.9375rem; min-width:1.96875rem"

 alt="Partial derivative with upright ∂ character from fourier font">
    </a>
</figure>

<p>“Unslanting” the default <code>\partial</code> character tends to blend in better with the
surrounding text. To do this, use these commands:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="nb">{</span>scalerel<span class="nb">}</span>
<span class="k">\newsavebox</span><span class="nb">{</span><span class="k">\unslantbox</span><span class="nb">}</span>
<span class="k">\newlength</span><span class="nb">{</span><span class="k">\unslantdim</span><span class="nb">}</span>
<span class="k">\newcommand</span><span class="nb">{</span><span class="k">\slantbox</span><span class="nb">}</span>[2][0]<span class="nb">{</span><span class="k">\mbox</span><span class="nb">{</span><span class="c">%</span>
        <span class="k">\sbox</span><span class="nb">{</span><span class="k">\unslantbox</span><span class="nb">}{</span>#2<span class="nb">}</span><span class="c">%</span>
        <span class="k">\unslantdim</span>=#1<span class="k">\wd\unslantbox</span>
        <span class="k">\hskip</span> <span class="k">\wd\unslantbox</span>
        <span class="k">\hskip</span> -0.5<span class="k">\unslantdim</span>
        <span class="k">\pdfsave</span>
        <span class="k">\pdfsetmatrix</span><span class="nb">{</span>1 0 #1 1<span class="nb">}</span><span class="c">%</span>
        <span class="k">\llap</span><span class="nb">{</span><span class="k">\usebox</span><span class="nb">{</span><span class="k">\unslantbox</span><span class="nb">}}</span><span class="c">%</span>
        <span class="k">\pdfrestore</span>
        <span class="k">\hskip</span> 0.5<span class="k">\unslantdim</span>
<span class="nb">}}</span>
<span class="k">\newcommand\unslant</span><span class="na">[2][-.25]</span><span class="nb">{</span><span class="k">\ThisStyle</span><span class="nb">{</span><span class="k">\slantbox</span><span class="na">[#1]</span><span class="nb">{</span><span class="s">$</span><span class="nv">\SavedStyle</span><span class="nb">#</span><span class="m">2</span><span class="s">$</span><span class="nb">}}}</span>

<span class="k">\let\partialslanted\partial</span>
<span class="k">\renewcommand*</span><span class="nb">{</span><span class="k">\partial</span><span class="nb">}{</span><span class="k">\unslant</span><span class="nb">{</span><span class="k">\partialslanted</span><span class="nb">}}</span>
</code></pre></div>

<p><cite>Modified version
of <a href="https://tex.stackexchange.com/a/239823">this LaTeX Stack Exchange answer</a>
by <a href="https://tex.stackexchange.com/users/34505/john-kormylo">John Kormylo</a>,
© 2015 and licensed under <a href="https://creativecommons.org/licenses/by-sa/3.0/">CC BY‑SA 3.0</a>.</cite></p>

<p>which results in output like this:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/partial-derivative-upright-unslant-example.tex">
        <img src="https://jim.turner.link/media/teximg/partial-derivative-upright-unslant-example___default.png"
srcset="




/media/teximg/partial-derivative-upright-unslant-example___128w.png 128w,
/media/teximg/partial-derivative-upright-unslant-example___32w.png 32w,
/media/teximg/partial-derivative-upright-unslant-example___48w.png 48w,
/media/teximg/partial-derivative-upright-unslant-example___64w.png 64w,
/media/teximg/partial-derivative-upright-unslant-example___96w.png 96w"


sizes="(min-width: 4rem) 4rem, (min-width: 2rem) 100vw, 2rem"
style="width:4rem; min-width:2rem"

 alt="Partial derivative with upright ∂ character by unslanting the default">
    </a>
</figure>

<h2 id="useful-packages-by-topic">Useful Packages by Topic</h2>

<p>This is a collection of useful packages that I usually include in my documents,
organized by topic.</p>

<h3 id="page-layout-e-g-margins">Page Layout (e.g. Margins)</h3>

<p>The <a href="http://ctan.org/pkg/geometry"><code>geometry</code></a> package makes it simple to adjust page layout. In most of my
documents, I include the following line to get 1 inch margins:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="na">[margin=1in]</span><span class="nb">{</span>geometry<span class="nb">}</span>
</code></pre></div>

<h3 id="numbers-and-units">Numbers and Units</h3>

<p>The <a href="http://ctan.org/pkg/siunitx"><code>siunitx</code></a> package provides commands for correctly typesetting numbers
and units according to SI/ISO/NIST standards and makes it easy to switch
between styles of unit display. You can typeset numbers with the <code>\num</code>
command, such as <code>\num{4.135667662(25)e-15}</code>, which yields</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/siunitx-num-example.tex">
        <img src="https://jim.turner.link/media/teximg/siunitx-num-example___default.png"
srcset="




/media/teximg/siunitx-num-example___146w.png 146w,
/media/teximg/siunitx-num-example___194w.png 194w,
/media/teximg/siunitx-num-example___291w.png 291w,
/media/teximg/siunitx-num-example___388w.png 388w,
/media/teximg/siunitx-num-example___97w.png 97w"


sizes="(min-width: 12.125rem) 12.125rem, (min-width: 6.0625rem) 100vw, 6.0625rem"
style="width:12.125rem; min-width:6.0625rem"

 alt="Example of typesetting a number with the \num command">
    </a>
</figure>

<p>You can typeset units with the <code>\si</code> command. You can specify the units by
typing the literal representation (e.g. <code>\si{m/s}</code>) or by using commands
provided by <a href="http://ctan.org/pkg/siunitx"><code>siunitx</code></a>. The advantage of using the commands is that
adjusting the format of all of the units later is simply a matter of changing
the <a href="http://ctan.org/pkg/siunitx"><code>siunitx</code></a> configuration. An example of using the command format is
<code>\si{\micro\meter\candela\squared\per\second}</code>, which yields</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/siunitx-si-unit-example.tex">
        <img src="https://jim.turner.link/media/teximg/siunitx-si-unit-example___default.png"
srcset="




/media/teximg/siunitx-si-unit-example___120w.png 120w,
/media/teximg/siunitx-si-unit-example___159w.png 159w,
/media/teximg/siunitx-si-unit-example___40w.png 40w,
/media/teximg/siunitx-si-unit-example___60w.png 60w,
/media/teximg/siunitx-si-unit-example___80w.png 80w"


sizes="(min-width: 5rem) 5rem, (min-width: 2.5rem) 100vw, 2.5rem"
style="width:5rem; min-width:2.5rem"

 alt="Example of typesetting units with the \si command">
    </a>
</figure>

<p>You can typeset physical quantities (numbers with units) with the <code>\SI</code>
command, such as <code>\SI{9.81}{\meter\per\second\squared}</code>, which yields</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/siunitx-physical-quantity-example.tex">
        <img src="https://jim.turner.link/media/teximg/siunitx-physical-quantity-example___default.png"
srcset="




/media/teximg/siunitx-physical-quantity-example___114w.png 114w,
/media/teximg/siunitx-physical-quantity-example___152w.png 152w,
/media/teximg/siunitx-physical-quantity-example___38w.png 38w,
/media/teximg/siunitx-physical-quantity-example___57w.png 57w,
/media/teximg/siunitx-physical-quantity-example___76w.png 76w"


sizes="(min-width: 4.75rem) 4.75rem, (min-width: 2.375rem) 100vw, 2.375rem"
style="width:4.75rem; min-width:2.375rem"

 alt="Example of typesetting a physical quantity with the \SI command">
    </a>
</figure>

<p>If you prefer a slash instead of negative exponents, you can add
<code>\sisetup{per-mode=symbol}</code> after loading <a href="http://ctan.org/pkg/siunitx"><code>siunitx</code></a>. Note that the
<code>\num</code>, <code>\si</code>, and <code>\SI</code> commands can be used in both text and math mode.</p>

<p><a href="http://ctan.org/pkg/siunitx"><code>siunitx</code></a> also provides an <code>S</code> column type that aligns numbers by their
decimal point and typesets numbers in the same way as the <code>\num</code> command. Note
that you have to be careful to wrap your headers in curly braces. Additionally,
it’s a good idea to specify <code>table-format</code> for the <code>S</code> columns to ensure the
headers are properly centered. (Otherwise, they’re centered about the decimal
point.) For example:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\begin</span><span class="nb">{</span>tabular<span class="nb">}</span>
  <span class="nb">{</span>
    @<span class="nb">{}</span>
    &gt;<span class="nb">{</span><span class="s">$</span><span class="nb">}l&lt;{</span><span class="s">$</span><span class="nb">}</span>
    S[table-format=-2.3(1)]
    S[table-format=3.3(1)]
    @<span class="nb">{}</span>
  <span class="nb">}</span>
  <span class="k">\toprule</span>
  <span class="nb">{</span><span class="k">\text</span><span class="nb">{</span>Parameter<span class="nb">}}</span> <span class="nb">&amp;</span> <span class="nb">{</span>First<span class="nb">}</span> <span class="nb">&amp;</span> <span class="nb">{</span>Second<span class="nb">}</span> <span class="k">\\</span>
  <span class="k">\midrule</span>
  a / <span class="k">\si</span><span class="nb">{</span><span class="k">\angstrom</span><span class="nb">}</span> <span class="nb">&amp;</span> 1.234(2) <span class="nb">&amp;</span> 5.678(4) <span class="k">\\</span>
  <span class="k">\theta</span> / <span class="k">\si</span><span class="nb">{</span><span class="k">\degree</span><span class="nb">}</span> <span class="nb">&amp;</span> -90.34(4) <span class="nb">&amp;</span> 104.45(7) <span class="k">\\</span>
  q / <span class="k">\si</span><span class="nb">{</span><span class="k">\micro\meter</span><span class="nb">}</span> <span class="nb">&amp;</span> 0.532 <span class="nb">&amp;</span> 0.894 <span class="k">\\</span>
  <span class="k">\gamma</span> / <span class="k">\si</span><span class="nb">{</span><span class="k">\per\kilo\gram</span><span class="nb">}</span> <span class="nb">&amp;</span> 2.734 <span class="nb">&amp;</span> 1.485 <span class="k">\\</span>
  <span class="k">\bottomrule</span>
<span class="k">\end</span><span class="nb">{</span>tabular<span class="nb">}</span>
</code></pre></div>

<p>yields</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/siunitx-s-column-example.tex">
        <img src="https://jim.turner.link/media/teximg/siunitx-s-column-example___default.png"
srcset="




/media/teximg/siunitx-s-column-example___145w.png 145w,
/media/teximg/siunitx-s-column-example___217w.png 217w,
/media/teximg/siunitx-s-column-example___289w.png 289w,
/media/teximg/siunitx-s-column-example___434w.png 434w,
/media/teximg/siunitx-s-column-example___579w.png 579w"


sizes="(min-width: 18.0625rem) 18.0625rem, (min-width: 9.03125rem) 100vw, 9.03125rem"
style="width:18.0625rem; min-width:9.03125rem"

 alt="Table with one math column and two S columns">
    </a>
</figure>

<h3 id="graphics">Graphics</h3>

<p>The <a href="http://ctan.org/pkg/graphicx"><code>graphicx</code></a> package works well for including most types of external
images (e.g. JPG, PNG, EPS, PDF) but does not have support for SVG images.</p>

<p>The <a href="http://ctan.org/pkg/svg"><code>svg</code></a> package makes it easy to include SVG images by converting them to
PDFs with <a href="https://inkscape.org/">Inkscape</a>. It also makes it possible to compile the text in your
SVGs with LaTeX so the formatting of the text and equations matches your
document. Note that using <a href="http://tex.stackexchange.com/a/74693">MS Windows requires some special configuration</a> and using <a href="#svg-subcaption-workaround">the <code>subcaption</code> package
requires a workaround</a>. Also note that on all
platforms, you need to use the <code>-shell-escape</code> option with pdfLaTeX for this to
work. For example, if you’re using a <a href="https://www.gnu.org/software/make/manual/make.html">Makefile</a> with <a href="http://www.ctan.org/pkg/latexmk">latexmk</a>, then you
could write your rule like this:</p>
<div class="highlight"><pre><code class="language-makefile" data-lang="makefile"><span></span><span class="nf">%.pdf</span><span class="o">:</span> %.<span class="n">tex</span> <span class="k">$(</span><span class="nv">wildcard</span> *<span class="nv">.svg</span><span class="k">)</span>
	latexmk -pdf -pdflatex<span class="o">=</span><span class="s2">&quot;pdflatex -shell-escape %O %S&quot;</span> <span class="s2">&quot;</span>$<span class="s2">&lt;&quot;</span>
</code></pre></div>

<p>The <a href="http://ctan.org/pkg/pgf"><code>tikz</code></a> package is great for drawing complex technical diagrams that
would be difficult to draw by other means. As a nice bonus, the source of the
diagram is plain text, so it works well with version control. For example, I
drew this image with <a href="http://ctan.org/pkg/pgf"><code>tikz</code></a>:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/tikz-schematic-example.tex">
        <img src="https://jim.turner.link/media/teximg/tikz-schematic-example___default.png"
srcset="




/media/teximg/tikz-schematic-example___1143w.png 1143w,
/media/teximg/tikz-schematic-example___286w.png 286w,
/media/teximg/tikz-schematic-example___429w.png 429w,
/media/teximg/tikz-schematic-example___572w.png 572w,
/media/teximg/tikz-schematic-example___857w.png 857w"


sizes="(min-width: 35.75rem) 35.75rem, (min-width: 17.875rem) 100vw, 17.875rem"
style="width:35.75rem; min-width:17.875rem"

 alt="Tilted disc with off-center hole illustrating multiple rotated 3-D coordinate systems">
    </a>
</figure>

<p>You can find many additional examples at <a href="https://texample.net/tikz/examples/">TeXample.net</a>.</p>

<p>Often, the best way to include a <a href="http://ctan.org/pkg/pgf"><code>tikz</code></a> image in your document is to create
it in a separate file, and then include it with the <a href="http://ctan.org/pkg/standalone"><code>standalone</code></a> package.
This way, you can render the image by itself quickly while you’re editing it,
and your main document stays clean.</p>

<h3 id="captions">Captions</h3>

<p>I prefer the captions to be smaller than the body text. You can achieve this
with the <a href="http://ctan.org/pkg/caption"><code>caption</code></a> package. It also provides the <code>\captionof</code> command,
which is useful for cover pages. Include the package like this to get small
captions:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="na">[font=small]</span><span class="nb">{</span>caption<span class="nb">}</span>
</code></pre></div>

<p>Often, it’s useful to have multiple subfigures/subtables make up an overall
figure/table and be able to label/cross-reference the subfigures/subtables
individually. The <a href="http://ctan.org/pkg/subcaption"><code>subcaption</code></a> package works well for this, in particular
the <code>subfigure</code>/<code>subtable</code> environments and the <code>\subcaptionbox</code> command.
Include the package like this:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="nb">{</span>subcaption<span class="nb">}</span>
</code></pre></div>

<p>Here&rsquo;s an example of using the <code>subfigure</code> environment:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/subcaption-subfigure-example.tex">
        <img src="https://jim.turner.link/media/teximg/subcaption-subfigure-example___default.png"
srcset="




/media/teximg/subcaption-subfigure-example___213w.png 213w,
/media/teximg/subcaption-subfigure-example___319w.png 319w,
/media/teximg/subcaption-subfigure-example___426w.png 426w,
/media/teximg/subcaption-subfigure-example___638w.png 638w,
/media/teximg/subcaption-subfigure-example___851w.png 851w"


sizes="(min-width: 26.625rem) 26.625rem, (min-width: 13.3125rem) 100vw, 13.3125rem"
style="width:26.625rem; min-width:13.3125rem"

 alt="Subfigure environment example using subcaption package, illustrating two subfigures side-by-side.">
    </a>
</figure>

<p><a id="svg-subcaption-workaround"></a> Note that if you use
the <a href="http://ctan.org/pkg/subcaption"><code>subcaption</code></a> package with the <a href="http://ctan.org/pkg/svg"><code>svg</code></a> package, you need to add the
following to your preamble (before including <a href="http://ctan.org/pkg/svg"><code>svg</code></a>) to avoid conflicts
between the packages:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="nb">{</span>scrlfile<span class="nb">}</span>
<span class="k">\PreventPackageFromLoading</span><span class="nb">{</span>subfig<span class="nb">}</span>
</code></pre></div>

<h3 id="table-formatting">Table Formatting</h3>

<p>The <a href="http://ctan.org/pkg/booktabs"><code>booktabs</code></a> package makes it easy to generate professional-looking
tables with a minimum of effort (only three commands for most tables), such as
this one:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/booktabs-simple-example.tex">
        <img src="https://jim.turner.link/media/teximg/booktabs-simple-example___default.png"
srcset="




/media/teximg/booktabs-simple-example___164w.png 164w,
/media/teximg/booktabs-simple-example___245w.png 245w,
/media/teximg/booktabs-simple-example___327w.png 327w,
/media/teximg/booktabs-simple-example___491w.png 491w,
/media/teximg/booktabs-simple-example___654w.png 654w"


sizes="(min-width: 20.4375rem) 20.4375rem, (min-width: 10.2188rem) 100vw, 10.2188rem"
style="width:20.4375rem; min-width:10.2188rem"

 alt="Simple table using booktabs package, with horizontal borders at top, below header row, and at bottom.">
    </a>
</figure>

<p>Here&rsquo;s a more complex example from the documentation:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/booktabs-complex-example.tex">
        <img src="https://jim.turner.link/media/teximg/booktabs-complex-example___default.png"
srcset="




/media/teximg/booktabs-complex-example___139w.png 139w,
/media/teximg/booktabs-complex-example___208w.png 208w,
/media/teximg/booktabs-complex-example___277w.png 277w,
/media/teximg/booktabs-complex-example___416w.png 416w,
/media/teximg/booktabs-complex-example___555w.png 555w"


sizes="(min-width: 17.3125rem) 17.3125rem, (min-width: 8.65625rem) 100vw, 8.65625rem"
style="width:17.3125rem; min-width:8.65625rem"

 alt="More complex table using booktabs package, with a full-width border at the top, partial width between the two header rows, full-width below the lower header row, and full-width at the bottom.">
    </a>
</figure>

<p>In some cases, the <a href="http://ctan.org/pkg/tabu"><code>tabu</code></a> package is easier to use than the built-in LaTeX
<code>tabular</code> environment, such as when creating tables inside of equations.</p>

<h3 id="code-listings">Code Listings</h3>

<p>Use the <a href="http://ctan.org/pkg/listings"><code>listings</code></a> package. It allows you to include external files as code
listings, easily cross-reference the listings, and format the listings nicely.
This is how I like include the package:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="nb">{</span>listings<span class="nb">}</span>
<span class="k">\lstset</span><span class="nb">{</span>frame=single,basicstyle=<span class="k">\small\ttfamily</span>,showstringspaces=false<span class="nb">}</span>
</code></pre></div>

<p>and here&rsquo;s an example of the output:</p>

<figure>
    <a href="https://jim.turner.link/downloads/teximg/listings-example.tex">
        <img src="https://jim.turner.link/media/teximg/listings-example___default.png"
srcset="




/media/teximg/listings-example___1246w.png 1246w,
/media/teximg/listings-example___312w.png 312w,
/media/teximg/listings-example___467w.png 467w,
/media/teximg/listings-example___623w.png 623w,
/media/teximg/listings-example___935w.png 935w"


sizes="(min-width: 38.9375rem) 38.9375rem, (min-width: 19.4688rem) 100vw, 19.4688rem"
style="width:38.9375rem; min-width:19.4688rem"

 alt="Simple listing using the listings package, showing the source code of the .tex file in a figure.">
    </a>
</figure>

<h3 id="hyperlinks-urls-and-pdf-metadata">Hyperlinks, URLs, and PDF Metadata</h3>

<p>Use the <a href="http://ctan.org/pkg/hyperref"><code>hyperref</code></a> package to automatically create links for
cross-references, to allow you create external links to URLs, and to allow you
to edit the metadata of the output PDF. For example, I include
the <a href="http://ctan.org/pkg/hyperref"><code>hyperref</code></a> package like this (I define <code>\thetitle</code> and <code>\theauthor</code>
elsewhere):</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="na">[unicode,xetex]</span><span class="nb">{</span>hyperref<span class="nb">}</span>
<span class="k">\hypersetup</span><span class="nb">{</span>
  colorlinks,
  citecolor = black,
  filecolor = black,
  linkcolor = black,
  urlcolor = black,
  pdftitle = <span class="k">\thetitle</span>,
  pdfauthor = <span class="k">\theauthor</span>,
  pdfdisplaydoctitle = true
<span class="nb">}</span>
</code></pre></div>

<p>You can create a hyperlinked URL like this: <code>\url{https://archive.org}</code>, and
you can create a named external link like this:
<code>\href{https://archive.org}{Internet Archive}</code>.</p>

<h3 id="cross-referencing">Cross-Referencing</h3>

<p>The <a href="http://ctan.org/pkg/cleveref"><code>cleveref</code></a> package makes it easy to correctly format cross references.
For example, you can write <code>\cref{fig:foo,fig:bar}</code> instead of
<code>figures~\ref{fig:foo} and~\ref{fig:bar}</code>. I change the <code>\creflastjunction</code>
from the default to add a serial comma:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="nb">{</span>cleveref<span class="nb">}</span>
<span class="k">\newcommand</span><span class="nb">{</span><span class="k">\creflastconjunction</span><span class="nb">}{</span>, and~<span class="nb">}</span>
</code></pre></div>

<p>Note that you must include the <a href="http://ctan.org/pkg/cleveref"><code>cleveref</code></a> package <em>after</em> you include
the <a href="http://ctan.org/pkg/hyperref"><code>hyperref</code></a> package.</p>

<h3 id="header-and-footer">Header and Footer</h3>

<p>I like to have a fancy header that gives the current section, document title,
and author’s name. I use the <a href="http://ctan.org/pkg/fancyhdr"><code>fancyhdr</code></a> package:</p>
<div class="highlight"><pre><code class="language-latex" data-lang="latex"><span></span><span class="k">\usepackage</span><span class="nb">{</span>fancyhdr<span class="nb">}</span>
<span class="k">\fancyhf</span><span class="nb">{}</span>
<span class="k">\fancyhead</span><span class="na">[L]</span><span class="nb">{</span><span class="k">\leftmark</span><span class="nb">}</span>
<span class="k">\fancyhead</span><span class="na">[C]</span><span class="nb">{</span><span class="k">\MakeUppercase</span><span class="nb">{</span><span class="k">\thetitle</span><span class="nb">}}</span>
<span class="k">\fancyhead</span><span class="na">[R]</span><span class="nb">{</span><span class="k">\MakeUppercase</span><span class="nb">{</span><span class="k">\theauthor</span><span class="nb">}}</span>
<span class="k">\fancyfoot</span><span class="na">[C]</span><span class="nb">{</span><span class="k">\thepage</span><span class="nb">}</span>
<span class="k">\pagestyle</span><span class="nb">{</span>fancy<span class="nb">}</span>
</code></pre></div>