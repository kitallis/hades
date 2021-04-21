+++
title = "Nonlinear Model for Athletic Training"
date = "2017-07-19T00:00:00+00:00"

[extra]
author = "Rust — Jim Turner"
link = "https://jim.turner.link/pages/nonlinear-model-athletic-training/"
+++
<p>This was research I conducted at Duke University to:</p>

<ol>
<li>mathematically model the relationship between athletic training and
performance and</li>
<li>use the model to optimize training protocols subject to constraints.</li>
</ol>

<p>For example, the model can relate the time it takes you to cycle 10 miles (your
performance metric) to the duration and intensity of your cycling training
(your training stress). Then, using an optimization algorithm, you can create a
training protocol that maximizes your performance on race day.</p>

<p>The model we developed accounts for important physiological effects, such as
saturation and over-training, that existing linear models in the literature
didn’t capture. Our model was</p>

<div>
\begin{gather*}
  p = p_0 + f - u \\
  \dot{f} + \frac{1}{\tau_1} f^\alpha = k_1 \sigma \\
  \dot{u} + \frac{1}{\tau_2} u^\beta = k_2 \sigma
\end{gather*}
</div>

<p>where \(p\) is performance as a function of time, \(p_0\) is the
individual’s performance in an untrained state, \(f\) is fitness as a
function of time, \(u\) is fatigue as a function of time, \(σ\) is training
stress impulse as a function of time, \(τ_1\) and \(τ_2\) are time
constants, \(k_1\) and \(k_2\) are gain terms, \(α\) and \(β\) are
exponents that represent the model’s nonlinearities, and \(t\) is time. An
overdot indicates a time derivative. From a dynamical systems perspective, the
independent variable is
<span class="nobreak">\(t\),</span>
the state variables are \(f\) and
<span class="nobreak">\(u\),</span>
the initial conditions are \(f_0\) and
<span class="nobreak">\(u_0\),</span>
and the parameters are
<span class="nobreak">\(p_0\),</span>
<span class="nobreak">\(τ_1\),</span>
<span class="nobreak">\(τ_2\),</span>
<span class="nobreak">\(k_1\),</span>
<span class="nobreak">\(k_2\),</span>
<span class="nobreak">\(α\),</span>
and
<span class="nobreak">\(β\).</span></p>

<p>In addition to introducing the model, a core part of the research was using the
model to optimize training protocols subject to various realistic constraints
(such as limits on fatigue). The figure below is fig. 16 from the journal
article. It shows the result of optimizing a training protocol for an example
athlete to maximize performance after 12 weeks of training. The optimization
was subject to constraints related to the individual’s fitness and fatigue over
time. The prescribed training protocol in subfigure (c) is similar to the
accepted practice for training in preparation for a competition (initial
training progression, followed by a high-intensity phase, and then a taper).</p>

<figure>
    <img src="https://jim.turner.link/media/BHK-2017-0013_fig16.svg" style="width:28em"
    alt="A figure with three plots sharing the same horizontal axis, which is labeled “t [time]”. The bottom plot, with vertical axis “σ [stress]” shows stress increase gradually at the beginning, then remain relatively constant, then drop to almost zero at the start of the taper, and then remain near zero until the end. The same plot is overlaid with lines indicating two constraints; the fitness-based constraint is active during the gradual increase at the beginning, and the fatigue-based constraint is active during the middle. The top plot, with vertical axis “p [performance]”, shows the performance dip slightly initially, then increase at a decreasing rate for most of the time, then increase sharply at the onset of the taper, and then flatten out until the end. The middle plot, with vertical axis “u/f”, shows the ratio of fatigue to performance increase sharply at the beginning, followed by a relatively constant middle, and then a moderately rapid decrease starting at the onset of the taper. The same plot illustrates a constraint on the fatigue/fitness ratio of 0.8.">
    <figcaption>
    Optimized training schedule to maximize performance \(p\) after 12 weeks of
    training. Subfigure (a) shows the performance as a function of time \(t\)
    during training, (b) shows the ratio of fatigue \(u\) to fitness \(f\), and
    (c) shows the prescribed training stress \(σ\).
    </figcaption>
</figure>

<p>This research provides a new mathematical foundation for modeling and
optimizing athletic training protocols subject to an individual athlete’s
physiology, constraints, and performance goals. See the <a href="#abstract">abstract</a>
and <a href="https://jim.turner.link/downloads/BHK-2017-0013.pdf">journal article (PDF)</a> for more details.</p>

<h2 id="publication">Publication</h2>

<p>Turner, J. D., M. J. Mazzoleni, J. A. Little, D. Sequeira, and B. P. Mann. “A
nonlinear model for the characterization and optimization of athletic training
and performance”. <em>Biomedical Human Kinetics,</em> 9.1 (Feb. 2017), pp. 82–93.
(<a href="https://jim.turner.link/downloads/BHK-2017-0013.pdf">journal article (PDF)</a> and <a href="http://dx.doi.org/10.1515/bhk-2017-0013">journal
webpage</a>)</p>

<h2 id="abstract">Abstract</h2>

<p>This is the abstract from the journal article:</p>

<blockquote>
<p><strong>Study aim:</strong> Mathematical models of the relationship between training and
performance facilitate the design of training protocols to achieve
performance goals. However, current linear models do not account for
nonlinear physiological effects such as saturation and over-training. This
severely limits their practical applicability, especially for optimizing
training strategies. This study describes, analyzes, and applies a new
nonlinear model to account for these physiological effects.</p>

<p><strong>Material and methods:</strong> This study considers the equilibria and step
response of the nonlinear differential equation model to show its
characteristics and trends, optimizes training protocols using genetic
algorithms to maximize performance by applying the model under various
realistic constraints, and presents a case study fitting the model to human
performance data.</p>

<p><strong>Results:</strong> The nonlinear model captures the saturation and over-training effects;
produces realistic training protocols with training progression, a
high-intensity phase, and a taper; and closely fits the experimental
performance data. Fitting the model parameters to subsets of the data
identifies which parameters have the largest variability but reveals that the
performance predictions are relatively consistent.</p>

<p><strong>Conclusions:</strong> These findings provide a new mathematical foundation for
modeling and optimizing athletic training routines subject to an individual’s
personal physiology, constraints, and performance goals.</p>
</blockquote>

<p><cite>Turner, J. D., et al., Biomedical Human Kinetics, 2017.</cite></p>