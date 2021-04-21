+++
title = "Failure 1.0.0 on March 15"
date = "2018-02-22T00:00:00+00:00"

[extra]
author = "woboats@gmail.com (boats)"
link = "https://boats.gitlab.io/blog/post/2018-02-22-failure-1.0/"
+++
I&rsquo;m planning to release a 1.0.0 version of failure on March 15. Once this happens, I don&rsquo;t plan to release any further breaking changes to the failure crate (though maybe someday in the distant future).
Breaking changes in 1.0 failure is in a somewhat unique position as being a significant part of the public API of other libraries that depend on it. Whether they use the Error struct or derive Fail for a custom error type, this becomes a part of the API they expose to other users.