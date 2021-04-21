+++
title = "reqwest v0.10"
date = "2019-12-30T10:38:35-08:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/189960517042"
+++
<p><a href="https://crates.io/crates/reqwest">reqwest</a> is a higher-level HTTP client for Rust. Let me introduce you the <a href="https://github.com/seanmonstar/reqwest/releases/tag/v0.10.0">v0.10</a> release that adds <code>async</code>/<code>await</code> support!</p>

<p>Some headline features are:</p>

<ul><li>Add <code>std::future::Future</code> support (hello <code>async</code>/<code>await</code>).</li>
<li>Add experimental WASM support.</li>
<li>Change the default client API to async, moving the previous synchronous API to <code>reqwest::blocking</code>.</li>
<li>Make many &ldquo;extra&rdquo; features optional to reduce unnecessary dependencies (<code>blocking</code>, <code>cookies</code>, <code>gzip</code>, <code>json</code>, etc).</li>
<li>Enable automatic &ldquo;system&rdquo; proxy detection.</li>
</ul><p>Here&rsquo;s a simple streaming example using the new syntax:</p>

<pre><code class="rust">async fn example() -&gt; Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {
    let mut resp = reqwest::get("https://hyper.rs").await?;

    while let Some(chunk) = resp.chunk().await? {
        stdout().write_all(&amp;chunk).await?;
    }

    Ok(())
} 
</code></pre>

<p>I want to thank all those contributing to make the best Rust HTTP client even better!</p>

<p>Take a look at the <a href="https://github.com/seanmonstar/reqwest/releases/tag/v0.10.0">changelog</a> for all the details.</p>