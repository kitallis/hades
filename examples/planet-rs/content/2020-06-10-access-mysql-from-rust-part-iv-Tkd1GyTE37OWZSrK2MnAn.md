+++
title = "Access MySQL from Rust Part IV"
date = "2020-06-10T21:09:08+00:00"

[extra]
author = "Bibhas Bhattacharya : mobiarch"
link = "https://mobiarch.wordpress.com/2020/06/10/access-mysql-from-rust-part-iv/"
+++
In Part III we built a module for database access. In this post we will bring the whole series to its conclusion. We will build a web service layer to expose the data access layer to the outside world. We will use the actix web framework. Add Dependencies Open Cargo.toml. Add actix-web, actix-rt and serde [&#8230;]