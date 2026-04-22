# Eli On DD-Overlapped Scheduling

Local preservation of Eli's clarification about Differential Dataflow and
egglog scheduling:

> Aiui the cool thing about DD is that you can (maybe..) get "relaxed
> scheduling" without actually changing the semantics of the egglog program.
>
> This is because DD has a multi dimensional concept of time: you may be able
> to get it to "start running the next iteration" before the first iteration is
> completely finished. DD has enough tracking to incrementally evaluate the rest
> of iteration N+1 once all of iteration N finishes (and so on, for N+2 as
> well)
>
> This is a very neat thing that it does that egglog doesn't do. Where egglog
> needs to start and stop between iterations DD at least sometimes doesn't have
> to

Interpretation for this repo: the main Option 3 scheduling hypothesis should
be exact logical egglog scheduling with overlapped DD physical execution, not
semantic relaxation by default. Explicitly relaxed scheduling remains a fallback
variant if exact overlap is too constrained or expensive.
