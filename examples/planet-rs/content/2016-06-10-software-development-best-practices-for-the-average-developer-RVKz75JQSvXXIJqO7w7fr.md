+++
title = "Software Development Best Practices for the Average Developer"
date = "2016-06-10T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/software,/quality/2016/06/10/best-practices.html"
+++
<p>It is hard to make good software. For a project to be successful, every team member must
bring quality with the work they do. Quality is an elusive attribute, gotten through some
combination of knowledge, discipline, tooling and planning.</p>

<p>It feels as though the ease of developing quality software has been going up. Powerful tools
and methodologies like git, unit-testing, agile and others have significantly increased
the productivity and quality of the work done by developers. By providing powerful revision
control tied with short testing cycles, it is much easier to develop software which meets
the requirements and is robust even with changing requirements.</p>

<p>There is an enourmous gap, however, in current software development practices, and that is
an almost complete failure to adhere to and develop tools for actually following best practices –
by which I mean the process of requirements gathering, developing detailed design specifications,
risk analysis and test design.</p>

<p>I blame this, primarily, on the almost complete lack of tools for <em>developers</em> to use to meet 
these needs. Most developers depend on writing their thoughts down on a wiki, including them
in code documentation or other such unreliable and untrackable means. Sure, there is some
expensive proprietary software that might help, but non of it seems to be designed for those
who hack at code in their day to day. In other words, none of it seems to be designed for
the people who are actually developing software.</p>

<p>In the same way that writing tests can help you write better code, writing and tracking your
requirements, specifications and test-designs can help you be a better software developer.
But in order to become better at following best practicies, you need to apply them
and see their value in every project you work on – from the smallest personal project to the largest
project involving a team or community of people.</p>

<p>That is why I have decided to create <a href="https://github.com/vitiral/rsk">rsk</a>, an open source
requirements tracking tool made for developers. The hope is that by creating a simple and highly
scalable tool which uses commands and formats developers are already familiar with,
developers will be able to easily follow design best-practices as often as they follow
unit testing best-practices – and they will get at least as much value from doing so.</p>

<p>Although I am fairly new to software development and even newer to quality, it is my intent
to create a wiki which both explains how to use this tool as well as educating the average
software developer about the usefullness of requirements gathering and detailed design.
My hope is that all projects (especially open source) can be well designed and that progress, 
new features and completeness can be easily tracked by anyone using a command line tool that 
is easy to become familiar with.</p>

<blockquote>
  <p>Note: <strong>rsk</strong> is still in pre-alpha, and is not considered usable. This blog post is about
why I am writing rsk.</p>
</blockquote>