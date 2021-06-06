# diversenether

Simple searcher to find seeds with all nether biomes within a certain radius.

![2021-06-04_13 24 47](https://user-images.githubusercontent.com/11098083/120741833-22a55380-c539-11eb-8ef9-5992911a897d.png)

## Overview

Find a Minecraft seed that contains all nether biomes within the given radius (from `(0, 0)`).  Arguments are used like this:

```bash
$ diversenether.exe <radius> <skip>
```
The `radius` is the size around `(0, 0)` to check. The `skip` parameter is an optimization that will only check every `<skip>` block, because biomes are large and you don't need to check every single block. For example, 

```bash
$ diversenether.exe 100 25
```

Means that the search will be 100 blocks wide around `(0, 0)`, and only every 25th block will be checked.

## Performance

This search runs very quickly. See below for benchmarks:
```
Found seed with all nether biomes within 50 blocks
1133 seeds searched in 0 seconds
Seed: 1579501499

Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz   4.20 GHz
```
