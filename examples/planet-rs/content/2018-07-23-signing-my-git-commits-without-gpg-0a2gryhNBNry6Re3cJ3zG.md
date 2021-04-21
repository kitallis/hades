+++
title = "Signing my git commits without GPG"
date = "2018-07-23T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/signing-commits-without-gpg/"
+++
Unlike most git users, I try to sign my commits. Unfortunately, the only way to do this right now is to use PGP signatures, because that is all that git is able to integrate with. This has meant that in practice I have to use GPG if I want to sign my commits, an experience I do not relish. Last week, I wrote a program to replace GPG for that purpose.