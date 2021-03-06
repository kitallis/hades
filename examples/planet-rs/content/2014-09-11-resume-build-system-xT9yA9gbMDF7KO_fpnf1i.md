+++
title = "Resume Build System"
date = "2014-09-11T00:00:00+00:00"

[extra]
author = "Rust — Jim Turner"
link = "https://jim.turner.link/pages/resume-build-system/"
+++
<p>I developed a system to parse my resume from <a href="http://yaml.org/">YAML</a>-formatted data files and
use templates to generate PDF, HTML, plain text, <a href="http://sphinx-doc.org/">Sphinx</a>, and <a href="https://daringfireball.net/projects/markdown/">Markdown</a>
output. I track all of my data files and templates in <a href="https://www.git-scm.com/">Git</a>, which allows me
to easily keep track of changes over time, tag versions customized for specific
applications, etc. It is partly inspired by <a href="https://github.com/mhyee/resume">Ming-Ho Yee&rsquo;s resume project</a>.
You can view the public portions of the project source code on
its <a href="https://github.com/jturner314/resume">GitHub page</a>, and you can
view <a href="https://jim.turner.link/resume/">generated copies of my resume</a>.</p>

<h2 id="motivation">Motivation</h2>

<p>For a long time, I maintained my resume in Microsoft Word, but that was a pain
because</p>

<ul>
<li><p>Much of the formatting information was hidden in the
<abbr title="What You See Is What You Get">WYSIWYG</abbr> interface,
which made debugging formatting problems very frustrating, particularly on
something like a resume where precise formatting is important. I used Word&rsquo;s
support for styles, tab stops, etc., and I am very familiar with Word, but
it&rsquo;s just a frustrating and buggy program.</p></li>

<li><p>Word does not support Linux, so when I switched from Windows to Linux, I had
to use Word in a Windows virtual machine. I tried using <a href="https://www.libreoffice.org/">LibreOffice</a>
before switching to my current approach, but LibreOffice has most of Word&rsquo;s
other problems.</p></li>

<li><p>There was no good way to track my changes, view differences between versions,
etc., due to Word&rsquo;s binary format. Of course, Word has built-in reviewing and
diffing functionality, but it&rsquo;s designed for reviewing documents, not
long-term history tracking, and it doesn&rsquo;t integrate nicely with Git.</p></li>

<li><p>There was no good way to produce other file formats (except PDF), which were
necessary for some job applications and my personal website.</p></li>
</ul>

<p>Now, I have complete control of formatting without it being hidden behind a
GUI, I can build my resume on Linux, I can use Git for version control with
understandable diffs, and I can easily produce whatever file format I need.</p>

<h2 id="how-it-works">How it Works</h2>

<p>A <a href="https://www.python.org/">Python</a> script takes as arguments the desired output format, the template
to use, the output filename, and the source data file(s). It uses
the <a href="http://pyyaml.org/wiki/PyYAML">PyYAML</a> library to parse the YAML-formatted data file(s) into an
in-memory data structure. If multiple data files are provided, the script
merges the data together, which allows e.g. for separation between data to be
published online and data to be included only on hard copies (such as an
address or phone number).</p>

<p>Next, the script parses the section of the YAML-formatted configuration file
corresponding to the desired output format. This includes</p>

