+++
title = "Understanding Lifetime in Rust – Part II"
date = "2015-07-08T19:34:41+00:00"

[extra]
author = "Bibhas Bhattacharya : mobiarch"
link = "https://mobiarch.wordpress.com/2015/07/08/understanding-lifetime-in-rust-part-ii-3/"
+++
In Part I we discussed the motivation behind lifetime management in Rust and how it works from a function. In this installment we will explore how lifetime helps us model containment relationship (that is, when an object contains a reference to another object). The Business Requirement We will design a Person type. A person may [&#8230;]