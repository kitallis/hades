+++
title = "Announcing a new project: configure"
date = "2018-01-18T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-01-18-configure/"
+++
Hi :) I&rsquo;ve been working on a project called configure, which is intended to create a uniform way to load configuration variables from the environment of the program. Specifically, the goal is to create something that libraries can rely on to allow applications to delegate decisions about how configuration is loaded to applications, without those applications having to write a lot of bespoke configuration management glue.
Storing configuration in the environment &ldquo;The 12 Factor App&rdquo; has this very good advice about managing configuration: