+++
title = "The problem of effects in Rust"
date = "2020-04-13T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/the-problem-of-effects/"
+++
In a previous post, I shortly discussed the concept of &ldquo;effects&rdquo; and the parallels between them. In an unrelated post since then, Yosh Wuyts writes about the problem of trying to write fallible code inside of an iterator adapter that doesn&rsquo;t support it. In a previous discussion, the users of the Rust Internals forum hotly discuss the notion of closures which would maintain the so-called &ldquo;Tennant&rsquo;s Correspondence Principle&rdquo; - that is, closures which support breaking to scopes outside of the closure, inside of the function they are in (you can think of this is closures capturing their control flow environment in addition to capturing variables).