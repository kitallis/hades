+++
title = "Writing a Raytracer in Rust - Part 3 - Reflection and Refraction"
date = "2017-03-27T00:00:00-06:00"

[extra]
author = "bheisler.github.io"
link = "https://bheisler.github.io/post/writing-raytracer-in-rust-part-3/"
+++
Hello again, and welcome to the final part of my series on writing a raytracer in Rust (Part 1, Part 2). Previously we implemented a basic raytracer which could handle diffuse shading of planes and spheres with multiple objects and multiple lights. This time, we&rsquo;ll add texturing, reflection and transparent objects.
First, I&rsquo;ve refactored the common parts of Sphere and Plane out to a separate structure. Since this post is all about handling more complex surface properties, we&rsquo;ll need a structure to represent them and avoid duplication.