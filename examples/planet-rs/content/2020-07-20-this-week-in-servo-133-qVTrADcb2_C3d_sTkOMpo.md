+++
title = "This Week In Servo 133"
date = "2020-07-20T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2020/07/20/twis-133/"
+++
<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2020-07-13..2020-07-20+user%3Aservo+">past week</a>,
we merged 62 PRs in the Servo organization’s repositories.</p>

<p>The latest nightly builds for common platforms are available at <a href="https://download.servo.org/">download.servo.org</a>.</p>

<p>We now have a collection of <a href="https://github.com/servo/servo/blob/master/docs/hololens.md">tips &amp; tricks</a> for using Firefox Reality on the HoloLens 2.</p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2020.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>pcwalton is <a href="https://github.com/servo/servo/pull/27216">implementing</a> support for CSS floats in the new Layout 2020 engine.</li>
  <li>AbhishekSharma102 and gterzian are <a href="https://github.com/servo/servo/pull/26710">making</a> some JS scripts compile off the main thread.</li>
  <li>kunalmohan is <a href="https://github.com/servo/servo/projects/24">implementing</a> the draft WebGPU specification.</li>
</ul>

<h3 id="notable-additions">Notable Additions</h3>

<ul>
  <li>jdm <a href="https://github.com/servo/webxr/pull/184">avoided</a> a GL error when exiting immersive mode.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27295">implemented</a> support for streaming from webxr content to the GStreamer plugin embedding.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27300">fixed</a> an issue with websocket HTTP requests, enabling Mozilla Hubs to load.</li>
  <li>ferjm <a href="https://github.com/servo/media/pull/373">fixed</a> a regression preventing getUserMedia and other MediaStream APIs from working.</li>
  <li>kunalmohan <a href="https://github.com/servo/servo/pull/27285">implemented</a> error scopes for WebGPU.</li>
  <li>avr1254 <a href="https://github.com/servo/servo/pull/27255">added</a> the missing <code class="language-plaintext highlighter-rouge">relList</code> DOM attribute for HTMLFormElement.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27250">improved</a> the behaviour of dismissing an on-screen keyboard in Firefox Reality.</li>
  <li>cybai <a href="https://github.com/servo/servo/pull/27026">implemented</a> support for dynamic module imports.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li><a href="https://github.com/avr1254">Arjun Ramachandrula</a></li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>