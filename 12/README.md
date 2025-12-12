# Day 12 - Bounding Checks

## Abstract

We are given plenty of trees to store presents below. Trees have
an area (width x height), and numbers of presents to store below them.
Presents are shaped 3x3 with empty spaces.

## Strategy

Perform a theshold checks for every tree.
* If every present could be treated as 3x3 square, and they fit, mark the tree as "works".
* If the total area below the tree is less than the sum of all presens, mark the tree as "never works"
* Verify there are no uncertain trees.
