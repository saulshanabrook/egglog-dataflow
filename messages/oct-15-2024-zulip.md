Frank McSherry: Hi folks\!

I've been reading a bit, and coding a bit, and writing a bit. In particular, I whipped up an e-graph "implementation" in a short enough span of time that I think I probably missed a few things. I wrote about it [here](https://github.com/frankmcsherry/blog/blob/master/posts/2024-10-19.md), which isn't live yet (well, not yet advertised as such). If anyone has eyes for it I'll sponsor a round of drinks when next we meet (or, have my UW representative leap into action) .

In particular, a few weird-ish things shook out. In no particular order:

1. I didn't really use union-find. I mean, I used it for graph connectivity in the rebuilding, but I didn't use it "live" or in any way that a normal graph connectivity algorithm wouldn't have worked.  
2. I didn't think too hard about choosing the representative for the merging, but I think now that it is pretty important to leave the largest e-class as the root and merge others into it, to avoid quadratic time. It may be that this isn't a big deal (and egg seems to already do this correctly, if I understand right).  
3. As far as I can tell, each rebuild egg takes linear time updating various caches (like `classes_by_op` in `rebuild_classes`). It seems like you could take much less time than this with careful updating. At least, afaict I do work only on impacted e-nodes, though .. also I'm new here and probably missed a bunch of things.

I'm delighted to see that you all have connected e-matching and graph motif finding\! WCOJs are really great, and .. I've for the longest time had no real application of algorithms there (I have more triangles enumerated than any person needs in their life). I need to improve this part of the code.

I'll have some analysis questions in the future. We have a similar sort of framework at Materialize, though it has stacks of analyses that depend on each other, and supports arbitrary nested recursion, which .. I didn't understand how I would integrate into e-graphs. At least, the conventional lattice-based update rules need to shell out to some optimistic iteration, or something brand new.

I put together a version of [EqSat in differential dataflow](https://github.com/TimelyDataflow/differential-dataflow/pull/525). It's not anything you'd want to use, but I really like the idea that you have some equivalence classes of e-nodes, and everything else is derived views from this. DD does a good job at maintaining (potentially iterative) views over data, scales out, etc. I need to internalize the `egglog` stuff a bit more, but DD has been much more general than datalog, and I wonder if there's a further connection. It's able to handle equality retractions, for example, but at great cost. AMA.

Last Q for now: I have a strong interest in multiple-return languages, because dataflow stuff wants this (mutual recursion, demuxing, probably other operators with shared state) and there aren't really tupling operations on collections. E-graphs seem to have a pretty strong binding to "an expression evaluates to one thing" but I was hoping to fish for any thoughts there.

I appreciate all the work\! Please keep thinking\!

Saul Shanabrook: Thanks for the post\! I'll leave the more egg implementation questions to someone else but on a few of your last points:

Last Q for now: I have a strong interest in multiple-return languages, because dataflow stuff wants this (mutual recursion, demuxing, probably other operators with shared state) and there aren't really tupling operations on collections. E-graphs seem to have a pretty strong binding to "an expression evaluates to one thing" but I was hoping to fish for any thoughts there.

This is quite straightforward in egglog (and I imagine in egg to), by defining a Pair type with left/right destructors: https://egraphs.zulipchat.com/\#narrow/channel/328972-general/topic/Multi-Output.20Data.20structure/near/472746624 The only annoying thing about it in egglog is the lack of user defined generics currently, requiring you to redefine it for every instantiation of left/right type.

I need to internalize the `egglog` stuff a bit more, but DD has been much more general than datalog, and I wonder if there's a further connection. It's able to handle equality retractions, for example, but at great cost. AMA.

A while back I was asking around if anyone had experimented or thought about trying to build egglog on top of differential dataflow. @Eli Rosenthal is one of the main folks who has been working on the current backend and is in progress with a new backend as well to egglog. If I remember correctly, one of his main questions was the support for WCOG in differential dataflow. From someone who isn't familiar with all tradeoffs, it seems attractive to push some of the harder DB problems down to an existing production system if possible, and to build a rich declarative query language like egglog on top of it.

Happy to have you over in egg land :)

