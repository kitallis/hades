+++
title = "Global Executors"
date = "2019-11-14T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/global-executors/"
+++
One of the big sources of difficulty on the async ecosystem is spawning tasks. Because there is no API in std for spawning tasks, library authors who want their library to spawn tasks have to depend on one of the multiple executors in the ecosystem to spawn a task, coupling the library to that executor in undesirable ways.
Ideally, many of these library authors would not need to spawn tasks at all.