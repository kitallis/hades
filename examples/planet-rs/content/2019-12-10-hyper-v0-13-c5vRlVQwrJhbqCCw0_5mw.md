+++
title = "hyper v0.13"
date = "2019-12-10T09:48:08-08:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/189594157852"
+++
<p>After a few months of <a href="https://seanmonstar.com/post/187493499882/hyper-alpha-supports-asyncawait">alpha development</a>, the final release of <a href="https://github.com/hyperium/hyper/releases/tag/v0.13.0">hyper v0.13.0</a> is now ready! <a href="https://hyper.rs">hyper</a> is a maturing HTTP library written in Rust, already one of the <a href="https://www.techempower.com/benchmarks/#section=data-r18&amp;hw=ph&amp;test=plaintext">fastest out there</a><sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>, and trusted by many for its correctness.</p>

<p>The highlights of this release:</p>

<ul><li>Full <code>async</code>/<code>await</code> support.</li>
<li>Tokio v0.2 upgrade.</li>
<li>Adopting <code>tower::Service</code>.</li>
</ul><h3>async / await</h3>

<p>The premise of <code>async</code> and <code>await</code> in Rust is to allow writing code that uses <code>Future</code>s in a similar style to &ldquo;blocking&rdquo; code. No more combinators, no more &ldquo;callbacks&rdquo;, just slap <code>.await</code> on the end of the expression. For instance, here&rsquo;s how we can use the <code>Client</code>:</p>

<pre><code class="rust">#[tokio::main]
async fn main() -&gt; Result {
    let client = Client::new();

    let mut resp = client.get("http://httpbin.org/ip".parse()?).await?;
    println!("Status: {}", resp.status());
    println!("Headers: {:#?}\n", resp.headers());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&amp;chunk?).await?;
    }

    Ok(())
}
</code></pre>

<p>Connecting, writing the request, receiving the response, streaming the body, and writing to stdout can all be done without &ldquo;blocking&rdquo; the thread. Instead, with the use of <code>await</code>, just <em>that</em> future will make as much progress as it can without blocking, and then go to a pending state and register a notification when more progress could be made. And yet, it looks just like regular &ldquo;blocking&rdquo; code. This should <em>hugely</em> improve the ergonomics of writing server code that scales under load.</p>

<h3>Tokio v0.2</h3>

<p><a href="https://tokio.rs">Tokio</a> is a phenomenal async IO runtime for Rust, and hyper has built-in support by default. The <a href="https://tokio.rs/blog/2019-11-tokio-0-2/">Tokio v0.2 upgrade</a> includes <code>async</code>/<code>await</code> support, a <a href="https://tokio.rs/blog/2019-10-scheduler/">significant scheduler improvement</a>, and even faster compile times.</p>

<h3>Tower Services</h3>

<p><a href="https://github.com/tower-rs/tower">Tower</a> is an RPC design that builds off Twitter&rsquo;s &ldquo;your server as a function&rdquo;<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup>. It defines the base <code>Service</code> trait, which handles some request-like value, and asynchronously returns a response-like value. The <code>tower-service</code> crate is minimal, and protocol-agnostic. Our hope is others in the ecosystem can be just use <code>Service</code> and <code>http</code>, and not have to depend directly on hyper<sup id="fnref:3"><a href="#fn:3" class="footnote-ref" role="doc-noteref">3</a></sup>.</p>

<p>An additional benefit of integrating with Tower is being able to make use of many of the <a href="https://github.com/tower-rs/tower">middleware</a> we&rsquo;ve already developed.</p>

<ul><li><p>Server middleware:</p>

<pre><code class="rust">let svc = ServiceBuilder::new()
  // Reject the request early if concurrency limit is hit
  .load_shed()
  // Only allow 1,000 requests in flight at a time
  .concurrency_limit(1_000)
  // Cancel requests that hang too long
  .timeout(Duration::from_secs(30))
  // All wrapped around your application logic
  .service(your_app_service);
</code></pre></li>
<li><p>Or wrapping a Client:</p>

<pre><code class="rust">let svc = ServiceBuilder::new()
  // Retry requests depending on the responses or errors
  .retry(your_retry_policy)
  // Cancel when the server takes too long
  .timeout(Duration::from_secs(5)
  // Load balance using P2C
  .layer(p2c::peak_ewma(dns_resolver))
  // Wrapping a hyper::Client
  .service(hyper_client);
</code></pre></li>
</ul><p>Additionally, most <code>async fn(In) -&gt; Out</code> things in hyper are now just a <code>Service</code>. This means you can easily add these middleware to custom resolvers or connectors, for instance. Uses include adding a timeout or whitelist to a resolver.</p>

<h3>v0.13.0</h3>

<p>This probably the most exciting release of hyper yet. It&rsquo;s all thanks to the 30+ contributors tireless efforts this release that we&rsquo;ve gotten this far. Our production users continue to help us improve hyper&rsquo;s correctness, performance, and features. The current goal is that we can finish up the remaining design questions and release hyper 1.0 in the middle of 2020.</p>

<p>To see even more details, check out the <a href="https://github.com/hyperium/hyper/releases/tag/v0.13.0">v0.13.0 changelog</a>!</p>

<div class="footnotes" role="doc-endnotes">
<hr><ol><li id="fn:1" role="doc-endnote">
<p>Always take benchmarks with a carton of salt. <a href="#fnref:1" class="footnote-backref" role="doc-backlink">↩︎</a></p>
</li>

<li id="fn:2" role="doc-endnote">
<p><a href="https://monkey.org/~marius/funsrv.pdf">&ldquo;Your Server as a Function&rdquo; (PDF)</a> <a href="#fnref:2" class="footnote-backref" role="doc-backlink">↩︎</a></p>
</li>

<li id="fn:3" role="doc-endnote">
<p>This is similar to Python&rsquo;s WSGI, Ruby&rsquo;s Rack, and Java&rsquo;s Servlet. <a href="#fnref:3" class="footnote-backref" role="doc-backlink">↩︎</a></p>
</li>

</ol></div>