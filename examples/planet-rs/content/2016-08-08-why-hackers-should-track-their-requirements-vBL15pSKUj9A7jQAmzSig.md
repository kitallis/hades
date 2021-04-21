+++
title = "Why hackers should track their requirements"
date = "2016-08-08T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/developer/2016/08/08/why-hackers-should-track-their-requirements.html"
+++
<p><a href="https://en.m.wikipedia.org/wiki/Software_requirements_specification">“Creating requirements”</a> 
may not be the first two words that come to mind
when you think of a hacker, but nothing could be more important. Whether
those requirements exist entirely inside the developer’s head or span multiple directories
depends on the person and the project they are working on, but all of
us create requirements if we expect the product of our efforts to look
anything like what we were intending to create. Oftentimes, the quality
of our requirements directly corelate to the quality of our product.</p>

<p>It is for the hacker that I developed the tool <a href="https://github.com/vitiral/rst">rst</a>.
It is an ultra-simple method of writing requirements and specifications so
they can be tracked throughout the lifecycle of a project.</p>

<p>Common developer wisdom (hacker and enterprise alike) now tells us that we 
should always be using revision control, that we should always be writing unit 
tests and that we should keep good documentation – and we should, and there are a 
ton of great tools for doing exactly those things. However, those “core pillars”
of good software development miss one of the most important pieces, and that is 
writing down your requirements, creating your high level design (linked to your 
reqs), and thinking about what tests to write before you write your code.</p>

<p>All the tools that I have seen for this are Web/GUI’s that are clunky and 
difficult to integrate. They are particularily ill-suited to small projects.
It doesn’t make sense to me. If we are to become good at requirements, we need
to practice for small projects so that we know what we are doing for large ones.
This problem isn’t difficult to solve: requirements are just text that
have a few basic features:</p>

<ul>
  <li>you want to be able to name them</li>
  <li>you want to be able to describe them</li>
  <li>you want to be able to link them</li>
  <li>you want to be able to track their completeness</li>
  <li>you want to be able to revision control them</li>
</ul>

<p>These four goals are trivial to accomplish for a text based tool and
quite difficult to accomplish for a GUI tool. Why then are so many
requirements tracking tools GUI based?</p>

<p><a href="https://github.com/vitiral/rst">rst</a> chooses to be simple instead of
complicated, while still choosing to provide features users want (i.e.
viewing requirements via the web in future releases). Creating 
requirements and specifications is something all of us should be doing 
for our projects – wouldn’t it be great if it were easy to do?</p>

<p>I intend to add a “hacker’s guide to quality” tutorial to the 
<a href="https://github.com/vitiral/rst/wiki">rst wiki</a> over the next couple of months.
I would highly appreciate feedback from developers all over the world about
how they address quality in their projects, and what <strong>rst</strong> could do to
help them.</p>