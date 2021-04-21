+++
title = "Handshake Patterns"
date = "2017-01-21T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2017-01-21-handshake-patterns/"
+++
The problem: defining a &lsquo;handshake&rsquo; protocol between two traits You have a problem that decomposes in this way: you want any type which implements trait Alpha to be composable with any type which implements trait Omega&hellip;
That is, if Foo and Bar are both Alphas and Baz and Quux are both Omegas, you can compose Foo with Baz or Quux, and the same with Bar, and so on.
This is not a trivial problem.