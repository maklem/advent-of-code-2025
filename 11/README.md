# Day 11 - Cached Paths

## Abstract

We are given a network graph.
* Part 1: How many paths are between one node (near the output) and the output node?
* Part 2: How many paths are between entry and output node, that run through two defined nodes?

## Strategy

Cache results once a sub path has been evaluated. This is required for part 2 to end within reasonable time.
