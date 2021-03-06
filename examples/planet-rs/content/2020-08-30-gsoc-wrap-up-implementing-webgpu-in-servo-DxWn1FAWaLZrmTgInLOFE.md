+++
title = "GSoC wrap-up - Implementing WebGPU in Servo"
date = "2020-08-30T00:30:00+00:00"

[extra]
author = "Servo Blog"
link = "https://blog.servo.org/2020/08/30/gsoc-webgpu/"
+++
<h2 id="introduction">Introduction</h2>
<p>Hello everyone! I am Kunal(<a href="https://github.com/kunalmohan">@kunalmohan</a>), an undergrad student at Indian Institute of Technology Roorkee, India. As a part of Google Summer of Code(GSoC) 2020, I worked on implementing WebGPU in Servo under the mentorship of Mr. Dzmitry Malyshau(<a href="https://github.com/kvark">@kvark</a>). I devoted the past 3 months working on ways to bring the API to fruition in Servo, so that Servo is able to run the existing examples and pass the Conformance Test Suite(CTS). This is going to be a brief account of how I started with the project, what challenges I faced, and how I overcame them.</p>

<h2 id="what-is-webgpu">What is WebGPU?</h2>
<p>WebGPU is a future web standard, cross-platform graphics API aimed to make GPU capabilities more accessible on the web. WebGPU is designed from the ground up to efficiently map to the Vulkan, Direct3D 12, and Metal native GPU APIs. A native implementation of the API in Rust is developed in the <a href="https://github.com/gfx-rs/wgpu">wgpu project</a>. Servo implementation of the API uses this crate.</p>

<h2 id="the-project">The Project</h2>
<p>At the start of the project the implementation was in a pretty raw state- Servo was only able to accept shaders as SPIRV binary and ran just the compute example. I had the following tasks in front of me:</p>
<ul>
  <li>Implement the various <a href="https://gpuweb.github.io/gpuweb/#idl-index">DOM interfaces</a> that build up the API.</li>
  <li>Setup a proper Id rotation for the GPU resources.</li>
  <li>Integrate WebGPU with WebRender for presenting the rendering to HTML canvas.</li>
  <li>Setup proper model model for async error recording.</li>
</ul>

<p>The final goal was to be able to run the live examples at https://austineng.github.io/webgpu-samples/ and pass a fair amount of the CTS.</p>

<h2 id="implementation">Implementation</h2>
<p>Since Servo is a multi-process browser, GPU is accessed from a different process(server-side) than the one running the page content and scripts(content process). For better performance and asynchronous behaviour, we have a separate wgpu thread for each content process.</p>

<p>Setting up a proper Id rotation for the GPU resources was our first priority. I had to ensure that each Id generated was unique. This meant sharing the <a href="https://github.com/servo/servo/blob/a5a21a59addae0df6d9e050f17d44399db04fec3/components/script/dom/identityhub.rs#L56-L67">Identity Hub</a> among all threads via <code class="language-plaintext highlighter-rouge">Arc</code> and <code class="language-plaintext highlighter-rouge">Mutex</code>. For recycling the Ids, wgpu exposes an <code class="language-plaintext highlighter-rouge">IdentityHandler</code> trait that must be implemented on the server-side interface of the browser and wgpu. This facilitates the following: when wgpu detects that an object has been dropped by the user (which is some time after the actual drop/garbage collection), wgpu calls the trait methods that are responsible for releasing the Id. In our case they send a message to the content process to free the Id and make it available for reuse.</p>

<p>Implementing the DOM Interfaces was pretty straight forward. A DOM object is just an opaque handle to an actual GPU resource. Whenever a method, that performs an operation, is called on a DOM object there are 2 things to be done- convert the IDL types to wgpu types. And send a message to the server to perform the operation. Most of the validation is done within wgpu.</p>

