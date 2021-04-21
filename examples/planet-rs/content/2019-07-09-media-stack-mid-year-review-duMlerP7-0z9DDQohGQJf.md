+++
title = "Media stack Mid-Year review"
date = "2019-07-09T00:00:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2019/07/09/media-update-h1-2019/"
+++
<p>We recently closed the first half of 2019 and with that it is time to look back and do a quick summary of what the media team has achieved during this 6 months period.</p>

<p>Looking at some stats, we merged 87 Pull Requests, we opened 56 issues, we closed 42 issues and we welcomed 13 new amazing contributors to the media stack.</p>

<h3 id="av-playback">A/V playback</h3>

<p>These are some of the selected A/V playback related H1 acomplishments</p>

<h4 id="media-cache-and-improved-seeking">Media cache and improved seeking</h4>

<p>We significally <a href="https://github.com/servo/servo/pull/22692">improved</a> the seeking experience of audio and video files by implementing preloading and buffering support and a media cache.</p>

<iframe title="vimeo-player" src="https://player.vimeo.com/video/311414154" width="640" height="360" frameborder="0" allowfullscreen=""></iframe>

<h4 id="basic-media-controls">Basic media controls</h4>

<p>After a few months of work we got <a href="https://github.com/servo/servo/pull/22743">partial support for the Shadow DOM API</a>, which gave us the opportunity to implement our first basic set of <a href="https://github.com/servo/servo/pull/23208">media controls</a>.</p>

<p><img src="https://s3.amazonaws.com/media-p.slid.es/uploads/105177/images/6275339/Jun-19-2019_17-11-57.gif" alt="media controls" width="640" /></p>

<p>The UI is not perfect, among other things, because we still have no way to render a progress or volume bar properly, as that depends on the <code class="language-plaintext highlighter-rouge">input type="range"&gt;</code> layout, which so far is rendered as a simple text box instead of the usual slider with a thumb.</p>

<h4 id="gstreamer-backend-for-magicleap">GStreamer backend for MagicLeap</h4>

<p>Another great achievement by <a href="https://github.com/xclaesse">Xavier Claessens</a> from <a href="https://www.collabora.com/">Collabora</a> has been the GStreamer backend for <a href="https://www.magicleap.com/">Magic Leap</a>. The work is not completely done yet, but as you can see on the animation below, he already managed to paint a full screen video on the Magic Leap device.</p>

<p><img src="https://s3.amazonaws.com/media-p.slid.es/uploads/105177/images/6274304/Jun-19-2019_13-12-31.gif" alt="magic leap video" width="640" /></p>

<h4 id="hardware-accelerated-decoding">Hardware accelerated decoding</h4>

<p>One of the most wanted features that we have been working on for almost a year and that has recently landed is hardware accelerated decoding.</p>

<p>Thanks to the excellent and constant work from the <a href="https://www.igalia.com/">Igalian</a> <a href="https://github.com/ceyusa">Víctor Jáquez</a>, Servo recently gained <a href="https://github.com/servo/servo/pull/23483">support for hardware-accelerated media playback</a>, which means lower CPU usage, better battery life and better thermal behaviour, among other goodies.</p>

<p>We only have support on Linux and Android (EGL and Wayland) so far. Support for other platforms is on the roadmap.</p>

<video src="https://s3.amazonaws.com/media-p.slid.es/videos/105177/rzteE40V/hwacceleration.mp4" width="640" controls=""></video>

<p>The numbers we are getting are already pretty nice. You might not be able to see it clearly on the video, but the renderer CPU time for the non hardware accelerated playback is ~8ms, compared to the ~1ms of CPU time that we get with the accelerated version.</p>

<h4 id="improved-web-compatibility-of-our-media-elements-implementation">Improved web compatibility of our media elements implementation</h4>

<p>We also got a bunch of other smaller features that significantly improved the web compatibility of our media elements.</p>

