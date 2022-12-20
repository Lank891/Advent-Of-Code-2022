# [Day 19](https://adventofcode.com/2022/day/19)

Similarly to day 16, it doesn't work that fast, although it is very input-dependent. First of all, one of the assumptions is "if geode robot can be built, it is optimal" - it is not true in some special cases which are not present in this day. Additionally, the speed execution of part 2 really depends on input data. For 1st blueprint in example, part 1 was around 5s, part 2 around 50s. For real input, 1st part was done in 33 seconds (parallelized to 6 cores - so around 5s per input), but 2nd part took 1100 seconds (again parallelized, so the slowest from first 3 blueprints was this slow).

Note: example.txt will crash part 2 since it expects 3 blueprints, but there is only 1 (since 1st one is explained so I used it to build the algoruthm).