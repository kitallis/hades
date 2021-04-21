+++
title = "Code Snippets"
description =  "Post about code snippets"
+++

Code blocks.. with automatic syntax highlighting!

``` scala
def add(x: Int, y: Int): Int = x + y
println(add(1, 2)) // 3
```

See [the docs](https://www.getzola.org/documentation/content/syntax-highlighting/) for options.

### Shortcodes

Zola defines a few default shortcodes, including to embed a [Github gist](https://gist.github.com).

The arguments are:

- `url`: the url to the gist (mandatory)
- `file`: by default, the shortcode will pull every file from the URL unless a specific filename is requested
- `class`: a class to add to the `<div>` surrounding the iframe

Usage example:

```md
{{/* gist(url="https://gist.github.com/Keats/e5fb6aad409f28721c0ba14161644c57") */}}

{{/* gist(url="https://gist.github.com/Keats/e5fb6aad409f28721c0ba14161644c57", class="gist") */}}
```

Result example:

{{ gist(url="https://gist.github.com/Keats/e5fb6aad409f28721c0ba14161644c57") }}

