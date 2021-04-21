+++
title = "This Week In Servo 134"
date = "2020-07-27T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2020/07/27/twis-134/"
+++
<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2020-07-20..2020-07-27+user%3Aservo+">past week</a>,
we merged 69 PRs in the Servo organization’s repositories.</p>

<p>The latest nightly builds for common platforms are available at <a href="https://download.servo.org/">download.servo.org</a>.</p>

<p>Servo has been successfully <a href="https://blog.mozvr.com/a-browser-plugin-for-unity/">integrated</a> into 3d Unity scenes as a 2d browser plugin.</p>

<p><a href="https://www.youtube.com/watch?v=NoEzDAYlflE"><img src="https://img.youtube.com/vi/NoEzDAYlflE/0.jpg" alt="Servo in the Unity editor" /></a></p>

<p>Our macOS nightly builds last week panicked on launch due to missing shared libraries. That issue has been fixed.</p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2020.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>philip-lamb is <a href="https://github.com/MozillaReality/servo-unity/">creating</a> a Unity embedding for Servo.</li>
  <li>pcwalton is <a href="https://github.com/servo/servo/pull/27216">implementing</a> support for CSS floats in the new Layout 2020 engine.</li>
  <li>kunalmohan is <a href="https://github.com/servo/servo/projects/24">implementing</a> the draft WebGPU specification.</li>
</ul>

<h3 id="notable-additions">Notable Additions</h3>

<ul>
  <li>AhibshekSharma102 and gterzian <a href="https://github.com/servo/servo/pull/26710">made</a> large external JS scripts compile on a background thread.</li>
  <li>pcwalton <a href="https://github.com/servo/servo/pull/27216">implemented</a> core float layout algorithms in the Layout 2020 engine.</li>
  <li>avr1254 <a href="https://github.com/servo/servo/pull/272990">standardized</a> some navigation-related algorithms for forms and anchors.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27304">made</a> the UWP devtools server use a fixed port instead of a random one.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27313">updated</a> the XRWebGLLayer interface to match the WebXR specification.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27316">fixed</a> a WebXR rendering regression on the HoloLens 2 that affected Babylon.js and Unity WebXR content.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27325">made</a> it possible for WebXR URLs opened via the <code class="language-plaintext highlighter-rouge">fxrmin://</code> protocol to launch immersive mode immediately on document load.</li>
  <li>utsavoza <a href="https://github.com/servo/mozjs/pull/256">allowed</a> SpiderMonkey to build with the most recent macOS SDK.</li>
  <li>SimonSapin <a href="https://github.com/servo/servo/pull/27385">added</a> support for <code class="language-plaintext highlighter-rouge">display: list-item</code> in the Layout 2020 engine.</li>
  <li>kunalmohan <a href="https://github.com/servo/servo/pull/27389">implemented</a> <a href="https://github.com/servo/servo/pull/27329">multiple</a> <a href="https://github.com/servo/servo/pull/27348">missing features</a> of the WebGPU specification.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27368">fixed</a> a regression reported in the UWP certification process for allowed APIs.</li>
  <li>Manishearth <a href="https://github.com/servo/servo/pull/27339">implemented</a> part of aligning flexboxes in Layout 2020.</li>
  <li>jdm <a href="https://github.com/servo/webxr/pull/186">fixed</a> a regression in exiting immersive mode.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li><a href="https://github.com/AbhishekSharma102">AbhishekSharma102</a></li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>