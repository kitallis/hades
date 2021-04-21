+++
title = "This Week In Servo 132"
date = "2020-07-13T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2020/07/13/twis-132/"
+++
<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2020-07-06..2020-07-13+user%3Aservo+">past week</a>,
we merged 64 PRs in the Servo organization’s repositories.</p>

<p>The latest nightly builds for common platforms are available at <a href="https://download.servo.org/">download.servo.org</a>.</p>

<p>Servo now has the ability to record 360 degree videos of immersive web content like <a href="http://rawcdn.githack.com/mrdoob/three.js/master/examples/webxr_vr_rollercoaster.html">this three.js rollercoaster</a>:
<a href="http://www.youtube.com/watch?v=buivCKjTtbk" title="360 rollercoaster video"><img src="http://img.youtube.com/vi/buivCKjTtbk/0.jpg" alt="360 rollercoaster video" /></a></p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2020.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>pcwalton is <a href="https://github.com/servo/servo/pull/27216">implementing</a> support for CSS floats in the new Layout 2020 engine.</li>
  <li>AbhishekSharma102 and gterzian are <a href="https://github.com/servo/servo/pull/26710">making</a> some JS scripts compile off the main thread.</li>
  <li>cybai and jdm are <a href="https://github.com/servo/servo/pull/27026">implementing</a> dynamic module script imports.</li>
  <li>kunalmohan is <a href="https://github.com/servo/servo/projects/24">implementing</a> the draft WebGPU specification.</li>
</ul>

<h3 id="notable-additions">Notable Additions</h3>

<ul>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27229">updated</a> the developer tools server to support recent nightly Firefox versions.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27227">fixed</a> a bug preventing some DOM interfaces like XRInputSourceArray from being iterated.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27224">implemented</a> support for <a href="https://github.com/servo/webxr/pull/181">recording</a> immersive WebXR content as 360 degree videos.</li>
  <li>jdm <a href="https://github.com/rust-ammonia/rust-content-security-policy/pull/30">fixed</a> a bug causing over-zealous CSP blocking.</li>
  <li>ferjm <a href="https://github.com/servo/servo/pull/27152">implemented</a> the <code class="language-plaintext highlighter-rouge">MediaDevices.enumerateDevices</code> <a href="https://github.com/servo/media/pull/368">media</a> API.</li>
  <li>jdm <a href="https://github.com/servo/media/pull/370">removed</a> a panic when HRTF audio is requested.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27177">fixed</a> a bug preventing the <code class="language-plaintext highlighter-rouge">fxr://</code> protocol handler from working in Firefox Reality.</li>
  <li>jdm <a href="https://github.com/servo/font-kit/pull/156">worked around</a> a permission error when requesting system fonts under UWP, also avoiding a <a href="https://github.com/servo/servo/pull/27184">panic</a> when using the <code class="language-plaintext highlighter-rouge">fillText</code> API.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27142">fixed</a> a bug in GL context initialization for the gstreamer plugin that renders Servo content.</li>
  <li>mbrubeck <a href="https://github.com/servo/rust-smallvec/pull/213">avoided</a> leaking memory in the <code class="language-plaintext highlighter-rouge">smallvec</code> crate when panicking during <code class="language-plaintext highlighter-rouge">insert_many</code>.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27164">replaced</a> the websocket backend, allowing SSL websockets to work on Windows as a result.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li>No new contributors this week.</li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>