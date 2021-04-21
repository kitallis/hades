+++
title = "for await loops (Part I)"
date = "2019-04-15T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/for-await-i/"
+++
The biggest unresolved question regarding the async/await syntax is the final syntax for the await operator. There&rsquo;s been an enormous amount of discussion on this question so far; a summary of the present status of that discussion and the positions within the language team is coming soon. Right now I want to separately focus on one question which impacts that decision but hasn&rsquo;t been considered very much yet: for loops which process streams.