Frank McSherry:

This is quite straightforward in egglog (and I imagine in egg to), by defining a Pair type ...

Yeah, I thought it might be something like this, but .. I'll need to ponder it a bit more. It's not a natural concept in the space I'm in, but perhaps you do the optimization in a different framework than you end up describing the implementation, and it's all good.

one of his main questions was the support for WCOG in differential dataflow

It's been there forever. :D Probably one of the first scalable WCOJ implementations (in 2014, I think). The fundamentals are all here abouts: [https://github.com/TimelyDataflow/differential-dataflow/tree/master/dogsdogsdogs](https://github.com/TimelyDataflow/differential-dataflow/tree/master/dogsdogsdogs). The main thing is that it's about a 180-turn from leapfrog triejoin, which works great in-memory but less well distributed. Rather than use tries, you end up with the `count`, `propose`, `validate` operators, and build things out of them. Lmk if you have questions, as I've coded it up many times.

Related, I was wondering if streaming WCOJ made sense, if you all were bottlenecked on rule evaluation. Presumably with each round of e-matching and unioning you only want to find new e-matches in the changed region of the e-graph, there's some potential to iterate faster by not starting from scratch. Though, also a bunch of noise just because e-class consolidation means labels change. Anyhow, there's a paper about that, if you are interested: [http://www.vldb.org/pvldb/vol11/p691-ammar.pdf](http://www.vldb.org/pvldb/vol11/p691-ammar.pdf)

Frank McSherry: And, just checked around, here is the TD rig that I think we used for the VLDB paper. I .. haven't checked that it still builds against current TD, and I'm pretty sure that ye olde TD is UB in current Rust. \=/ But, figured I'd link it: [https://github.com/frankmcsherry/dataflow-join](https://github.com/frankmcsherry/dataflow-join)

Eli Rosenthal: Hello; long-time fan of your posts\! cc @Max Willsey who is probably the best person to answer questions / suggestions re: the implementation in egg.

A couple general points that may help give some context on egglog:

* **egglog / datalog / Differential Dataflow** I'm definitely curious about how egglog relates to DD. On the one hand egglog is a [good deal more](https://effect.systems/blog/trs-regularity.html) expressive than Datalog. On the other hand, my recollection is that DD's model let's you correctly compute aggregates/reductions over groups (like the integers with addition), and not just join semilattices and the like. egglog does okay if running all the rules again is idempotent, but it fails to compute (say) the sum of a column out of the box. I suspect there are more differences too...

* **arbitrarily nested iterations** aren't something that egglog has great support for today. We have a scheduling language that sort-of lets you do this ("run rules X,Y,Z till saturation, then proceed to run A,B,C ... ") but a lot of the time we've found that it's hard to reason correctly about stuff like that; I think we may still be climbing the learning curve with patterns that complex.

* **WCOJ** Saul's right that a major reason for implementing egglog on its own was WCOJ, which wasn't in DD at the time. We are continuing to investigate new algorithms from the database literature [1](https://arxiv.org/abs/2301.10841) [2](https://db.in.tum.de/people/sites/birler/papers/diamond.pdf) to try and speed things up though. And we also haven't done a ton of recent benchmarking comparing egglog's current WCOJ implementation to a bunch of binary joins with a DP or Cascades-style query optimizer.

* **Parallelism / Scale out** A few folks at UW are starting to look into parallelism for egglog as well. Starting with some recent work for parallelizing WCOJs that I'm still digesting. I think looking at DD may be a helpful comparison for that work. Egglog as a language is, I think, a kind-of fun place to think about parallelism because you can run rebuilding "whenever you want" and the language still behaves well. You can imagine running smaller versions of eqsat locally and then unioning identifiers "globally" less often, for example.

AMA

I do have a couple questions \!

Related, I was wondering if streaming WCOJ made sense, if you all were bottlenecked on rule evaluation. Presumably with each round of e-matching and unioning you only want to find new e-matches in the changed region of the e-graph, there's some potential to iterate faster by not starting from scratch. Though, also a bunch of noise just because e-class consolidation means labels change

As I read it, Delta-GJ in this paper is doing a form of semi-naive evaluation? Either way, it's \_exactly\_ what egglog is doing for rule evaluation and it has indeed been a big win performance-wise\! We did have to make sure to compute the deltas due to rebuilding appropriately, as you say. The egglog paper talks a bit about this

I put together a version of [EqSat in differential dataflow](https://github.com/TimelyDataflow/differential-dataflow/pull/525).... \[DD is\] able to handle equality retractions, for example, but at great cost.

This PR is really cool, and I think I need to dig into it more. One thing I'm wondering is if the TC iteration at the end would be faster if it was replaced by a union-find. How many times does that inner loop need to iterate in order to converge? Is this what you mean by 'equality retraction'?

In general, rebuilding is a major reason why it's been hard to look at using an existing framework to implement egglog: using a union-find directly and being able to manually mutate tables has been a big performance win in the past. Always interested in reevaluating this though.

Frank McSherry:

Hello; long-time fan of your posts\!

Oh, thank you\! :D

DD's model lets you correctly compute aggregates/reductions ..

This is true, but more generally it allows you to traffic in non-monotone operations. So, for example union-find is pretty easy to implement, because you are allow to *retract* the edges you want to replace, rather than only adding additional edges, as you might in Datalog.

We are continuing to investigate new algorithms from the database literature ..

This makes sense\! I'm still stuck on the v1 algorithms, and really should catch up. And afaiu there's been a bunch of progress, and no better way to learn than doing things yourselves (e.g. me tying up my own e-graph impl, vs just using egg).

As I read it, Delta-GJ in this paper is doing a form of semi-naive evaluation?

Perhaps\! It supports retractions as well as additions, and I thought that might be important for EqSat where although the e-graph only "adds" information, the representation of that information experiences retractions (e.g. an e-class label changing, which "retracts" any relationships involving the old label). It isn't using anything more complicated than the multi-linearity of join, so it's not top secret stuff. I'll crack open the egglog paper and read harder, to look for the corresponding moments.

This PR is really cool, and I think I need to dig into it more. One thing I'm wondering is if the TC iteration at the end would be faster if it was replaced by a union-find.

1. I wrote the PR before I wrote my e-graph implementation, so .. it should be fixed. At least, it doesn't reflect what I understand now. Don't study it too hard, or if you do and conclude "this is bad" you are right\!  
2. Union find should be pretty easy in DD. The code there is doing the transitive closure with repeated squaring, but its for a tree not a general graph. It's a bit like concurrent path compression, in union-find language. Because DD can express it, the iterations are *updating* the labels not just adding more as Datalog would. Each iteration has a linear number of records and does at most a linear amount of work, for at most log iterations, and actually most values stabilize much faster (in log (initial distance to root)).

One of the main caveats with DD, especially that PR, is that unless you tell it otherwise it maintains enough information to arbitrarily update the input to the computation (any input additions or retractions, or generally any update along the semigroup it uses for `diff`). So, while I said the union-find will be "linear" it's actually maintaining the trace of updates across all iterations. So, more like n log n because of the iterations. But also, the outer EqSat loop as written maintains enough state to undo any input equivalence, which is probably more state than folks care to pay (who wants to remove equivalences? theorem provers with backtracking, maybe? Idk how DPLL works anymore. T.T).

The "fix" is to express the iteration differently, if you want to update in place, by not using DD's `iterate` method. You can instead wire up a loop manually, and with carefully chosen timestamp types (a lexicographic order like Rust's tuples, rather than the product partial order) you can seed each outer saturation iteration with the results of the prior iteration. This starts the next iteration off better, but also informs the state compaction algorithms indirectly that they don't need to track all of those per-update histories.

That's a lot of words, so perhaps worth me typing up something longer and more post-y. Union-find might be a good example, though, of something folks expect to work well because it is hand-rolled, but can still be made to work (I claim; watch this space to see if it is true) in a framework like DD.

(( Fwiw, DD is roughly equivalent to a [PRAM model](https://en.wikipedia.org/wiki/Parallel_RAM), where `join` is "concurrent read", `reduce` is "concurrent write", and `iterate` gets you looping. Which makes it very easy to say things like "you can do blah blah in DD" because you probably can, and the only question is whether it sucks or not ))

Saul Shanabrook:

This is quite straightforward in egglog (and I imagine in egg to), by defining a Pair type ...

Yeah, I thought it might be something like this, but .. I'll need to ponder it a bit more. It's not a natural concept in the space I'm in, but perhaps you do the optimization in a different framework than you end up describing the implementation, and it's all good.

I would be curious to hear more about the space you are in. If I am understanding your last phrase correctly, your suggesting that the way you would naturally describe your implementation would be quite different from this kind of structure?

If it's helpful, I am also translating a space that has operators with shared state (the array API in Python), and in trying to translate it to egglog I have to basically functionalize everything... So that even if it looks like you are doing mutation or having implicit shared state, it has to be represented in a purely functional manner.

Max Willsey: Hello @Frank McSherry\! Thanks for your post and your questions, very cool what you're thinking about. If you're ever around SF/Berkeley, let me know\!

Lots to respond to here. From your original post:

1. Union-find isn't really necessary if you don't care about a "record" of the equivalences. It suffices (as you found out) to "do" the equivalences right away. As long as your update everything to be canonical, you can trash the old equivalences since they are already accounted for.  
2. Egg does do this, choosing the smaller e-class (not by rank in the UF).  
3. To a first approximation, rebuilding (refreshing) doesn't really matter. The current impl is good enough to not be the bottleneck in real workloads, which are dominated searching for patterns and inserting e-nodes as part of applying rules. This is "the point" of running the rules, so that makes sense.

**Analysis**: I'd be interested to hear more about this. I think e-class analyses have been one of the more powerful results from this line of work, more so than we expected at the time. (The egg paper focuses a lot on congruence closure, which I now think is just much less interesting. Hindsight and all that)

**Differential Dataflow**: Very cool\! I'm curious why you chose this instead of DDlog? I am not surprised, however, that this is possible. I have long thought that a "sufficiently smart" and extensible datalog (or perhaps dataflow\!) system could easily implement egglog. We just didn't know about one at the time, hence rolling our own datalog. I have also heard that @Kristopher Micinski's group has been looking into implement e-graphs on top of their [ascent](https://s-arash.github.io/ascent/) datalog system. A question on this front: does the Differential Dataflow approach automatically derive an efficient congruence closure implementation? That would be a very interesting result\!

**Multiple return**: Yes, indeed e-graphs represent terms which evaluate to one thing. As Saul mentioned, constructing and projecting out of pairs gets job done in many cases. At EGRAPHS 2024 there was a cool presentation/paper on [connecting e-graphs up to monoidal theories](https://pldi24.sigplan.org/details/egraphs-2024-papers/9/Equivalence-Hypergraphs-E-Graphs-for-Monoidal-Theories), in which things have multiple inputs and outputs. A bit far from practical at this moment, but interesting nonetheless.

Using differential dataflow as the basis of e-graphs is very interesting, and something I should give more thought to.

Another axis here is @Chris Fallin's approach to implementing e-graphs in the Cranelift compiler. He implements a much simpler version of e-graphs, no congruence closure or even e-classes (instead union is itself represented as node). E-matching is done with a very naive algorithm but it's efficient from a systems perspective thanks to code generation. I wonder if this perspective would be even \_more\_ amenable to differential dataflow.

Frank McSherry: @Saul Shanabrook

I would be curious to hear more about the space you are in. If I am understanding your last phrase correctly, your suggesting that the way you would naturally describe your implementation would be quite different from this kind of structure?

Dataflow stuff feels a bit like circuits, and you end up with building blocks that naturally take multiple inputs and can produce multiple outputs. Not outputs of sum types, which could work, just multiple outputs of vanilla types. My guess is that while I could tuple things up, or pack them in sum types, the very next step would be to write a field/variant-level analysis to learn anything at all about any actual collection, as the tupling would otherwise interfere with expression level analyses. Not the end of the world, but a bit of friction.

As an ill-thought out example, we want to track things like "monotonicity" for evolving collections, when they don't have any negations in them ("append-only"). This would be a collection-level property, not a tuple-of-collection-level property, and packing/unpacking that information ends up being busywork without an inherent payoff. I mean, the payoff might be "can use e-graphs", but not otherwise.

And more concretely with the multiple returns, when doing something like mutual recursion of collections, you end up building dataflows that map from `[Collection]` to `[Collection]`, list of collections to (similarly sized) lists of collections. That is the thing you need to subject to analysis, and .. typing it out maybe it's easy because you just replace `[Foo]` with `(Foo+)`. It means I have to step away from my stack machine abstraction though. T.T

Frank McSherry: @Max Willsey

If you're ever around SF/Berkeley, let me know\!

Believe me that I will. :D I will extend the UW drinks offer to the Bay Area; I hadn't realized that you had moved (hooray\!).

1, 2, 3:

Thanks\! I was secretly hoping that with incrementally maintained e-matching as well, each part of the system would update pretty promptly and each would need to not be accidentally linear. But also, slightly different constraints from where I am (negligible e-matching, and lots more "exression minimization" that churns the rebuilding).

Analysis: I'd be interested to hear more about this.

Tbh this is probably me communicating badly. We have what is a fairly vanilla lattice-based optimizer that handles nested recursion with conventional optimistic approaches. But, it means that the update rules are not purely local, in that you need to stage the evaluation (reach each fixed point, before continuing on). I am basically confused about how that will work out in a continually evolving e-graph, and probably just need to read and think a bit. Probably scouring the egg code for how it handles `letrec` in the analysis code might clarify things, and potentially there's no problem. But also, if one consolidates an e-class within a recursive term, that has already reached fixed point optimistically, I'm not sure if/how that can "improve" the result, on account of optimistic analyses generally not moving in the "better" direction (starting from "best"/top/whatever, and getting worse).

Independently, I was trying to grok egg's `EGraph` has having an analysis generic, in a world where you probably have many analyses you'd like to use without necessarily rebuilding the e-graph. We end up doing this with some `Map<Box<dyn Analysis>, _>` and `Any` shenanigans, but perhaps these are the same and it's just statically binding the stack of analyses you might require.

I'm curious why you chose this instead of DDlog?

Ah, familiarity? DDlog is Leo and Mihai while at vmware, and is mostly (I think) a Haskell package that produces Rust that targets DD. But, other than the query optimization, I'm more comfortable writing directly in DD (and also, you end up wanting to write bespoke operators to implement WCOJ stuff; you don't get it out of DDlog afaik).

If you've read the DBSP paper from VLDB, TD/DD is a superset of what it can do, but there's some characterization of its complex recursive relational logic that Val Tannen had that that I don't fully understand (but e.g. Turing complete, unlike Datalog).

does the Differential Dataflow approach automatically derive an efficient congruence closure implementation?

Ah, well you have to tell it how to do the congruence closure as an iterative DD expression, and then it assembles the fully incrementalized implementation of that. The version I wrote in the PR iteratively develops a map from exogenous term `Id` to e-class `Id`, which you use to transform e-nodes with exogenous ids into e-nodes with e-class ids, which you consolidate (group by e-node) to further update the map from exogenous ids. The "group by e-node" part is one "congruence step" and that it lives in an `iterate` means it runs until fixed point ("closure").

I'd write it differently knowing more now, and .. probably should. I'll shout that out here if / when I do.

Max Willsey: Very cool stuff. re: Analysis. Egg doesn't have any ability to combine analyses; you have to basically construct the product yourself. egglog fares much better here (in fact this was one of the original motivations); analysis are in many respects "just another" function that updates its results monotonically. I suspect there are many ways in which egglog is more amenable to DD-ification.

re: lattices, letrec, staging, we have a poor story here. Basically, neither egg nor egglog have a good answer. Egg lets you write one analysis, and it better be pessimistic or it'll be unsound. In egglog, we at least have the conceptual framework of datalog, and so stratification becomes enticing. Unfortunately, a simple attempt (which is basically all we've done) of stratifying is unsatisfying, as all analyses depend on the (implicit) equivalence relation, and most analyses are principally consumed by rules to make new equivalences. I suspect a better version of this would figure out that you can indeed stratify "through equivalence" and break this cycle, but that's all very fuzzy. Neither egg nor egglog will stop you from writing an optimistic analysis, but you simply have to be very careful in how you consume it to not violate soundness.

Frank McSherry: Ah, so I pondered this a bit and .. lmk if this is a dead end (or .. don't and we'll see what happens). This is some amount in pursuit of something reasonable for `letrec`, but also just borrowing from how timely dataflow works with its nested scopes.

It seems like each e-graph naturally exists in some context with some set of free variables, not sure where they come from but they exist, ambiently. They can appear in terms, be equated, all that.

It seems not unreasonable to allow the introduction of a "nested scope", represented by a single operator, with inputs and (multiple?\!?) outputs, which inherits all the information from the surrounding e-graph but is also able to introduce new free variables. From the outside, it presents as an operator `op` with potentially opaque semantics. However, e.g. if it is a fixed point operator you might equate its e-class `x` with `op(x)`, to communicate the fixed-pointedness (not sure if/why that helps, but ..).

Internally, the operator is represented by another e-graph, which reflects the outer e-graph, with additional variables and the potential to introduce further equivalences among any terms that are in scope (its own variables, or the outer scope). Equivalences in this scope are not made visible to the outer scope, and are hidden behind the abstraction of "an operator". (( Edit: to parallel TD, you might only be able to see the e-class structure of the inputs, to abstract away the complexity outside the scope ))

Two goals here:

1. We (at MZ) have a sequence of operators like `ReadInput`, `Filter`, `Join`, `Project` each of which transform collections that have some number of columns, and our expressions are over the column identifiers as the "free variables". However, when moving from one operator to the next, new things may become true, like after a `Filter(c0 == c3)` we might equate columns `0` and `3`, which previously were not equated. Those columns are only equal *after* the filter, not before, even though "column 0" might seem like it is equivalent to the column produced by `ReadInput`. Similarly, after `Project` we re-arrange some columns, and column names change, so clearly we wouldn't want to use the same names *across* these operator stages. Scopes feel like they might provide an abstraction that makes sense here: you inherit things that are true of your input, and can introduce further constraints, but they do not make their way back to the input.  
2. Make-pretend that since a thing worked well at abstraction in TD, perhaps it works well in another context. In TD the relevant information ("progress tracking" and "internal reachability") were both losslessly communicated, and it worked great. The outer scopes couldn't possibly know e.g. the timestamps of the inner scopes (generic parameters), and this abstraction hid away details that the outer scope didn't need, while still allowing precision and complexity for the inner scope (e.g. loop iteration, which the outer scope wouldn't understand, but the inner scope needs to operate correctly).

Ah, ok so maybe make believe. Is there material I should read instead of writing fanfic about e-graphs and context/binders? My sense is that (again, fanfic) that if each `Op` is also allowed to monotonically improve its transfer function (for various analyses), and that is a permitted "update" to the e-graph, then you may be able to stash recursion in such a scope. Not sure about optimistic analyses yet, but it also allows you to stage/stratify the execution, by allowing the nested scope to control when it communicates information back out of the scope.

Iurii Zamiatin: I suspect slotted e-graphs might be of interest when talking about variables \- [https://pldi24.sigplan.org/details/egraphs-2024-papers/10/Slotted-E-Graphs](https://pldi24.sigplan.org/details/egraphs-2024-papers/10/Slotted-E-Graphs). E-graphs by themselves don’t really have “variables”, we just treat them as term former (ie there isn’t much difference between “x” and “True)

Contextual EqSat (where we can introduce equalities that are context-dependent like the filter example) is done with assume nodes or the coloured e-graphs abstraction [https://arxiv.org/abs/2305.19203](https://arxiv.org/abs/2305.19203)

Oliver Flatt: Hi Frank, jumping in now that there are interesting questions about scope and context.

As you've observed, variable names and nested scopes don't play super well with egraphs. In fact, I've been working on this problem for the last year: [https://github.com/egraphs-good/eggcc](https://github.com/egraphs-good/eggcc)  
 Our approach is to adopt the RVSDG IR: [https://arxiv.org/abs/1912.05036](https://arxiv.org/abs/1912.05036)  
 That way, scopes are not nested. Instead, values are explicitly passed between scopes as tuples/extra arguments. This also solves the problem of names for variables.

You also mentioned the problem of context, where in different scopes you can assume different predicates and infer context-specific equalities. In eggcc we tackle this problem by reifying context as a term, inspired by assume nodes in this paper: [https://arxiv.org/pdf/2205.14989](https://arxiv.org/pdf/2205.14989)

As for analyzing letrec/loops, we've stuck to pessimistic analysis in the eggcc compilier. That way, we can soundly start from a bad answer and refine it into a good one. Even our pointer analysis is pessimistic (but barely working so far so take with a grain of salt)

All of this is in-progress work\! We hope to publish it soon.

Frank McSherry: Thank you for all the links; I'll get reading\! :D

Max Willsey: @Frank McSherry I think you've bumped into a space that is very interesting and challenging for e-graphs. I think there are broadly two ways off approaching this (or perhaps some way better way that I haven't though of\!):

1. Make it so you can have multiple, nested equivalence relations. So if E1 \< E2, E2 "inherits" all the equivs from E1 and can add more that don't leak back into E1. This recent FMCAD paper on so-called [Colored E-Graphs](https://books.google.com/books?hl=en&lr=&id=8KglEQAAQBAJ&oi=fnd&pg=PA70&ots=xdlsnWDNDZ&sig=Znx2weZtig-lOKjU3AUfKY3G0oY#v=onepage&q&f=false) is probably the best reference here.  
2. The second approach is more node-based. In your filter example: `Filter(c0 == c3)`, you dont' want to equate c0 and c3 globally of course. But you could have the filter return the filtered columns, and then you could equate those if you wanted. This is similar to the assume nodes that Oliver mentioned above. The challenge here is it requires a substitution-like operation to create/push-down these contextual nodes, this can be expensive if you're not careful.

This is a fundamentally hard problem I think, but I think the community is making progress\!

Frank McSherry: Yeah, the colored e-graphs one lined up pretty well with my hopes/intuition\! If nothing else, trying to connect the dots to e-graphs have forced a bit more principled thinking about what we're working on (e.g. as you say, what is Filter "returning"; a thing we've been pretty casual about to date). Some other exciting moments around "error" values, and whether they should be equated or not. It feels pretty good though; I had a hacked-together "these exprs are all equal, and those are separately equal", but having some formalism (and a much more efficient implementation) is eye opening\! :D
