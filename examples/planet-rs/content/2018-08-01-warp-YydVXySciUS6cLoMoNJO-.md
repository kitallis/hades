+++
title = "warp"
date = "2018-08-01T14:42:07-07:00"

[extra]
author = "Sean McArthur : seanmonstar"
link = "https://seanmonstar.com/post/176530511587"
+++
<p>Over the past several months, I&rsquo;ve been working a web framework in Rust. I wanted to make use of the <a href="http://seanmonstar.com/post/174480374517/hyper-v012">new hyper 0.12 changes</a>, so the framework is just as fast, is asynchronous, and benefits from all the <a href="https://twitter.com/seanmonstar/status/1024370108137857031">improvements found</a> powering <a href="https://linkerd.io">Linkerd</a>. More importantly, I wanted there to be a reason for making a new framework; it couldn&rsquo;t just be <em>yet another</em> framework with the only difference being I&rsquo;ve written it. Instead, the way this framework is used is quite different than many that exist. In doing so, it expresses a strong opinion, which might not match your previous experiences, but I believe it manages to do something really special.</p>

<p><strong>I&rsquo;m super excited to reveal <a href="https://crates.io/crates/warp">warp</a></strong>, a joint project with <a href="https://twitter.com/carllerche">@carllerche</a>.</p>

<h3>Background</h3>

<p>What makes warp different?</p>

<p>I&rsquo;ve been working on web servers for years. Before coming to Rust, I did several things in PHP, moved over to Python, and then shifted again to Nodejs. I&rsquo;ve tried <em>many</em> frameworks. I found that I often times need to configure predicates, like certain headers required, query parameters needed, etc, and sometimes, I need to configure that a set of routes should be &ldquo;mounted&rdquo; at a different path, and possibly want certain predicates there too. I noticed the concept of mounting or sub-routes or sub-resources or whatever the framework calls them didn&rsquo;t feel&hellip; natural, at least to me. It frequently felt like a secondary concept, occasionally not having all the power that a standard route does.</p>

<p>I&rsquo;ve also been working in Rust for several years now, and what kept using the language was its powerful type system<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>. The more I wrote Rust, and learned how amazing the &ldquo;fearless refactoring&rdquo; is, the more I hated working in dynamic languages (in my case, it was a large Nodejs server), as trying to refactor pieces inevitably would remind us (in production) that our supposedly comprehensive test suite still had holes in it. I wanted app-specific types to save me from shipping bugs.</p>

<p>A few months ago, I found the <a href="https://finagle.github.io/finch/">Finch</a> library in Scala, and shortly after, <a href="https://akka.io">Akka</a>, both of which instead just treat everything as a sort of function converting from input to output, and from there, you just chain together these different pieces, and they compose and reuse really well. Scala also has a powerful type system, and those frameworks embrace converting information from HTTP messages into app-specific types. I fell in <em>love</em>.</p>

<p><strong>The thing that makes warp special is its <code>Filter</code> system.</strong></p>

<h3>Filters</h3>

<p>A <code>Filter</code> in warp is essentially a function that can operate on some input, either something from a request, or something from a previous <code>Filter</code>, and returns some output, which could be some app-specific type you wish to pass around, or can be some reply to send back as an HTTP response. That might sound simple, but the exciting part is the combinators that exist on the <code>Filter</code> trait. These allow composing smaller <code>Filter</code>s into larger ones, allowing you modularize, and reuse any part of your web server.</p>

<p>Let me show you what I mean. Suppose you need to piece together data from several different places of a request before your have your domain object. Maybe an ID is a path segment, some verification is in a header, and other data is in the body.</p>

<pre><code class="rust">let id = warp::path::param();
let verify = warp::header("my-app-header");
let body = warp::body::json();
</code></pre>

<p>Each of these is a single <code>Filter</code>. We can combine them together with <code>and</code>, and then <code>map</code> the combined result to get a really natural feeling handler:</p>

<pre><code class="rust">let route = id
    .and(verify)
    .and(body)
    .map(|id: u64, ver: MyVerification, body: MyAppThingy| {
        // ...
    });
</code></pre>

