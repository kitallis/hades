+++
title = "warp v0.1.10"
date = "2018-12-18T09:28:15-08:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/181223452087"
+++
<p><a href="https://seanmonstar.com/post/176530511587/warp">warp</a> is a breakthrough server web framework for the Rust language.</p>

<p>Today sees the 11th release of <a href="https://github.com/seanmonstar/warp">warp</a>, v0.1.10! I wanted to show off the new features, and highlight some of the amazing work that has appeared since the initial announcement.</p>

<h3>v0.1.10</h3>

<ul><li><strong>TLS Support</strong>: there is now optional support TLS, enabled via the <code>tls</code> feature.</li>
<li><strong>CORS</strong>: There is a &ldquo;wrapping&rdquo; filter (warp&rsquo;s idea of middleware) that can provide CORS to any existing <code>Filter</code>.</li>
<li>Retrieving the remote address.</li>
<li><strong>Websocket test helpers</strong>: testing filters has always been easy thanks to <code>warp::test</code>, and now, <code>warp::test::ws</code> allows for easy testing of Websocket routes specifically.</li>
</ul><h3>Previously</h3>

<p>In case you missed it, some highlights of work that has landed before v0.1.10:</p>

<ul><li>Rejection system clarity: warp initially had a rejection system that would try to automatically translate rejections into HTTP responses. It wasn&rsquo;t that scalable. The rejection system now is simply errors for why a request failed, and <code>Filter::recover</code> can be used to translate those into specific HTTP responses.</li>
<li><code>warp::fs</code> filters automatically support Conditional and Range requests, and try to use the OS blocksize for improved performance all around.</li>
<li>Streaming request and response bodies.</li>
<li>Support for custom transports besides the default TCP.</li>
<li>And many smaller improvements and new filters.</li>
</ul><h3>Next Focus: <code>Service</code></h3>

<p>When I <a href="https://seanmonstar.com/post/176530511587/warp">announced warp initially</a>, I had mentioned the <code>Service</code> trait, and the <a href="https://medium.com/@carllerche/tower-web-a-new-web-framework-for-rust-e2912856851b">tower-web</a> framework. There are still plans to see warp and tower-web merge, and current efforts have been around solidifying the <code>Service</code> trait itself.</p>

<p>As a recap, the <code>Service</code> trait is essentially some extra pieces on top of an <code>async fn(Request) -&gt; Response</code>. <strong>Our aim is that <code>Service</code> and the <a href="https://crates.io/crates/http">http crate</a> are the most basic interface that the ecosystem can gather behind.</strong> Server implementations and frameworks could be compatible with each other, as long they both just knew about <code>Service</code> and <code>http</code>.<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup></p>

<p>Being the common interface, it then becomes easier for frameworks and users to add in &ldquo;tower middleware&rdquo;, since a wrapped <code>Service</code> is still a <code>Service</code>. There are already several <a href="https://github.com/tower-rs/tower">tower</a> middlewares that have been developed in support of <a href="https://linkerd.io/2">Linkerd2</a>.</p>

<p>We recently published a new release of <code>tower-service</code>. A prototype now exists for warp to be able to convert a <code>Filter</code> directly into an HTTP <code>Service</code>. From there, we could simply run it directly via <code>hyper::Server</code>. Other HTTP server implementations that supported <code>Service</code> could theoretically also just run it, and the user would still just deal with <code>warp</code> types.</p>

<p>The future of webdev in Rust looks bright!</p>

<div class="footnotes" role="doc-endnotes">
<hr><ol><li id="fn:1" role="doc-endnote">
<p>This is similar to other languages, like WSGI, Rack, Servlet, WAI, and the like. <a href="#fnref:1" class="footnote-backref" role="doc-backlink">↩︎</a></p>
</li>

</ol></div>