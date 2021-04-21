+++
title = "Disk-based algorithms: External merge-sort"
date = "2017-11-13T15:50:55+05:45"

[extra]
author = "Samrat Man Singh"
link = "https://samrat.me/posts/2017-11-13-external-sorting/"
+++
I have lately been studying how databases work under the hood. There are many problems that a database has to solve. At the moment, I&rsquo;ve been having fun exploring how data is stored on disk. Previously, I wrote about how a hashtable is stored on disk. This post is somewhat similar&ndash; I talk about how data stored on disk can be sorted.
Databases frequently have to sort data of massive sizes, sizes big enough that they don&rsquo;t fit in memory at once.