<ul>
<li><p>replacements to be applied to the raw data strings for escaping problematic
characters (e.g. replacing <code>&amp;</code> with <code>&amp;amp;</code> in HTML), applying simple markup
(e.g. <code>//emphasis//</code> for <em>emphasis</em>), etc.</p></li>

<li><p>the delimiters to be used by the template system (because the default Jinja2
delimiters don&rsquo;t work well with LaTeX)</p></li>

<li><p>the desired line endings for the output file (generally <code>\n</code>, but <code>\r\n</code> for
the plain text file for MS Notepad users)</p></li>
</ul>

<p>Finally, the script applies the replacements to the raw data strings and then
uses <a href="http://jinja.pocoo.org/">Jinja2</a> to fill the data into the template. The script provides some
custom filters to use in the templates for line wrapping, alignment, etc.</p>

<p>A <a href="https://www.gnu.org/software/make/manual/make.html#Introduction">Makefile</a> provides instructions to build individual files or just rebuild
all of them, so a simple call to <code>make</code> builds everything at once.</p>

<h2 id="input-files">Input Files</h2>

<p>This is a portion of a resume data file. It illustrates the YAML-formatted data
structure and some of the markup specified in the configuration file. For
example, <code>--</code> is used to represent en dashes in date ranges, <code>~</code> is used to
represent non-breaking spaces, and <code>//emphasis//</code> is used to emphasize award
names.</p>
<div class="highlight"><pre><code class="language-yaml" data-lang="yaml"><span></span><span class="nt">summary</span><span class="p">:</span>
  <span class="l l-Scalar l-Scalar-Plain">Mechanical engineering Ph.D.~student with research and industry experience in</span>
  <span class="l l-Scalar l-Scalar-Plain">reinforcement learning, nonlinear dynamical systems, mechanical simulation,</span>
  <span class="l l-Scalar l-Scalar-Plain">electromechanical controls, product development, testing and verification,</span>
  <span class="l l-Scalar l-Scalar-Plain">data analysis, and software development.</span>
<span class="nt">education</span><span class="p">:</span>
  <span class="p p-Indicator">-</span> <span class="nt">degree</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">Ph.D.~Mechanical Engineering</span>
    <span class="nt">university</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">Duke University</span>
    <span class="nt">location</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">Durham,~NC</span>
    <span class="nt">grade</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">4.00~GPA</span>
    <span class="nt">date</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">2015--present</span>
    <span class="nt">multicols</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">2</span>
    <span class="nt">description</span><span class="p">:</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//2017</span><span class="nv"> </span><span class="s">National</span><span class="nv"> </span><span class="s">Defense</span><span class="nv"> </span><span class="s">Science</span><span class="nv"> </span><span class="s">and</span><span class="nv"> </span><span class="s">Engineering</span><span class="nv"> </span><span class="s">Graduate</span><span class="nv"> </span><span class="s">(NDSEG)</span><span class="nv"> </span><span class="s">Fellowship://</span><span class="nv"> </span><span class="s">Merit-based,</span><span class="nv"> </span><span class="s">national,</span><span class="nv"> </span><span class="s">full-ride</span><span class="nv"> </span><span class="s">fellowship&quot;</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//2015</span><span class="nv"> </span><span class="s">James~B.~Duke</span><span class="nv"> </span><span class="s">Fellowship://</span><span class="nv"> </span><span class="s">Merit-based,</span><span class="nv"> </span><span class="s">four-year</span><span class="nv"> </span><span class="s">fellowship&quot;</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//2015</span><span class="nv"> </span><span class="s">Pratt/Gardner</span><span class="nv"> </span><span class="s">Fellowship://</span><span class="nv"> </span><span class="s">Merit-based</span><span class="nv"> </span><span class="s">fellowship&quot;</span>
  <span class="p p-Indicator">-</span> <span class="nt">degree</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">B.S.~Mechanical Engineering</span>
    <span class="nt">university</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">North Carolina State University</span>
    <span class="nt">location</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">Raleigh,~NC</span>
    <span class="nt">grade</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">Valedictorian, 4.00~GPA</span>
    <span class="nt">date</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">2011--2015</span>
    <span class="nt">multicols</span><span class="p">:</span> <span class="l l-Scalar l-Scalar-Plain">2</span>
    <span class="nt">description</span><span class="p">:</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//Minors://</span><span class="nv"> </span><span class="s">Spanish</span><span class="nv"> </span><span class="s">&amp;</span><span class="nv"> </span><span class="s">Computer</span><span class="nv"> </span><span class="s">Programming&quot;</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//2015</span><span class="nv"> </span><span class="s">NCSU</span><span class="nv"> </span><span class="s">Mech.~&amp;</span><span class="nv"> </span><span class="s">Aero.~Engineering</span><span class="nv"> </span><span class="s">Senior</span><span class="nv"> </span><span class="s">Award</span><span class="nv"> </span><span class="s">for</span><span class="nv"> </span><span class="s">Leadership//&quot;</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//2014</span><span class="nv"> </span><span class="s">Goldwater</span><span class="nv"> </span><span class="s">Scholar://</span><span class="nv"> </span><span class="s">Merit-based,</span><span class="nv"> </span><span class="s">national</span><span class="nv"> </span><span class="s">scholarship&quot;</span>
      <span class="p p-Indicator">-</span> <span class="s">&quot;//2011</span><span class="nv"> </span><span class="s">NCSU</span><span class="nv"> </span><span class="s">Park</span><span class="nv"> </span><span class="s">Scholar://</span><span class="nv"> </span><span class="s">Merit-based,</span><span class="nv"> </span><span class="s">full-ride</span><span class="nv"> </span><span class="s">scholarship&quot;</span>
</code></pre></div>

<p>This is a portion of a configuration file for LaTeX output:</p>
<div class="highlight"><pre><code class="language-yaml" data-lang="yaml"><span></span><span class="nt">latex</span><span class="p">:</span>
  <span class="nt">replacements</span><span class="p">:</span>
    <span class="p p-Indicator">-</span> <span class="p p-Indicator">[</span><span class="s">&#39;&amp;&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;\\&amp;&#39;</span><span class="p p-Indicator">]</span>
    <span class="p p-Indicator">-</span> <span class="p p-Indicator">[</span><span class="s">&#39;%&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;\\%&#39;</span><span class="p p-Indicator">]</span>
    <span class="p p-Indicator">-</span> <span class="p p-Indicator">[</span><span class="s">&#39;\$&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;\\$&#39;</span><span class="p p-Indicator">]</span>
    <span class="p p-Indicator">-</span> <span class="p p-Indicator">[</span><span class="s">&#39;LaTeX&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;\\LaTeX&#39;</span><span class="p p-Indicator">]</span>
    <span class="p p-Indicator">-</span> <span class="p p-Indicator">[</span><span class="s">&#39;//(.*?)//&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;\\emph{\1}&#39;</span><span class="p p-Indicator">]</span>
  <span class="nt">jinja2_delim</span><span class="p">:</span> <span class="p p-Indicator">[</span><span class="s">&#39;%&lt;&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;&gt;%&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;&lt;&lt;&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;&gt;&gt;&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;%%&lt;&#39;</span><span class="p p-Indicator">,</span> <span class="s">&#39;&gt;%%&#39;</span><span class="p p-Indicator">]</span>
  <span class="nt">line_endings</span><span class="p">:</span> <span class="s">&quot;\n&quot;</span>
</code></pre></div>

<p>This is a portion of a template for plain text output. It illustrates some of
the features of the <a href="http://jinja.pocoo.org/docs/dev/templates/">Jinja2 templating language</a>, including filters,
conditionals, and loops.</p>
<div class="highlight"><pre><code class="language-jinja" data-lang="jinja"><span></span><span class="cp">{{</span> <span class="s2">&quot;#&quot;</span> <span class="o">*</span> <span class="o">(</span><span class="nv">contact.name.first</span> <span class="o">+</span> <span class="s2">&quot; &quot;</span> <span class="o">+</span> <span class="nv">contact.name.last</span><span class="o">)|</span><span class="nf">length</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{{</span> <span class="nv">contact.name.first</span> <span class="cp">}}</span><span class="x"> </span><span class="cp">{{</span> <span class="nv">contact.name.last</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{{</span> <span class="s2">&quot;#&quot;</span> <span class="o">*</span> <span class="o">(</span><span class="nv">contact.name.first</span> <span class="o">+</span> <span class="s2">&quot; &quot;</span> <span class="o">+</span> <span class="nv">contact.name.last</span><span class="o">)|</span><span class="nf">length</span> <span class="cp">}}</span><span class="x"></span>

<span class="cp">{%</span> <span class="k">if</span> <span class="nv">contact.address</span> <span class="k">is</span> <span class="nf">defined</span> <span class="k">and</span> <span class="nv">contact.address.street</span> <span class="k">is</span> <span class="nf">defined</span> <span class="k">and</span> <span class="nv">contact.address.city</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">Address: </span><span class="cp">{{</span> <span class="nv">contact.address.street</span> <span class="cp">}}</span><span class="x"> – </span><span class="cp">{{</span> <span class="nv">contact.address.city</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">elif</span> <span class="nv">contact.address</span> <span class="k">is</span> <span class="nf">defined</span> <span class="k">and</span> <span class="nv">contact.address.city</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">Address: </span><span class="cp">{{</span> <span class="nv">contact.address.city</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">endif</span> <span class="cp">%}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">if</span> <span class="nv">contact.address</span> <span class="k">is</span> <span class="nf">defined</span> <span class="k">and</span> <span class="nv">contact.address.country</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">         </span><span class="cp">{{</span> <span class="nv">contact.address.country</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">endif</span> <span class="cp">%}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">if</span> <span class="nv">contact.phone</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">Phone:   </span><span class="cp">{{</span> <span class="nv">contact.phone</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">endif</span> <span class="cp">%}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">if</span> <span class="nv">contact.email</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">Email:   </span><span class="cp">{{</span> <span class="nv">contact.email</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">endif</span> <span class="cp">%}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">if</span> <span class="nv">contact.linkedin</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">Social:  linkedin.com/in/</span><span class="cp">{{</span> <span class="nv">contact.linkedin</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">endif</span> <span class="cp">%}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">if</span> <span class="nv">contact.web</span> <span class="k">is</span> <span class="nf">defined</span> <span class="cp">%}</span><span class="x"></span>
<span class="x">Web:     </span><span class="cp">{{</span> <span class="nv">contact.web</span> <span class="cp">}}</span><span class="x"></span>
<span class="cp">{%</span> <span class="k">endif</span> <span class="cp">%}</span><span class="x"></span>


<span class="x">Summary</span>
<span class="x">=======</span>

<span class="cp">{{</span> <span class="nv">summary</span><span class="o">|</span><span class="nf">hard_wrap</span><span class="o">(</span><span class="nv">WIDTH</span><span class="o">)</span> <span class="cp">}}</span><span class="x"></span>


<span class="x">Education</span>
<span class="x">=========</span>

<span class="cp">{{</span> <span class="nv">entrylist</span><span class="o">(</span><span class="nv">education</span><span class="o">)</span> <span class="cp">}}</span><span class="x"></span>

<span class="x">Publications &amp; Presentations</span>
<span class="x">============================</span>

<span class="cp">{%</span> <span class="k">for</span> <span class="nv">item</span> <span class="k">in</span> <span class="nv">publications</span> <span class="cp">%}</span><span class="x"></span>
<span class="cp">{{</span> <span class="nv">item.text</span><span class="o">|</span><span class="nf">hard_wrap</span><span class="o">(</span><span class="nv">WIDTH</span><span class="o">,</span><span class="m">0</span><span class="o">,</span><span class="s2">&quot;&quot;</span><span class="o">,</span><span class="m">2</span><span class="o">)</span> <span class="cp">}}</span><span class="x"></span>

<span class="cp">{%</span> <span class="k">endfor</span> <span class="cp">%}</span><span class="x"></span>
</code></pre></div>

<h2 id="more-information">More Information</h2>

<p>If you&rsquo;d like to see the specifics of usage, dependencies, license, etc., see
the <a href="https://github.com/jturner314/resume">GitHub page</a>. The project is <a href="https://github.com/jturner314/resume/blob/master/LICENSE.rst">published under fairly open licenses</a>, so
you can adapt it for your own use.</p>

<p>You can view the <a href="https://jim.turner.link/resume/">Markdown output for my resume</a>
integrated into this site, or you can
download <a href="https://jim.turner.link/downloads/resume-web.pdf">my resume as PDF</a>
or <a href="https://jim.turner.link/downloads/resume-web.txt">my resume as plain text</a>.</p>