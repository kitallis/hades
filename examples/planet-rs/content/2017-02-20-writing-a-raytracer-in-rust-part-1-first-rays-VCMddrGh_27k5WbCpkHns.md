+++
title = "Writing a Raytracer in Rust - Part 1 - First Rays"
date = "2017-02-20T11:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/"
+++
Hello! This is part one of a short series of posts on writing a simple raytracer in Rust. I&rsquo;ve never written one of these before, so it should be a learning experience all around.
So what is a raytracer anyway? The short version is it&rsquo;s a computer program that traces the paths of simulated rays of light through a scene to produce high-quality 3D-rendered images. Despite that, it also happens to be the simplest way to render 3D images.