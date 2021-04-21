+++
title = "This Week In Servo 135"
date = "2020-08-10T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2020/08/10/twis-135/"
+++
<p>In the <a href="https://github.com/pulls?utf8=%E2%9C%93&amp;q=is%3Apr+is%3Amerged+closed%3A2020-07-27..2020-08-10+user%3Aservo+">past two weeks</a>,
we merged 108 PRs in the Servo organization’s repositories.</p>

<p>The latest nightly builds for common platforms are available at <a href="https://download.servo.org/">download.servo.org</a>.</p>

<p>Last week we released <a href="https://www.microsoft.com/p/firefox-reality/9npq78m7nb0r">Firefox Reality v1.2</a>, which includes a smoother
developer tools experience, along with support for Unity WebXR content and self-signed SSL certificates. See the
<a href="https://github.com/servo/servo/wiki/Firefox-Reality-release-notes">full release notes</a> for more information about the new release.</p>

<p><img src="https://user-images.githubusercontent.com/27658/89798375-19568080-dafa-11ea-8bd3-d47a85741ed5.png" alt="Image of unity webxr content" /></p>

<h3 id="planning-and-status">Planning and Status</h3>

<p>Our <a href="https://github.com/servo/servo/wiki/Roadmap">roadmap</a> is available online, including the team’s plans for 2020.</p>

<p>This week’s status updates are <a href="https://build.servo.org/standups/">here</a>.</p>

<h3 id="exciting-works-in-progress">Exciting works in progress</h3>

<ul>
  <li>paulrouget is <a href="https://github.com/servo/servo/pull/27556">adding</a> bookmarks to Firefox Reality.</li>
  <li>Manishearth is <a href="https://github.com/servo/servo/pull/27488">implementing</a> basic table support in the new Layout 2020 engine.</li>
  <li>jdm is <a href="https://github.com/servo/servo/pull/27474">making</a> it easy to create builds that integrate AddressSanitizer.</li>
  <li>pcwalton is <a href="https://github.com/servo/servo/pull/27539">implementing</a> support for CSS floats in the new Layout 2020 engine.</li>
  <li>kunalmohan is <a href="https://github.com/servo/servo/projects/24">implementing</a> the draft WebGPU specification.</li>
</ul>

<h3 id="notable-additions">Notable Additions</h3>

<h5 id="new-layout-engine">New layout engine</h5>

<ul>
  <li>jdm <a href="https://github.com/servo/servo/pull/26447">made</a> <code class="language-plaintext highlighter-rouge">&lt;br&gt;</code> elements break lines.</li>
  <li>Manishearth <a href="https://github.com/servo/servo/pull/27388">implemented</a> support for the <code class="language-plaintext highlighter-rouge">clip</code> CSS property.</li>
  <li>Manishearth <a href="https://github.com/servo/servo/pull/27399">fixed</a> the behaviour of the <code class="language-plaintext highlighter-rouge">inset</code> CSS property for absolutely positioned elements.</li>
</ul>

<h4 id="non-layout-changes">Non-layout changes</h4>

<ul>
  <li>kunalmohan <a href="https://github.com/servo/servo/pull/27402">added</a> the WebGPU conformance test suite to Servo’s automated tests.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27403">improved</a> the macOS nightly GStreamer packaging.</li>
  <li>nicoabie <a href="https://github.com/servo/rust-mozjs/pull/520">made</a> a SpiderMonkey Rust binding API more resilient.</li>
  <li>utsavoze <a href="https://github.com/servo/servo/pull/27413">added</a> support for <code class="language-plaintext highlighter-rouge">mouseenter</code> and <code class="language-plaintext highlighter-rouge">mouseleave</code> DOM events.</li>
  <li>avr1254 <a href="https://github.com/servo/servo/pull/27420">removed</a> some unnecessary UTF-8 to UTF-16 conversions when interacting with SpiderMonkey.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27425">implemented</a> <code class="language-plaintext highlighter-rouge">preserveDrawbingBuffer</code> support in WebGL code.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27438">added</a> a crash reporting UI to Firefox Reality.</li>
  <li>mustafapc19 <a href="https://github.com/servo/servo/pull/27443">implemented</a> the <code class="language-plaintext highlighter-rouge">Console.clear</code> DOM API.</li>
  <li>kunalmohan <a href="https://github.com/servo/servo/pull/27447">fixed</a> a WebGPU crash related to the <code class="language-plaintext highlighter-rouge">GPUErrorScope</code> API, and <a href="https://github.com/servo/servo/pull/27536">improved</a> the reporting behaviour to match the specification.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27448">fixed</a> a source of WebGL texture corruption in WebXR.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27456">added</a> infrastructure to the GStreamer plugin to allow live-streaming 360 degree videos of Hubs rooms to Youtube.</li>
  <li>kunalmohan <a href="https://github.com/servo/servo/pull/27480">improved</a> the error reporting behaviour of the WebGPU API.</li>
  <li>asajeffrey <a href="https://github.com/servo/servo/pull/27487">update</a> the WebXR Layers implementation to match the latest specification.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27491">improved</a> the Firefox Reality preferences panel to highlight specific experimental features.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27530">fixed</a> a crash when playing media in Firefox Reality.</li>
  <li>paulrouget <a href="https://github.com/servo/servo/pull/27506">fixed</a> a source of memory corruption in the C++ embedding layer.</li>
  <li>jdm <a href="https://github.com/servo/servo/pull/27512">avoided</a> pancking when a devtools client disconnects unexpectedly.</li>
  <li>asajeffrey <a href="https://github.com/servo/webxr/pull/191">made</a> it easier to test AR web content in desktop builds.</li>
</ul>

<h3 id="new-contributors">New Contributors</h3>

<ul>
  <li><a href="https://github.com/nosark">Kyle Nosar</a></li>
  <li><a href="https://github.com/Monty0045">Wyatt Turner</a></li>
  <li><a href="https://github.com/mustafapc19">mustafapc19</a></li>
</ul>

<p>Interested in helping build a web browser? Take a look at our <a href="https://starters.servo.org/">curated list</a> of issues that are good for new contributors!</p>