+++
title = "Async Methods I: generic associated types"
date = "2018-05-31T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/async-methods-i/"
+++
Async/await continues to move along swimmingly. We&rsquo;ve accepted an RFC describing how the async/await syntax will work in Rust, and work is underway on implementing support for it in the compiler. We&rsquo;re hopeful that users will be able to start experimenting with the syntax on nightly by early July.
The RFC for async/await didn&rsquo;t address one important thing: async methods. It is very important for people defining libraries to be able to define traits that contain async functions, like this: