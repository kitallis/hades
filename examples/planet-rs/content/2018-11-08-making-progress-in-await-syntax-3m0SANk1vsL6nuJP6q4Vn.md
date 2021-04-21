+++
title = "Making progress in await syntax"
date = "2018-11-08T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/await-syntax/"
+++
One thing we&rsquo;ve left as an unresolved question so far in the matter of async/await syntax is the exact final syntax for the await operation. In the current implementation, awaits are written using a compiler plugin:
async fn foo() { await!(bar()); }  This is not because of any technical limitation: the reason we have done this is that we have not decided on the precise, final syntax for the await operation.