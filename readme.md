# xdiff-rs

This is a bit of a daft project. At work we make use of `gitoxide` for our three
way merge implementation. In `gitoxide` the three way merge allows adjacent line
changes to be merged conflict-free. In mercurial they allow you to configure how
close lines can be changed in order for it to be considered a conflict.

Ideally `git` would be able to configure this.

Why this train of thought has lead to a rust version, I'm not sure. I kind of
thought it would just be a fun experiment and perhaps clarify to a non-c
programmer like me what exactly is going on in xdiff.

I'm not sure if it is practical, desirable, helpful, etc... for something like
this to be upstreamed.

I'm also not keen on trying to pioneer an upstream project for porting git
propper's xdiff to rust since I'm not sure I have the time or expertise to
follow through on a project like that.

Franky, for my original goal, this is the entirly wrong way to go about things.

## Porting methodology

The initial goal is to port as faithfully as possible in a safe manner.

-   pointers to other structs get represented as `Rc<T>` initially
    -   They may be changed to `Rc<RefCell<T>>` if required.
    -   They may be wrapped in `Option` if nullability is required.
-   c-style arrays should be represented with `Vec` structures.
    -   Linked lists ought to remain untouched initially.

The choice of using `Rc` means that there is more work done than the c
alternative. These can almost certainly be refactored out to be references with
the right lifetimes, but it is an easier starting point to work with reference
counted data first, before moving to a borrow checked solution.

Using `RefCell` which provides run-time checked mutablity constraints is also
not ideal, but it means we can better scope mutability removals compared to also
just having references and having `mut` leak everywhere.

## Goals

1. Create a series of commits that port xdiff as faithfully as possible.
2. Create a suite of tests that demonstrate 100% xdiff-c/xdiff-rs compatibility.
3. Refactor symbol names to be descriptive.
4. Refactor c-style linked lists to use appropriate list types.
5. Refactor bitflags to enums.
6. Refactor functions to avoid mutable references where possible.
7. More refactors?
8. Introduce configurable merge confict gaps.

## License

This project doesn't have a specified license yet. I'm not a laywer, but I
_think_ it should be possible to license a rewrite in rust under the MIT without
needing permission.

If this is not the case, I would hope the git project would grant permission for
it to be licensed under the MIT.

At this current point in time it would be damaging to the wider `git` ecosystem
to require projects like this to adopt a GPL license. In the unlikly scenario
where I complete and actually have the time to try upstreaming it, it would be a
great boon for the ecosystem if other projects could safly link to it and have
access to a diff implementation that is aligned with git proper's.
