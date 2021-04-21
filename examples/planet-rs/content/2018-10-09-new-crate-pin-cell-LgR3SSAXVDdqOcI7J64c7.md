+++
title = "New crate: pin-cell"
date = "2018-10-09T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/pin-cell/"
+++
Today I realized a new crate called pin-cell. This crate contains a type called PinCell, which is a kind of cell that is similar to RefCell, but only can allow pinned mutable references into its interior. Right now, the crate is nightly only and no-std compatible.
How is the API of PinCell different from RefCell? When you call borrow_mut on a RefCell, you get a type back that implements DerefMut, allowing you to mutate the interior value.