<p>The above <code>route</code> is a <em>new</em> <code>Filter</code>. It has combined the results of the others, and provided their results naturally to the supplied function for <code>map</code>. Additionally, the types are enforced, cause well yea, this is Rust! If you were to change around one of the filters such that it returned a different type, the compiler would let you know that you need to adjust for that change.</p>

<p>This combining of results is <strong>smart</strong>: it is able to automatically toss results that are nothing (well, unit, so <code>()</code>), instead of passing worthless unit arguments to your handlers. So if you needed to combine a new <code>Filter</code> into this route that <strong>only</strong> checks some request values to determine if the request is valid, and otherwise returns nothing, your handler doesn&rsquo;t need to change.</p>

<p>Besides dropping units, did you notice how even though multiple results were combined together, the <code>map</code> closure received each as individual arguments? This greatly improves development, since that means that <code>id.and(verify).and(body)</code> is actually exactly the same as <code>id.and(verify.and(body))</code>, but using just tuples would have changed around the signature of the results. The <a href="https://docs.rs/warp/0.1.*/warp/filters/path/">routing documentation</a> shows more ways this is useful.</p>

<p>This concept powers everything in warp. Once you know you can match a single path segment via <code>warp::path("foo")</code>, then the idea of mounting doesn&rsquo;t need to be something special. You just have your filter chain for a set of endpoints, and simply &ldquo;and&rdquo; it with a new path filter. If your &ldquo;mount&rdquo; location needs to also gate on headers, or something else, you can just <code>and</code> those <code>Filter</code>s as well.</p>

<h3>Built-in functionality</h3>

<p>As awesome as the <code>Filter</code> system is, if warp didn&rsquo;t provide common web server features, it&rsquo;d still be annoying to work with. Thus, warp provides a bunch of built-in <code>Filter</code>s, allowing you compose the functionality you need to descibe each route or resource or sub-whatever.</p>

<ul><li>Path routing and parameter extraction</li>
<li>Header requirements and extraction</li>
<li>Query string deserialization</li>
<li>JSON and Form bodies</li>
<li>Static Files and Directories</li>
<li>Websockets</li>
<li>Access logging</li>
<li>And others, and more being added.</li>
</ul><p>The <a href="https://docs.rs/warp/0.1.*/warp/filters/">docs</a> explains how to use each, and the <a href="https://github.com/seanmonstar/warp/blob/master/examples">examples</a> go more in-depth on how to combine them to make actual web servers.</p>

<h3>tower-web</h3>

<p>A few months ago, there was mention of a web framework, <a href="https://medium.com/@carllerche/announcing-tower-a-library-for-writing-robust-network-services-with-rust-67273f052c40">tower-web</a>, that&rsquo;d be coming soon. The concept behind it is to provide a web framework built around <a href="https://github.com/tower-rs/tower">tower&rsquo;s</a> <code>Service</code> trait. That is still coming. warp is being released right now for a couple reasons:</p>

<ol><li>The <code>Filter</code> system is really awesome, as touched on above.</li>
<li>To explore some ideas before solidifying <code>tower</code> and <code>tower-web</code>. We&rsquo;d like for warp to be able to make use of all the great <code>tower</code> middleware that already exists.</li>
</ol><p>Expect to hear more about it, and how it fits with warp, soon!</p>

<h3>warp</h3>

<p>This is <a href="https://crates.io/crates/warp">warp</a> v0.1. It&rsquo;s awesome. It&rsquo;s fast. It&rsquo;s safe. It&rsquo;s correct. There&rsquo;s <a href="https://docs.rs/warp">documentation</a>, and <a href="https://github.com/seanmonstar/warp/blob/master/examples">examples</a>, and an <a href="https://github.com/seanmonstar/warp/issues">issue tracker</a> to file bugs and track progress of new <code>Filter</code>s that are coming (CORS almost ready). I want to thank those of you who tried warp out privately and sent feedback in, it was super valuable!</p>

<div class="footnotes" role="doc-endnotes">
<hr><ol><li id="fn:1" role="doc-endnote">
<p>I realize other languages also have nice type systems, but I didn&rsquo;t usually want to pay the cost associated with those languages. Rust just gives me what I want.??<a href="#fnref:1" class="footnote-backref" role="doc-backlink">??????</a></p>
</li>

</ol></div>