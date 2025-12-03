# Day 3 - Lobby

## Abstract

For a sequence of digits we need to determine the largest number, that is a sequence, which is assembled from digits of the original sequence, with digits in the original order.

## Strategy

* we will assemble numbers with a fixed number of digits.
* to get a large number, the first digit needs to be as large as possible
* so we search for the largest digit in the substring `input[0..=length-digits]` giving us a value and position
  * for multiple occurances we take the first one, to have more flexibility afterwards
* any later digit at `position` looks in the substring `input[current_offset+1..=length-digits+position]` where `current_offset` is the absolute position of the previous selected digit.
  * as `find_largest_symbol` runs on a substring, `current_offset` is added up.

## Implementation 

* use string manipulation this time.