<ul>
  <li><a href="https://github.com/servo/ferjm">ferjm</a> <a href="https://github.com/servo/servo/pull/22399">added</a> support for the HTMLMediaElement <code class="language-plaintext highlighter-rouge">poster</code> frame attribute</li>
  <li><a href="https://github.com/swarnimarun">swarnimarun</a> <a href="https://github.com/servo/servo/pull/23236">implemented</a> support for the HTMLMediaElement <code class="language-plaintext highlighter-rouge">loop</code> attribute</li>
  <li><a href="https://github.com/jackxbritton">jackxbritton</a> <a href="https://github.com/servo/servo/pull/23188">implemented</a> the HTMLMediaElement <code class="language-plaintext highlighter-rouge">crossorigin</code> attribute logic.</li>
  <li>Servo got the ability to <a href="https://github.com/servo/servo/pull/22347">mute and unmute</a> as well as controlling the <a href="https://github.com/servo/servo/pull/22324">volume</a> of audio and video playback thanks to <a href="https://github.com/stevesweetney">stevesweetney</a> and <a href="https://github.com/lucasfantacuci">lucasfantacuci</a>.</li>
  <li><a href="https://github.com/sreeise">sreeise</a> <a href="https://github.com/servo/servo/pull/22622">implemented</a> the AudioTrack, VideoTrack, AudioTrackList and VideoTrackList interfaces.</li>
  <li><a href="https://github.com/georgeroman">georgeroman</a> <a href="https://github.com/servo/servo/pull/22449">coded</a> the required changes to allow changing the playback rate of audio and video files.</li>
  <li><a href="https://github.com/georgeroman">georgeroman</a>, again, <a href="https://github.com/servo/media/pull/232">implemented</a> support for the HTMLMediaElement <code class="language-plaintext highlighter-rouge">canPlayType</code> function.</li>
  <li><a href="https://github.com/dlrobertson">dlrobertson</a> paved the way for timed text tracks support by implementing the basics of the <a href="https://github.com/servo/servo/pull/22392">TextTrack API</a> and the <a href="https://github.com/servo/servo/pull/22563">HTMLTrackElement interface</a>.</li>
</ul>

<h3 id="webaudio">WebAudio</h3>
<p>We also got a few additions on the WebAudio land.</p>

<ul>
  <li><a href="https://github.com/PurpleHairEngineer">PurpleHairEngineer</a> <a href="https://github.com/servo/media/pull/243">implemented</a> the StereoPannerNode backend.</li>
  <li><a href="https://github.com/collares">collares</a> <a href="https://github.com/servo/servo/pull/22648">implemented</a> the DOM side of the ChannelSplitterNode.</li>
  <li><a href="https://github.com/Akhilesh1996">Akhilesh1996</a> <a href="https://github.com/servo/servo/pull/23259">implemented</a> the AudioParam setValueCurveAtTime function.</li>
  <li><a href="https://github.com/snarasi6">snarasi6</a> <a href="https://github.com/servo/servo/pull/23279">implemented</a> the deprecated setPosition and setOrientation AudioListener methods.</li>
</ul>

<h3 id="webrtc">WebRTC</h3>
<p>Thanks to <a href="https://github.com/jdm">jdm</a>’s and <a href="https://github.com/jdm">Manishearth</a>’s work, Servo has now the foundations of a <a href="https://github.com/servo/servo/pull/23377">WebRTC implementation</a> and it is able to perform a 2-way calling with audio and video playback coming from the <a href="https://github.com/servo/servo/pull/22780">getUserMedia API</a>.</p>

<iframe src="https://player.vimeo.com/video/328247783" width="640" height="392" frameborder="0" allow="autoplay; fullscreen" allowfullscreen=""></iframe>

<h2 id="next-steps">Next steps</h2>
<p><em>That’s <strong>not</strong> all folks!</em> We have exciting plans for the second half of 2019.</p>

<h4 id="av-playback-1">A/V playback</h4>
<p>On the A/V playback land, we want to:</p>

<ul>
  <li>Focus on adding hardware accelerated playback on Windows and OSX.</li>
  <li>Add support for fullscreen playback.</li>
  <li>Add support for 360 video.</li>
  <li>Improve the existing media controls by, for instance, implementing a nicer layout for the <code class="language-plaintext highlighter-rouge">&lt;input type="range"&gt;</code> element, with a proper slider and a thumb, so we can have progress and volume bars.</li>
</ul>

<h4 id="webaudio-1">WebAudio</h4>
<p>For WebAudio there are plans to make some architectural improvements related to the timeline and the graph traversals.</p>

<p>We would also love to work on the MediaElementAudioSourceNode implementation.</p>

<h4 id="webrtc-1">WebRTC</h4>
<p>For WebRTC, data channels are on the roadmap for the second half.</p>

<p>We currently support the playback of a single stream of audio and video simultaneously, so allowing the playback of multiple simulatenous streams of each type is also something that we would like to get during the following months.</p>

<h4 id="others">Others</h4>
<p>There were also plans to implement support for a global mute feature, and I am happy to say, that <a href="https://github.com/khodzha">khodza</a> already <a href="https://github.com/servo/media/pull/271">got this done</a> right at the start of the second half.</p>

<p>Finally, we have been trying to get Youtube to work on Servo, but it turned out to be a difficult task because of non-media related issues (i.e. layout or web compatibility issues), so we decided to adjust the goal and focus on embedded Youtube support instead.</p>