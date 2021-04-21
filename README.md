# hades

a modest RSS feed aggregator (think [planetplanet](https://en.wikipedia.org/wiki/Planet_(software))\) that generates markdown files with metadata to help augment an existing SSG like [zola](https://www.getzola.org) or [middleman-blog](https://middlemanapp.com/basics/blogging). hades does not try to apply any styling or markup to the posts, it only concerns itself with aggregation and generation.

### usage

```bash
hades --help
hades -c planet.toml
```

### config

There's a sample config file under `fixtures/planet.toml` for reference. But broadly, you can set the following overall configs:

```toml
[setting]
out_dir = "examples/planet-rs/content/"
front_matter_ext = "zola-toml"
workers = 25
tags = []
```

- `out_dir` specifies where the posts should get aggregated
- `front_matter_ext` specified whether to spit out the post metadata in TOML or YAML. The 2 values supported currently are `zola-toml` and `yaml`.
- `workers` is the number of threads spawned for fetching the posts
- `tag` is an allowlist of tags that the posts per feed will be filtered by

To add the authors, specify a configuration under `[[authors]]` like the following,

```toml
[[authors]]
name = "Aaron Turon"
feed = "http://aturon.github.io/atom.xml"
```
 
You may also optionally specify...

```toml
[[authors]]
...
skip = true
tags = []
```

...which allows you to `skip` the feed entirely and `tags` that are specific filters for the feed.

**Note:** if you do not specify the per-author tags, the global setting tags will be used. Similarly, if you specify per-author tags, they will override the global setting. Leaving tags empty fetches all posts.

### development

This project is still under active development and would really love some help! The current `TODO` list is the following:

- Caching posts: don't regen posts if they already exist, unless they are updated
- Better error messages
- Logging
- Tests (unit or otherwise)

### examples

An example of how one may use this is is under [examples/planet-rs](examples/planet-rs) and is hosted within the same repo [here](https://kitallis.github.io/hades).
