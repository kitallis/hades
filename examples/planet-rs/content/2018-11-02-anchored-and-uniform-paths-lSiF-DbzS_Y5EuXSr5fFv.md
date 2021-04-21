+++
title = "Anchored and Uniform Paths"
date = "2018-11-02T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/anchored-uniform/"
+++
Rust 2018 is almost out the door, but there is one big decision the language team has yet to make. It has to do with the modules and paths system, so of course it is a very easy decision that no one has a strong opinion about. ;-)
In Rust 2018, we&rsquo;ll be making some big changes to how paths work to try to create a more consistent experience. The &ldquo;lodestar&rdquo; (if you will) of these changes is an idea we call &ldquo;1path:&rdquo; the idea no matter where you are in your project, whether in a use statement or normal code, a path is interpreted the same way.