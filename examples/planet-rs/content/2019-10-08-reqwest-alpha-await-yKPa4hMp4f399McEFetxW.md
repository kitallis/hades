+++
title = "reqwest alpha.await"
date = "2019-10-08T14:05:02-07:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/188220739932"
+++
<p><a href="https://crates.io/crates/reqwest">reqwest</a> is a higher-level HTTP client for Rust. I&rsquo;m delighted to announce the first <a href="https://github.com/seanmonstar/reqwest/releases/tag/v0.10.0-alpha.1">alpha</a> release that brings <code>async</code>/<code>await</code> support!</p>

<p>Some headline features are:</p>

<ul><li>Add <code>std::future::Future</code> support (hello <code>async</code>/<code>await</code>).</li>
<li>Add experimental WASM support.</li>
<li>Change the default client API to async, moving the previous synchronous API to <code>reqwest::blocking</code>.</li>
<li>Make many &ldquo;extra&rdquo; features optional to reduce unnecessary dependencies (<code>blocking</code>, <code>cookies</code>, <code>gzip</code>, <code>json</code>).</li>
</ul><p>Hey look, a cute example using the new <code>async</code>/<code>await</code> syntax with <code>reqwest</code>:</p>

<pre><code class="rust">dbg!(reqwest::get("https://hyper.rs").await?.text().await?);
</code></pre>

<p>These alpha versions are depending on Rust 1.39, which (as of this post) aren&rsquo;t stable yet. Some other things may change in <code>reqwest</code> before the full release (can other features be made optional?), but the alphas allow others to experiment <em>now</em>.</p>

<p>My sincere thanks to all that help contribute to <code>reqwest</code>! Enjoy <code>&amp;lt;3</code></p>