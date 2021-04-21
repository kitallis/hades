+++
title = "This Week In Servo 129"
date = "2019-04-30T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2019/04/30/twis-129/"
+++
<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2019-04-22..2019-04-30
+user%3Aservo+">past week</a>,
we merged 68 PRs in the Servo organization’s repositories.</p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2019.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>ferjm is <a href="https://github.com/servo/servo/pull/23208">adding media controls</a> to video elements.</li>
  <li>Manishearth is implementing the <a href="https://github.com/servo/servo/pull/23292">WebXR input APIs</a>.</li>
  <li>mmatyas is adding support for <a href="https://github.com/servo/servo/pull/23226">compressed WebGL textures</a>.</li>
</ul>

<h3 id="notable-additions">Notable Additions</h3>

<ul>
  <li>ferjm <a href="https://github.com/servo/servo/pull/22743">implemented</a> enough Shadow DOM support to build user agent widgets include media controls.</li>
  <li>miller-time <a href="https://github.com/servo/servo/pull/23090">standardized</a> the use of referrers in fetch requests.</li>
  <li>krk <a href="https://github.com/servo/servo/pull/23200">added</a> a build-time validation that the DOM inheritance hierarchy matches the WebIDL hierarchy.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/23233">redesigned</a> part of the embedding API to separate per-window from per-application APIs.</li>
  <li>AZWN <a href="https://github.com/servo/servo/pull/23253">created</a> an API for using the type system to represent important properties of the JS engine.</li>
  <li>Akhilesh1996 <a href="https://github.com/servo/servo/pull/23259">implemented</a> the setValueCurveAtTime Web Audio API.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/23256">transitioned</a> the Windows build to rely on clang-cl instead of the MSVC compiler.</li>
  <li>snarasi6 <a href="https://github.com/servo/servo/pull/23279">implemented</a> the setPosition and setOrientation Web Audio APIs.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li><a href="https://github.com/Akhilesh1996">Akhilesh V</a></li>
  <li><a href="https://github.com/swarnimarun">Swarnim Arun</a></li>
  <li><a href="https://github.com/snarasi6">snarasi6</a></li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>