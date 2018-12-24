# Solution

The naive solution is a top-down search of the entire tree of solutions with depth-first search and remember the lowest cost combination. There are 2^N possible solutions for N paints and this exhaustive approach will enumerate them all before stopping.

For 5 paints this is a small search space and the naive approach would probably suffice but I'm going to assume we want to find a more efficient solution.

We can do much better than the naive search in all but the most pathological inputs by employing some strategies in the search.

## Best-first search

If we take that every choice of an additional paint has a cost of either 1 (for choosing a Matte paint) or 0 (for choosing a Gloss paint) we can enqueue nodes for extension in the search that have the least accumulated cost first. This is achieved by using a min-heap as a priority queue. This gives our search the convenient property that the first found viable solution is _also_ guaranteed to be the lowest cost one possible.

## Checking for constraint violation in subtrees

Some customers only have 1 paint combination that they will accept. Based on this constraint we can prune any subtrees from our search where this constraint isn't satisfied.

# Building

To build the executable:

```
$ cargo build --release
```

To test on some example inputs:

```
$ cat examples/input3.txt | ./target/release/colors
G M G M G
```
