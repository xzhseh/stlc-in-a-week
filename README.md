## Write a simply-typed lambda calculus (in Rust!) in a week

### Credit

This tutorial is initially inspired by the **great** programming language course project (originally written in Haskell) provided by [Prof. Delaware](https://www.cs.purdue.edu/homes/bendy/) at purdue university.

It's fun to learn (and write) Haskell, but I love Rust more.

### Introduction

This tutorial aims at guiding you to implement a (fairly simple) functional programming language embedded in Rust (as a DSL) within a week.

Specifically, we will adapt from the original syntax of lambda calculus introduced by Alonzo Church in 1930s, i.e.,

```
e ::= x         -- variable
    | λx.e      -- lambda abstraction
    | e e       -- application
```

The general roadmap of this tutorial is as follows, of course, it's under construction. (especially for type system and corresponding handouts)

- **day 1 ([handout](https://zhuanlan.zhihu.com/p/685876438) available)**: you will learn the basic **syntax** of the language with the design of how to embed our (currently) untyped lambda calculus using `Rust Enum`, do some simple encoding exercises, while also having the chance to add more syntax rules yourself (and of course, play with them).

- **day 2 ([handout](https://zhuanlan.zhihu.com/p/688690857) available)**: you will learn about the **semantic** of utlc, as well as the operational **rules** we will use throughout this tutorial, after which you also need to implement three useful utility functions that can help you do better in day 3. 

- **day 3**: you will first learn about two **evaluation strategies**, call-by-value and call-by-name. after this you will start to implement the core part of the first half of our tutorial - to achieve the ability to actually evaluate / interpret / use the untyped lambda calculus we have seen so far. (and a little spoiler here: I **promise** this would be a lot of fun)

- **day 4**: it's time to introduce some "**spicier**" features into our utlc, which is the ability of "seemingly" performing **recursion**; to achieve this, we will utilize `yCombinator`, and don't worry, you will learn how it works when we get there.

- **[WIP] day 5-7**: introduce a **sound** type systems and make our untyped lambda calulus typed.

Regarding official handout(s), currently they are only available (in chinese) on my [zhihu article page](https://www.zhihu.com/people/dawn-36-29-53/posts).

Any pull request / issue / advice will be greatly appreciated!

### Interactive Frontend

I've written a tiny frontend as the "driver" and interactive "playground" for our simple DSL language, the style is inspired by [ipython](https://ipython.org/).

To start it, simply run `cargo run`.

Typically you could use this frontend to **interactively** (and that's why I'm not adding a traditional parser at the very beginning) **build** / **test** / **evaluate** your stlc implementation.

It's still under heavy development (and refactor), I'll bring fancier features in the future - the ultimate goal is to create a ghci-like interactive interpreter for stlc.

Below is a **demo** gif using the built-in interactive environment to play with my (i.e., reference solutions) stlc implementation. (p.s. it is evaluating `(λx. λy. incr y) ω 114513` via call-by-name)

![demo](img/demo.gif)

### Exercises & Tests

Exercises are in the corresponding file in `src/exercises` directory. Though handout may not be available yet, you can still try the exercises in future day(s), and I've included detailed comments above each exercise.

There will be no hidden tests throughout this tutorial, every test is available in `tests` directory, feel free to check them out.

Regarding tests for different days, typically you could ensure you have passed all the tests by running following command, just remember to use the correct day.

```bash
cargo test --test <specific day>
```

### Reference Solutions

I have provide all my reference solutions for this tutorial from `day1` to `day4`, in `src/refsols` module. Feel free to check them out, or you can also play with my "official" solution(s) via the interactive environment.