<h2 id="presentation">Presentation</h2>
<p>WebGPU textures can be rendered to HTML canvas via <code class="language-plaintext highlighter-rouge">GPUCanvasContext</code>, which can be obtained from <code class="language-plaintext highlighter-rouge">canvas.getContext('gpupresent')</code>. All rendered images are served to WebRender as <code class="language-plaintext highlighter-rouge">ExternalImages</code> for rendering purpose. This is done via an async software presentation path. Each new <code class="language-plaintext highlighter-rouge">GPUCanvasContext</code> object is assigned a new <code class="language-plaintext highlighter-rouge">ExternalImageId</code> and a new swap chain is assigned a new <code class="language-plaintext highlighter-rouge">ImageKey</code>. Since WebGPU threads are spawned on-demand, an image handler for WebGPU is initialized at startup, stored in <code class="language-plaintext highlighter-rouge">Constellation</code>, and supplied to threads at the time of spawn. Each time <code class="language-plaintext highlighter-rouge">GPUSwapChain.getCurrentTexture()</code> is called the canvas is marked as dirty which is then flushed at the time of reflow. At the time of flush, a message is sent to the wgpu server to update the image data provided to WebRender. The following happens after this:</p>
<ul>
  <li>The contents of the rendered texture are copied to a buffer.</li>
  <li>Buffer is mapped asynchronously for read.</li>
  <li>The data read from the buffer is copied to a staging area in <a href="https://github.com/servo/servo/blob/669b16f2c054bd038b7a3c69985076607e140b7f/components/webgpu/lib.rs#L1353-L1365"><code class="language-plaintext highlighter-rouge">PresentionData</code></a>. <code class="language-plaintext highlighter-rouge">PresentationData</code> stores the data and all the required machinery for this async presentation belt.</li>
  <li>When WebRender wants to read the data, it locks on the data to prevent it from being altered during read. Data is served in the form of raw bytes.</li>
</ul>

<p>The above process is not the best one, but the only option available to us for now. This also causes a few empty frames to be rendered at the start.
A good thing, though, is that this works on all platforms and is a great fallback path while we???ll be adding hardware accelerate presentation in the future.</p>

<h2 id="buffer-mapping">Buffer Mapping</h2>
<p>When the user issues an async buffer map operation, the operation is queued on the server-side and all devices polled at a regular interval of 100ms for the same. As soon as the map operation is complete, data is read and sent to the content process where it is stored in the Heap. The user can read and edit this data by accessing it???s subranges via <code class="language-plaintext highlighter-rouge">GPUBuffer.getMappedRange()</code> which returns <code class="language-plaintext highlighter-rouge">ExternalArrayBuffer</code> pointing to the data in the Heap. On unmap, all the <code class="language-plaintext highlighter-rouge">ExternalArrayBuffer</code>s are detached, and if the buffer was mapped for write, data sent back to server for write to the actual resource.</p>

<h2 id="error-reporting">Error Reporting</h2>
<p>To achieve maximum efficiency, WebGPU supports an asynchronous error model. The implementation keeps a stack of <code class="language-plaintext highlighter-rouge">ErrorScope</code>s that are responsible for capturing the errors that occur during operations performed in their scope. The user is responsible for pushing and popping an <code class="language-plaintext highlighter-rouge">ErrorScope</code> in the stack. Popping an <code class="language-plaintext highlighter-rouge">ErrorScope</code> returns a promise that is resolved to null if all the operations were successfull, otherwise it resolves to the first error that occurred.</p>

<p>When an operation is issued, <code class="language-plaintext highlighter-rouge">scope_id</code> of the <code class="language-plaintext highlighter-rouge">ErrorScope</code> on the top of the stack is sent to the server with it and operation-count of the scope is incremented. The result of the operation can be described by the enum-</p>

<div class="language-rust highlighter-rouge"><div class="highlight"><pre class="highlight"><code><span class="k">pub</span> <span class="k">enum</span> <span class="n">WebGPUOpResult</span> <span class="p">{</span>
    <span class="nf">ValidationError</span><span class="p">(</span><span class="nb">String</span><span class="p">),</span>
    <span class="n">OutOfMemoryError</span><span class="p">,</span>
    <span class="nb">Success</span><span class="p">,</span>
<span class="p">}</span>
</code></pre></div></div>

<p>On receiving the result, we decrement the operation-count of the <code class="language-plaintext highlighter-rouge">ErrorScope</code> with the given <code class="language-plaintext highlighter-rouge">scope_id</code>. We further have 3 cases:</p>
<ul>
  <li>The result is <code class="language-plaintext highlighter-rouge">Success</code>. Do nothing.</li>
  <li>The result is an error and the <code class="language-plaintext highlighter-rouge">ErrorFilter</code> matches the error. We record this error in the <a href="https://github.com/servo/servo/blob/669b16f2c054bd038b7a3c69985076607e140b7f/components/script/dom/gpudevice.rs#L85-L91"><code class="language-plaintext highlighter-rouge">ErrorScopeInfo</code></a>, and if the <code class="language-plaintext highlighter-rouge">ErrorScope</code> has been popped by the user, resolve the promise with it.</li>
  <li>The result is an error but the <code class="language-plaintext highlighter-rouge">ErrorFilter</code> does not match the error. In this case, we find the nearest parent <code class="language-plaintext highlighter-rouge">ErrorScope</code> with the matching filter and record the error in it.</li>
