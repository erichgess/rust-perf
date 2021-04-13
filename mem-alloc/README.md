# Purpose
A simple application to compare the performance of allocating data on the stack
vs. allocating it on the heap.

# Usage
When you run `cargo run` this will execute a series of memory usage tests and 
output the duration of each test. The goal is to give an idea of the performance
costs that are incurred by allocating on the heap (the extra calls to malloc).
And to show the performance difference between using memory in the heap and stack
for single variable lookups (there should not be much difference).

Sample Output:
```
500000500000 in 55985542ns (Stack)
500000500000 in 54952515ns (Stack Alloc Once)
500000500000 in 237117154ns (Heap)
500000500000 in 59568421ns (Heap Alloc Once)
```

The third test is very slow because memory is allocated on the heap in each loop.
The fourth test allocates a variable on the heap once, before the loop, and reuses
that variable on each pass; demonstrating that using a variable in the heap is
just as fast as on the stack. BUT this does not take into account data locality,
when using many variables in an operation, they will be all contiguous on the stack
and therefore very fast to load but on the heap there is no guarantee they are
contiguous and so performance will almost certainly be slower as there will be more
cache misses.