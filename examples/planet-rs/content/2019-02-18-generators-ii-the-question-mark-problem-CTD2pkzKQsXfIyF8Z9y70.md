+++
title = "Generators II: The Question Mark Problem"
date = "2019-02-18T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/generators-ii/"
+++
This is my second post on the design of generators. In the first post, I outlined what an MVP of the feature would look like. In this post, I want to take a look at the first design issue for the feature: how it integrates with the ? operator.
To explain exactly what I mean, let&rsquo;s start with a specific motivating example:
// This generator yields the number of alphanumeric characters in every line // in some io::Read'able data // exact sign function declaration syntax left unspecified on purpose |data| { let mut buffered_data = BufReader::new(data); let mut string = String::new(); while buffered_data.