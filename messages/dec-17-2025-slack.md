Yihong Zhang  \[12:01 PM\]

Frank McSherry has a new datalog with wcoj

* [https://github.com/frankmcsherry/blog/blob/master/posts/2025-12-03.md](https://github.com/frankmcsherry/blog/blob/master/posts/2025-12-03.md)  
* [https://github.com/frankmcsherry/blog/blob/master/posts/2025-11-21.md](https://github.com/frankmcsherry/blog/blob/master/posts/2025-11-21.md)  
* [https://github.com/frankmcsherry/datatoad](https://github.com/frankmcsherry/datatoad)

I also talked with Hangdong about his flowlog project last week: [https://arxiv.org/abs/2511.00865](https://arxiv.org/abs/2511.00865) Some takeaways before I forgot:

* FlowLog is built on top of differential dataflow (DD)  
* Counter-intuitively, they want workload to be spread across many iterations, and each iteration only does a small amount of work for parallelism (\!). The rationale is so that DD operators can work on different iterations at the same time, and DD is not efficient handling large batch of workload (because of incrementality).  
* They do columnar representation and it benefits them a lot  
* They can’t do query optimization or change query plan mid-way, so instead they focus on making the join robust, e.g., sideway information passing and predicate transfer  
* By building on top of DD it can scale up to something like 128 (or 64?) cores  
* He told me Umbra’s recursive CTE is very fast, even though it does not do semi-naive, which is impressive.  
* He told me DD allows one to write nested fixpoint computation, which he initially thought is not that useful from a Datalog perspective. But this is exactly egglog’s evaluation algorithm, with the inner fixpoint being the congruence closure. Egglog’s schedules are also full of nested fixpoint computation.  
* DD supports lattice timestamps. This can be used to support SAT-style backtracking I believe? But it becomes unsupported in DBSP’s formalization, because this is too complex

7 replies

---

Eli Rosenthal  \[11:45 AM\]

Does DD allow for encoding arbitrary schedules, not just rules+rebuilding?

\[11:47 AM\]

Lattice timestamps aren't enough for backtracking on their own I think… but they would allow us to “start the next iteration before finishing the last one” , which could help our parallel throughput a lot I think.

Yihong Zhang  \[12:19 PM\]

Do you mean lattice timestamps aren't enough for backtracking under egglog’s specific semantic model, or it's just not enough for backtracking in general?

\[12:19 PM\]

Re arbitrary schedule I think so

\[12:22 PM\]

Lattice timestamps would allow halting a branch of exploration and merging multiple branches?

Eli Rosenthal  \[1:52 PM\]

Oh i see what you mean. If you keep the whole history around then yeah i think that may work\!

\[1:56 PM\]

Well I'd definitely be interested in replacing core-relations with something flowlog-esque and benchmarking\!

Main things I’m curious about other than schedules I how extensible the setup is / how easy or hard adding custom tables like union finds or containers would be
