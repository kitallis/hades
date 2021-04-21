+++
title = "hyper alpha supports async/await"
date = "2019-09-04T11:53:52-07:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/187493499882"
+++
<p>I&rsquo;m excited to announce the <a href="https://github.com/hyperium/hyper/releases/tag/v0.13.0-alpha.1">first alpha</a> of hyper 0.13. <a href="https://hyper.rs">hyper</a> is a maturing HTTP library written in Rust, already one of the fastest out there, and trusted by many for its correctness.</p>

<p>This alpha release brings support for the new <a href="https://doc.rust-lang.org/std/future/trait.Future.html"><code>std::future::Future</code></a>. The reason this is so exciting is that this allows using the new <code>async</code>/<code>await</code> syntax that will be stabilizing in Rust 1.39.</p>

<h3>Example</h3>

<p>The follow example shows how one can use <code>async</code>/<code>await</code> to dump a response to the console:</p>

<pre><code class="rust">#[tokio::main]
async fn main() -&gt; Result&lt;(), Error&gt; {
    let client = Client::new();

    let resp = client.get("http://httpbin.org/ip".parse()?).await?;
    println!("Status: {}", resp.status());
    println!("Headers: {:#?}\n", resp.headers());

    while let Some(chunk) = resp.body_mut().next().await {
        stdout().write_all(&amp;chunk?)?;
    }

    Ok(())
}
</code></pre>

<p>The same <code>async</code>/<code>await</code> style can be used for writing servers as well!</p>

<h3>Changes to come</h3>

<p>Besides the change from <code>futures</code> 0.1 to <code>std::future::Future</code> and how writing code with <code>async</code>/<code>await</code>, much of hyper&rsquo;s API will feel very similar. Still, there a some technically breaking changes that will be included in the 0.13 as well.</p>

<h3>Embracing <code>tower::Service</code></h3>

<p>During hyper 0.12, servers were defined via the <code>hyper::service::Service</code> trait. Since then, we&rsquo;ve been working hard on a general <code>Service</code> interface, and building <a href="https://github.com/tower-rs/tower">powerful middleware</a> that can utilize it. Our hope is that eventually, applications can be generic over <a href="https://docs.rs/tower-service/0.3.0-alpha.1/tower_service/trait.Service.html"><code>Service</code></a> and the <code>http</code> types, and a user could choose their backend that plugs right in (such as hyper).</p>

<p>Consider a small example that handles many mundane things for you:</p>

<pre><code class="rust">let svc = ServiceBuilder::new()
    // Reject the request early if concurrency limit is hit
    .load_shed()
    // Only allow 1,000 requests in flight at a time
    .concurrency_limit(1_000)
    // Cancel requests that hang too long
    .timeout(Duration::from_secs(30))
    // All wrapped around your application logic
    .service(your_app_service);
</code></pre>

<p>The <code>tower::Service</code> trait easily allows everyone to power up their servers and clients!</p>

<h3>Alpha One</h3>

<p>This first alpha is to allow people to try out writing HTTP servers and clients using the new <code>async</code>/<code>await</code> syntax. All the features from 0.12 work in this release. However, not all the API changes have been finalized, so as with other alphas, there will likely be breakage between alpha releases as we fine tune things.</p>

<p>But for now, get your fresh copy of hyper <a href="https://github.com/hyperium/hyper/releases/tag/v0.13.0-alpha.1">v0.13.0-alpha.1 here</a>!</p>

<pre><code class="toml">[dependencies]
hyper = "=0.13.0-alpha.1"
</code></pre>