+++
title = "This Week In Servo 131"
date = "2020-07-06T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2020/07/06/twis-131/"
+++
<p>Welcome back everyone - it’s been a year without written updates, but we’re getting this train back on track! Servo hasn’t been
dormant in that time; the biggest news was the public release of <a href="https://www.microsoft.com/en-ca/p/firefox-reality/9npq78m7nb0r?rtc=1&amp;activetab=pivot:overviewtab">Firefox Reality</a>
(built on Servo technology) in the Microsoft store.</p>

<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2020-06-30..2020-07-06+user%3Aservo+">past week</a>,
we merged 44 PRs in the Servo organization’s repositories.</p>

<p>The latest nightly builds for common platforms are available at <a href="https://download.servo.org/">download.servo.org</a>.</p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2020.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>jdm is <a href="https://github.com/servo/servo/pull/27164">replacing</a> the websocket backend.</li>
  <li>cybai and jdm are <a href="https://github.com/servo/servo/pull/27026">implementing</a> dynamic module script imports.</li>
  <li>kunalmohan is <a href="https://github.com/servo/servo/projects/24">implementing</a> the draft WebGPU specification.</li>
</ul>

<h3 id="notable-additions">Notable Additions</h3>

<ul>
  <li>SimonSapin <a href="https://github.com/servo/rust-smallvec/pull/228">fixed</a> a source of Undefined Behaviour in the <code class="language-plaintext highlighter-rouge">smallvec</code> crate.</li>
  <li>muodov <a href="https://github.com/servo/servo/pull/27174">improved</a> the compatibility of invalid form elements with the HTML specification, and <a href="https://github.com/servo/servo/pull/27100">added</a>
the missing <code class="language-plaintext highlighter-rouge">requestSubmit</code> API.</li>
  <li>kunalmohan <a href="https://github.com/servo/servo/pull/27173">implemented</a> GPUQueue APIs for WebGPU, and <a href="https://github.com/servo/servo/pull/27154">avoided</a>
synchronous updates, and <a href="https://github.com/servo/servo/pull/27126">implemented</a> the getMappedRange API for GPUBuffer.</li>
  <li>alaryso <a href="https://github.com/servo/servo/pull/27169">fixed</a> a bug preventing running tests when using rust-analyzer.</li>
  <li>alaryso <a href="https://github.com/servo/servo/pull/27163">avoided</a> a panic in pages that perform layout queries on disconnected iframes.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27149">integrated</a> virtual keyboard support for text inputs into Firefox Reality, as well as <a href="https://github.com/servo/servo/pull/27114">added</a>
support for keyboard events.</li>
  <li>Manishearth <a href="https://github.com/servo/servo/pull/27143">implemented</a> WebAudio node types for reading and writing MediaStreams.</li>
  <li>gterzian <a href="https://github.com/servo/servo/pull/27016">improved</a> the responsiveness of the browser when shutting down.</li>
  <li>utsavoza <a href="https://github.com/servo/servo/pull/27104">updated</a> the referrer policy implementation to match the updated specification.</li>
  <li>ferjm <a href="https://github.com/servo/servo/pull/26752">implemented</a> support for WebRTC data channels.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li><a href="https://github.com/muodov">Maxim Tsoy</a></li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>