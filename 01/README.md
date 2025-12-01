# Day 1

## Abstract

We are given rotating lock with 100 segments. We are given a list of rotations (>4k entries; i.e. `L25` for rotate 25 segments to the left). How often do we hit zero?
* Part 1: at the end of a rotation
* Part 2: at any time

## Strategy

* read the text input using a state machine
* rotate (stepwise) and count
