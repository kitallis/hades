+++
title = "This Week In Servo 130"
date = "2019-06-03T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2019/06/03/twis-130/"
+++
<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2019-04-30..2019-06-03+user%3Aservo+">past month</a>,
we merged 208 PRs in the Servo organization’s repositories.</p>

<p>Windows nightlies are temporarily <a href="https://github.com/servo/servo/issues/23348">broken</a>.</p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2019.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>ferjm is <a href="https://github.com/servo/servo/pull/23208">adding media controls</a> to video elements.</li>
  <li>ceyusa is implementing <a href="https://github.com/servo/servo/pull/23483">hardware accelerated video playback</a> for Linux.</li>
  <li>jdm is adding <a href="https://github.com/servo/servo/pull/23468">Windows arm64 support</a>.</li>
  <li>georgeroman is enabling <a href="https://github.com/servo/servo/pull/23443">automated tests for WebDriver</a>.</li>
  <li>paul is embedding Servo inside <a href="https://github.com/paulrouget/HLServo">a HoloLens app</a>.</li>
</ul>

<figure>
<img src="https://irccloud.mozilla.com/file/TfDvjrCP/s.jpg" />
<figcaption>An early success rendering a website in the HoloLens emulator.</figcaption>
</figure>

<h3 id="notable-additions">Notable Additions</h3>

<ul>
  <li>Manish <a href="https://github.com/servo/servo/pull/234850">added</a> foundations of automated testing of WebXR.</li>
  <li>Eijebong <a href="https://github.com/servo/servo/pull/23459">implemented</a> type-safe DOM APIs that interact with JS Promises.</li>
  <li>jdm and paulrouget <a href="https://github.com/servo/servo/pull/23457">upgraded</a> glutin to 0.21.</li>
  <li>maharsh312 <a href="https://github.com/servo/servo/pull/23381">implemented</a> most of the missing OffscreenCanvas APIs.</li>
  <li>ferjm <a href="https://github.com/servo/media/pull/260">added</a> support for simultaneous playback of audio and video streams.</li>
  <li>Darkspirit <a href="https://github.com/servo/servo/pull/23347">updated</a> various network security data files (HSTS, PSL, CAs).</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/22856">added</a> support for running Servo on Windows via ANGLE.</li>
  <li>Manishearth <a href="https://github.com/servo/servo/pull/23342">made</a> receiving streams through WebRTC possible.</li>
  <li>pylbrecht <a href="https://github.com/servo/servo/pull/23322">implemented</a> resource timing for synchronous network requests.</li>
  <li>jdm <a href="https://github.com/servo/rust-webvr/pull/74">fixed</a> a problem preventing transitioning into Daydream VR.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/23300">improved</a> the ergonomics of testing Magic Leap builds.</li>
  <li>codehag <a href="https://github.com/servo/servo/pull/23296">implemented</a> support for using a remote web console from Firefox with Servo’s content.</li>
  <li>PurpleHairEngineer <a href="https://github.com/servo/servo/pull/23281">implemented</a> the StereoPannerNode WebAudio API.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/23163">upgraded</a> the JavaScript engine.</li>
  <li>tdelacour <a href="https://github.com/servo/servo/pull/23272">improved</a> the type-safety of text input code that switches between UTF-8 and UTF-16 strings.</li>
  <li>ceyusa <a href="https://github.com/servo/media/pull/241">created</a> an API for providing hardware-accelerated GL video playback.</li>
  <li>mmatyas <a href="https://github.com/servo/servo/pull/23226">implemented</a> support for compressed textures in WebGL.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/21780">upgraded</a> the NDK in use for Android builds.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li><a href="https://github.com/jeremy-ir">Jeremy Ir</a></li>
  <li><a href="https://github.com/marcinwiacek">Marcin Wiącek</a></li>
  <li><a href="https://github.com/PurpleHairEngineer">Maria Sable</a></li>
  <li><a href="https://github.com/mboros1">Martin Boros</a></li>
  <li><a href="https://github.com/petemoore">Pete Moore</a></li>
  <li><a href="https://github.com/nehalem501">Tomek LECOCQ</a></li>
  <li><a href="https://github.com/will-bartlett">William Bartlett</a></li>
  <li><a href="https://github.com/ffwff">ffwff</a></li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>