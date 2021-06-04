# diversenether

Simple searcher to find seeds with all nether biomes within a certain radius.

## Overview

Find a Minecraft map seed that contains all nether biomes within the given radius (from `(0, 0)`).  Arguments are used like this:

```bash
$ diversenether.exe <radius> <skip>
```
The `radius` is the size around `(0, 0)` to check. The `skip` parameter is an optimization that will only check every `<skip>` block, because biomes are large and you don't need to check every single block. For example, 

```bash
$ diversenether.exe 100 25
```

Means that the search will be 100 blocks wide around `(0, 0)`, and only every 25th block will be checked.

## Peformance

This search runs very quickly. See below for benchmarks:

TBD