</ul>

<p>After the result is processed, we try to remove the <code class="language-plaintext highlighter-rouge">ErrorScope</code> from the stack- the user should have called <code class="language-plaintext highlighter-rouge">popErrorScope()</code> on the scope and the operation-count of the scope should be 0.</p>

<p>In case there are no error scopes on the stack or if <code class="language-plaintext highlighter-rouge">ErrorFilter</code> of none of the <code class="language-plaintext highlighter-rouge">ErrorScope</code>s match the error, the error is fired as an <a href="https://gpuweb.github.io/gpuweb/#gpuuncapturederrorevent"><code class="language-plaintext highlighter-rouge">GPUUncapturedErrorEvent</code></a>.</p>

<h2 id="conformance-test-suite">Conformance Test Suite</h2>
<p>Conformance Test Suite is required for checking the accuracy of the implementation of the API and can be found <a href="https://github.com/gpuweb/cts">here</a>. Servo vendors it???s own copy of the CTS which, currently, needs to be updated manually for the latest changes. Here are a few statistics of the tests:</p>
<ul>
  <li>14/36 pass completely</li>
  <li>5/36 have majority of subtests passing</li>
  <li>17/36 fail/crash/timeout</li>
</ul>

<p>The wgpu team is actively working on improving the validation.</p>

<h2 id="unfinished-business">Unfinished business</h2>
<p>A major portion of the project that was proposed has been completed, but there???s still work left to do. These are a few things that I was unable to cover under the proposed timeline:</p>
<ul>
  <li>Profiling and benchmarking the implementation against the WebGL implementation of Servo.</li>
  <li>Handle canvas resize event smoothly.</li>
  <li>Support Error recording on Workers.</li>
  <li>Support WGSL shaders.</li>
  <li>Pass the remaining tests in the CTS.</li>
</ul>

<h2 id="important-links">Important Links</h2>
<p>The WebGPU specification can be found <a href="https://gpuweb.github.io/gpuweb/">here</a>.
The PRs that I made as a part of the project can be accessed via the following links:</p>
<ul>
  <li><a href="https://github.com/servo/servo/pulls?q=is%3Apr+author%3Akunalmohan+created%3A%3E2020-05-05+merged%3A%3C2020-08-31+">Servo</a></li>
  <li><a href="https://github.com/gfx-rs/wgpu/pulls?q=is%3Apr+author%3Akunalmohan+created%3A%3E2020-05-05+merged%3A%3C2020-08-31+">wgpu</a></li>
  <li><a href="https://github.com/gpuweb/gpuweb/pulls?q=is%3Apr+author%3Akunalmohan+created%3A%3E2020-05-05+merged%3A%3C2020-08-31+">WebGPU Specification</a></li>
</ul>

<p>The progress of the project can be tracked in the <a href="https://github.com/servo/servo/projects/24">GitHub project</a></p>

<h2 id="conclusion">Conclusion</h2>
<p>WebGPU implementation in Servo supports all of the <a href="https://austineng.github.io/webgpu-samples/">Austin???s samples</a>. Thanks to CYBAI and Josh, Servo now supports dynamic import of modules and thus accept GLSL shaders. Here are a few samples of what Servo is capable of rendering at 60fps:</p>

<p><img src="/images/webgpu-fractal-cube.gif" alt="Fractal Cube" /></p>

<p><img src="/images/webgpu-instanced-cube.gif" alt="Instanced Cube" /></p>

<p><img src="/images/webgpu-compute-boids.gif" alt="Compute Boids" /></p>

<p>I would like to thank Dzmitry and Josh for guiding me throughout the project and a big shoutout to the WebGPU and Servo community for doing such awesome work! I had a great experience contributing to Servo and WebGPU. I started as a complete beginner to Rust, graphics and browser internals, but learned a lot during the course of this project. I urge all WebGPU users and graphics enthusiasts out there to test their projects on Servo and help us improve the implementation and the API as well :)</p>