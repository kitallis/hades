+++
title = "warp v0.2"
date = "2020-01-16T11:23:28-08:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/190293882502"
+++
<p><a href="https://crates.io/crates/warp">Warp</a> is a Rust web server framework focusing on composability and strongly-typed APIs.</p>

<p>Today sees the release of <a href="https://github.com/seanmonstar/warp/releases/tag/v0.2.0">v0.2</a>!</p>

<h3>Async and Await</h3>

<p>The most exciting part of this release is the upgrade to <code>std::future</code>, so you can now use <code>async</code>/<code>await</code> for cleaner flow control. Due to how warp encourages composition of filters, this is most noticeable at the &ldquo;ends&rdquo; of a filter chain, where an application is doing its &ldquo;business logic&rdquo;, converting input into actions and replies. And that&rsquo;s where most of the app code is!</p>

<h3>Services</h3>

<p>In the <a href="https://seanmonstar.com/post/176530511587/warp">original release of warp</a>, I wrote:</p>

<blockquote>
  <p>We’d like for warp to be able to make use of all the great <code>tower</code> middleware that already exists.</p>
</blockquote>

<p>As part of this release, that is now possible! Any <code>Filter</code> which returns a <code>Reply</code> can now be converted into a <code>Service</code> using <code>warp::service(filter)</code>. This means you can wrap your filters with any of the growing middlewares, as described in the <a href="https://seanmonstar.com/post/189594157852/hyper-v013">hyper v0.13 announcement</a>.</p>

<h3>Thanks</h3>

<p>This was a lot of work by <a href="https://github.com/seanmonstar/warp/graphs/contributors">over 60 new contributors</a>, including the massive <code>std::future</code> refactor by new collaborator <a href="https://github.com/jxs">@jxs</a>.</p>

<p>Be sure to check the <a href="https://github.com/seanmonstar/warp/releases/tag/v0.2.0">changelog</a> for all the goodies!</p>