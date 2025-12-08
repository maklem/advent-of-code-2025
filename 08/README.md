# Day 8 - Nearest Neighbours Network

## Abstract

We are given a list of 3D coordinates of junctions.
Over and over again, we will connect those two junctions, which are closest to each other.
Connected junctions form a network.

* Part 1: After 1000 connections, how large are the three largest networks (multiplied)?
* Part 2: Keep Connecting until everything is one network. Which is the final connection we need to make?

## Strategy

* From particle simulations I know that calculating all N^2 distances is slow.
  One usually places all particles on a grid, and only calculates distances
  (interactions in physics) for particles in the 3x3x3 cells around the original particle.  
  **However** as distances are calculated once (and calculation time << human time), I took the simple approach to calculate all distances.
* calculating a square root is slow as well. As the distance is never needed itself, I use the **distance squared**.
* Rust does not allow modifying two elements of a `Vec` at once.
  I can not use a mutable reference and then work with another element.
  Using `std::mem::replace` I can take one element from a vector, and merge it into another element afterwards.
