# Heapy Imp

imp with shared memory (i.e., heap) management (and of course, a simple memory model).

# Build & Run & Test

first run `sbt` in the root directory for heapy-imp.
- run `compile` in the interative sbt terminal to build the project.
- run `run` to execute the main routine.
- run `test` to execute all the unit tests in `MySuite.scala` - feel free to write your own test(s)!

# Memory Model

the memory model for this simple imperative calculus is also pretty straight-forward.

instead of a giant consecutive byte array (or vector), everything in the memory (a.k.a. the heap) is stored in a global map.

the map is just a one-to-one mapping from **address** to **value**, and both are just `Int`.

to see the detailed definition, check `ImpExp.scala`.