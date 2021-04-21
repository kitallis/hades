+++
title = "One Year With Rust"
date = "2017-02-25T00:00:00+00:00"

[extra]
author = "Vitiral"
link = "http://vitiral.github.com/2017/02/25/one-year-with-rust.html"
+++
<h1 id="i-wrote-a-full-featured-application-in-rust-and-so-can-you">I wrote a full featured application in rust, and so can you!</h1>

<p>This past year has been one of the most exciting years of programming for me,
and a large part of that is due to the programming language rust. It has been
nine months since I started writing <a href="https://github.com/vitiral/artifact">artifact</a>, and about a year since I
first started learning rust. Artifact is in its final beta release, and I
couldn’t possibly be happier with the rust language.</p>

<p>Before writing python at my current job as a Software Engineer in Test, I was an
Electrical Engineer who mostly had experience with C for microcontrollers.
When I first learned python it was an absolute breath of fresh air – I felt
like I could develop for every-day needs (other than microcontrollers) with
very little effort. The code was beautiful, easy to read and fairly easy to
write. I felt like there was no other language for me.</p>

<p>After writing python for more than a year as my day job however, I started to
see the rough edges of the language. Making small changes caused catastrophic
trivial problems like AttributeErrors to propagate throughout the code base.
Since our code was not well unit tested (its own problem to be sure) these
problems, coupled with a dynamic language like python, quickly became a
nightmare.</p>

<p>When I started to think about what kind of testing would need to be done to
mitigate these problems I balked: unit tests would basically have to be written
to validate not just a function but <em>callers</em> of the function to make sure they
were passing in the right type. The looseness of python was going to require a
huge amount of boilerplate and pain.</p>

<blockquote>
  <p>Note: tools like mypy would help alleviate some of these issues, and we are
currently using Pycharm’s type checking for all new code. However, there are
still problems (such as exception handling) which make you afraid when writing
python code.</p>
</blockquote>

<p>That’s when I started looking for a new languages, and the two I decided to
check out were Go and rust.</p>

<p>I have to admit, I really really <em>loved</em> the design concepts of Go. Keep it
simple, keep the number of concepts small, and try and stay out of your way as
much as possible. I wrote a bit with Go and it was exactly as it billed itself
to be: dead simple, no frills, almost no abstraction at all. Actually kind of
pythonic in a strange way, but with a type system and more robust error
handling. It stayed out of my way… but made me do everything. Although Go
contains a <code class="language-plaintext highlighter-rouge">for</code> loop it doesn’t have any concept of iterators (except the one
built into the langauge). This kind of magic bothered me at some deep level.</p>

<p>After spending a week doing the Go tutorials I decided that although I liked the
language alright it just wasn’t <em>fun</em>. I wasn’t learning anything, and if I
ever wanted to use it at a job I could easily pick up the language in a few
weeks. Possibly more importantly, my EE background chimed in – “could you write
an Operating System in this? How about code on a microcontroller?.” I have to
admit, I am the kind of person who want’s a language that is completely general
purpose and can be used for <em>any</em> task, and Go wasn’t that language. It was very
clearly designed for one specific purpose – and that wasn’t a purpose I was
very interested in.</p>

<p>That left me with rust, which had recently reached 1.0 and was <em>extremely
capable</em> of doing anything I might want it to do. Plus, it offered things which
were completely new: concurrency without fear, safety without garbage
collection and abstraction without cost. I was enthralled, and dove in quickly. I
found the borrow checker surprisingly easy to learn (much easier to learn that
trying to do an equivalent thing in C), and although passing around lifetimes
threw me for a loop for a long time (I was missing that <code class="language-plaintext highlighter-rouge">&lt;'a, 'b: 'a&gt;</code> means
that <code class="language-plaintext highlighter-rouge">'b</code> outlives <code class="language-plaintext highlighter-rouge">'a</code>) progress was good.</p>

<p>It was around that time that my work held a training on Risk Based Testing, and
my entire opinion of the software development community’s best practices were
completely torn down. I quickly realized that we should be writing down our
requirements, linking them to our design specifications and writing out the high
risk test cases for <em>every project</em>, no matter how small. Doing so was a
<em>pillar of good software development</em>, and we weren’t doing it.</p>

<p>I quickly realized why - there were no good tools for us to do it in a way that
made it easier for us to do our jobs. I wanted to do this for small projects,
so I started looking for a simple command line tool that could get the job
done… and hit a dead end. How could this be? How could such a simple and
important thing not exist? All I needed to do is name an artifact, write down a
description, and be able to link it to other artifacts. This is a simple
problem.</p>

<p>You can probably guess what happened next: I decided to create it. Hundreds of
hours of learning and programming later my tiny and simple command line tool
is now a full featured application with a whole suite of submodules
and static and hosted web-page support (I learned Elm too!). It has entered its
final beta release, with 1.0 slated for sometime next month.</p>

<p>The code base has grown to be larger than I expected, about 7000 lines of code.
I have refactored the <em>design documents</em> at least 3 times (something that was
very easy using the artifact tool) and have refactored large amounts of the
codebase several times more than that. My fairly extensive unit tests have
smoothed those efforts, but the rust language itself has been nothing less than
spectacular. People often say of rust “if it compiles, it runs” – and they
aren’t wrong. I can remove or change a critical struct or function base and
typically only have to fix the compiler errors. Adding new features is almost as
simple.</p>

<h2 id="conclusion">Conclusion</h2>

<p>In conclusion, I wrote a large and feature full application in rust and so can
you! Some say that rust’s libraries are not mature, but for these purposes they
are extremely well developed and supported. When I think about how things would
have been had I written artifact using python (a language with arguably the
<em>most</em> mature libraries) I run away scared. Every time I had refactored my
library would have been hours of grueling effort running and rerunning the
commands trying to hunt down every new bug. With rust, the compiler tells me
almost 100% of what needs to be fixed.</p>

<p>If you’re interested in simple quality development best practices, check out my
new open source book <a href="https://vitiral.gitbooks.io/simple-quality/content/">Simple Quality</a> and its associated tool <a href="https://github.com/vitiral/artifact">artifact</a>.
It has been a pleasure working on them, and I look forward to help improve the
quality best practices of the software development community for years to come.</p>