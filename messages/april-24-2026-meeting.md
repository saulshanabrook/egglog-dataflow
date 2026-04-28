Around: https://github.com/egraphs-good/egglog/issues/851

- Optimization for proofs-like stuff
  - (=e (Add x y)), (F y), (= z (AddProof e))
  - A                       B       C
  - Can execute by binding x, y, e, and then looking up into AddProof to find z.
    - In this way C "depends on" A.
  - A\[t..\], B\[0..t\], C\[0..t\]
  - A, B\[t..\], C\[0..t\]
  - A, B, C\[t..\]
  - A different, mostly valid (under certain assumptions \--- see the note below), set of queries is:
  -
  - A\[t..\], B\[0..t\], C
  - A, B\[t..\], C
  - MISSING CASE: new C, old B
    - A, B, C\[t…\]
  - We can add back:
    - A, B, C\[t..\]
    - *but only if* A\[t..\], B\[t..\] are both empty (and C\[t..\] is nonempty).
    - Eli: can we also skip case 3 when all 3 are nonempty?
      - Oliver: Yes BUT only if C depends only on A from the functional dependency
        - In the query above: if B has something new, we can skip 3
        - What about if A has something new and b does not? unclear
      - Yihong: I claim we can skip 3 as long as A is nonempty
        - Oliver’s examples:
        - Bad case:
          - A(x), B(y), C(y, res)
          - Old x, old y, new c
          - If you skip 3 you might miss this
        - Good case:
          - A(x), B(y), C(x, res)
          - Old, old, new not necessary- doesn’t depend on y
    - In the case of proofs, this will be free because C\[t..\] is
    - Is this equivalent to?
      A\[0..t\], B\[0..t\], C\[t…\]
  - So the final optimized query is
    - 1 A\[t..\], B\[0..t\], C                       (new, old, all)
    - 2 A, B\[t..\], C                              (all, new, all)
    - 3 A\[0..t\], B\[0..t\], C\[t…\]               (old, old, new)
    - We hit all the combinations
      - Old old new  (covered by 3\)
      - Old new old  (covered by 2\)
      - Old new new  (covered by 2\)
      - New old old    (covered by 1\)
      - New old new  (covered by 1\)
      - New new old  (covered by 2\)
      - New New new   (covered by 2\)
    - Yihong: we need some sort of cardinality fix for when C\[t…\] is small

- Differential Dataflow framing
  - Built on Timely Dataflow, roughly analogous to how egglog has a backend
    substrate today.
  - Computes streaming computations incrementally.
    - One way to think about this is keeping intermediate views up to date,
      while egglog often recreates or reruns them at schedule boundaries.
  - Has a strong notion of consistency.
  - Supports fixed points.
    - Can compute loops.
    - Rebuild-like work is one place we might use this.
  - Computes a `Z` relation: a mapping from row to integer multiplicity, or how
    many copies exist in the database.
    - This generalizes Datalog sets.
    - Transitive closure can be computed incrementally.
    - This generalizes conventional seminaive evaluation.
    - Related background: Naiad / Dryad work from MSR.
    - DD can use 2D/product timestamps for every value, for example the time at
      which data arrived plus the loop iteration. Egglog's current timestamps
      are 1D, but the partial-order timestamp model should still support the
      seminaive math above.
    - How would this be exposed in the language?
      - `loop/fixpoint` in the query, as an Option 3-dependent research
        question rather than a committed frontend feature.
