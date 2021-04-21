+++
title = "Writing type parametric functions in Go"
date = "2013-04-06T12:54:00-05:00"

[extra]
author = "Andrew Gallant: BurntSushi"
link = "https://blog.burntsushi.net/type-parametric-functions-golang/"
+++
<p>Go&rsquo;s only method of compile time safe polymorphism is structural subtyping, and
this article will do nothing to change that. Instead, I&rsquo;m going to present a
package <code>ty</code> with facilities to write type parametric functions in Go that
maintain <strong>run time</strong> type safety, while also being convenient for the
caller to use.</p>