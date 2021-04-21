+++
title = "http v0.2"
date = "2019-12-02T14:05:38-08:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/189439210962"
+++
<p><a href="https://users.rust-lang.org/t/announcing-the-http-crate/12123">A couple years ago</a>, we released the beginning of the <code>http</code> crate. It&rsquo;s purpose was to allow a common API for the ecosystem to interact with HTTP types, without those types referring to a specific implementation. We&rsquo;ve seen great things sprout up since then!</p>

<p>Today marks the <a href="https://github.com/hyperium/http/releases/tag/v0.2.0">0.2 release</a>, a chance to make some minor breaking changes, with the hopes that this 0.2 version can soon just be promoted to 1.0. So, what has changed?</p>

<h4>HTTP/3</h4>

<p>A seemingly simple change is adding the <code>Version::HTTP_3</code> constant. However, we couldn&rsquo;t add it in 0.1 due to an unexpected compiler behavior that allowed exhaustive matching on the <code>Version</code> constants even though the internal <code>enum</code> wasn&rsquo;t exposed. This time, we&rsquo;ve made sure to prevent exhaustive matches, so we can add new versions in the future.</p>

<h4>Builders are now by-value</h4>

<p>There are some pretty useful builders to construct a <code>Request</code>, <code>Response</code>, or <code>Uri</code>. In 0.1, they were &ldquo;by-reference&rdquo; builders, meaning that each builder method took <code>&amp;mut self</code> and returned <code>&amp;mut Builder</code>. Now, they take <code>self</code> and return <code>Builder</code>. There&rsquo;s pros and cons for each pattern, but the weightiest one that made us change was the nature of &ldquo;consuming&rdquo; the builder once finished. To &ldquo;build&rdquo; a &ldquo;by-ref&rdquo; builder requires that either the data inside be cloned, or the builder be left in a &ldquo;don&rsquo;t build me again&rdquo; state. This change now makes it clearer that a builder cannot used again, as it will now be a compiler error.</p>

<h4>Reduced public dependencies</h4>

<p>To help meet the goal of promoting to <code>http</code> v1.0, we&rsquo;ve reduced the number of public dependencies to 0. There&rsquo;s still a way to make use of <code>bytes</code> to reduce copies, but it&rsquo;s now exposed in a way that there&rsquo;s no API contract. This allows <code>http</code> to reach 1.0 even if <code>bytes</code> takes longer.</p>

<h3>Next</h3>

<p>We expect the ecosystem to start updating to <code>http</code> 0.2 so you all can have the improvements as soon as possible. For example, <a href="https://hyper.rs">hyper</a> should also be ready hopefully this week. Check the <a href="https://github.com/hyperium/http/releases/tag/v0.2.0">changelog</a> for the full details!</p>