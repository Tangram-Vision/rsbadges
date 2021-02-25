# Testing

Nothing special here. Just run:

    $ cargo test

There are two sets of tests. The first tests error functionality when bad arguments are passed. The second actually generates a series of badge SVGs in each style with different information and formats. It then saves these SVGs to `/tmp`.