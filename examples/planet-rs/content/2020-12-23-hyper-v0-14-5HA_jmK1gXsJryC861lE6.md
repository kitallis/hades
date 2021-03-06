+++
title = "hyper v0.14"
date = "2020-12-23T11:00:11-08:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/638320652536922112"
+++
<p>hyper v0.14
I&rsquo;m so thrilled to announce the release of <a href="https://github.com/hyperium/hyper/releases/tag/v0.14.0">hyper v0.14</a> today. <a href="https://hyper.rs">hyper</a> is a safe, reliable, fast HTTP library written in Rust. It provides asynchronous client and server APIs, supporting both HTTP/1 and HTTP/2.</p>

<h3>What&rsquo;s this release all about?</h3>

<h4>Tokio 1.0</h4>

<p>Today also sees the release of <a href="https://tokio.rs/blog/2020-12-tokio-1-0">Tokio v1.0</a>. We wanted to ensure that with that momentous release, hyper was ready immediately so the ecosystem can start using it right away. This change doesn&rsquo;t have much of an outward effect on hyper&rsquo;s API, but is a big step towards stabilizing the Rust networking experience.</p>

<h4>Optional Main Features</h4>

<p>hyper has had optional features for a while, such as optional support for the Tokio. This allows convenient usage with the excellent Tokio runtime, while allowing others who have an existing runtime to just use hyper for its HTTP bits.</p>

<p>In this release, however, we&rsquo;ve made the main pieces of hyper optional features.</p>

<ul><li><code>http1</code></li>
<li><code>http2</code></li>
<li><code>client</code></li>
<li><code>server</code></li>
</ul><p>These features cover the largest parts of hyper, its protocol implementations. With these now able to be enabled or disabled, you can significantly reduce the amount of code you compile and include in hyper. For example, if you&rsquo;re application always talks to an HTTP/1 server, you can enable just the <code>http1</code> and <code>client</code> features. Or <a href="https://github.com/hyperium/tonic">Tonic</a>, a Rust gRPC library, since it is based on HTTP/2, it can disable the HTTP/1 code in hyper, enabling only <code>http2</code>, <code>client</code>, and <code>server</code>.</p>

<h3>What&rsquo;s next?</h3>

<p>This release is timed to go out with <a href="https://tokio.rs/blog/2020-12-tokio-1-0">Tokio v1.0</a>, and notably isn&rsquo;t hyper 1.0 yet. That means we&rsquo;re not done. Here&rsquo;s what we have planned coming soon:</p>

<h4>Stability</h4>

<p>We have our eyes set on making hyper stable, with a 1.0 release. We know many people rely on hyper to make fast, reliable HTTP applications and libraries, and it&rsquo;s a top priority to make hyper stable and no longer need breaking changes.</p>

<p>Stability is also meant in terms of reliability: we&rsquo;ll continue to fix bugs, improve performance, and undertake internal refactors that shouldn&rsquo;t affect the API, but will result in more and more reliable HTTP for users.</p>

<h4>Simplification</h4>

<p>As part of the drive to stabilization and 1.0, we will be making strides towards simplifying the public API exposed in hyper. Simplifying things can mean different things, so for hyper, it refers to removing the more opinionated pieces. That means aiming to embrace the &ldquo;low level&rdquo; aspect of hyper, and moving more &ldquo;high level&rdquo; pieces into other libraries.</p>

<h4>C API</h4>

<p>Stabilizing and simplifying doesn&rsquo;t mean nothing new is coming. New things are being worked on <em>right now</em>, but with great care to not disrupt the stabilization efforts.</p>

<p>hyper is a <em>safe</em>, correct, fast HTTP implementation written in Rust, and as it matures, the desire to use that safety in places that <em>aren&rsquo;t</em> Rust grows. We&rsquo;ve been working on a <a href="https://github.com/hyperium/hyper/pull/2278">C API</a> for hyper, with the first main user being the <a href="https://daniel.haxx.se/blog/2020/10/09/rust-in-curl-with-hyper/">curl</a> project. We&rsquo;re so excited to be part of this effort for a <a href="https://www.abetterinternet.org/post/memory-safe-curl/">memory safe &lsquo;curl&rsquo;</a>, and you can already experimentally <a href="https://github.com/curl/curl/wiki/Hyper">try it out</a>!</p>

<p>To re-emphasize the stability and simplification goals, the first parts of this C API are based on the &ldquo;lower level&rdquo; client connection API in hyper. As hyper is used throughout curl&rsquo;s extensive test suite, hyper&rsquo;s Rust internals handle more edge cases and protocol features, which is a <strong>win for all hyper users</strong>, even if you don&rsquo;t use the thin C API part.</p>

<h4>HTTP/3</h4>

<p>In the greater Internet, work has been progressing on stabilizing QUIC and the next version of HTTP, HTTP/3. We&rsquo;ve started an effort on the <a href="https://github.com/hyperium/h3"><code>h3</code></a> library, which provides HTTP/3 while being generic over a QUIC implementation. There&rsquo;s been great progress, with a working client example already in the repository.</p>

<h3>Come join us!</h3>

<p>This release is an important step for hyper. But as you can see, there&rsquo;s still plenty to be done. If you find any issues, please <a href="https://github.com/hyperium/hyper/issues">report them</a>. If you&rsquo;d to help, you can look at the <a href="https://github.com/hyperium/hyper/issues">issue board</a> and pick something to work on or discuss, and you can join the <a href="https://discord.gg/kkwpueZ">chat</a>.</p>

<p>hyper is made through the efforts of many <a href="https://github.com/hyperium/hyper/graphs/contributors">contributors</a>, and their efforts are greatly appreciated. Together, today, we release <a href="https://github.com/hyperium/hyper/releases/tag/v0.14.0">hyper v0.14</a>, and look forward to what comes next!</p>