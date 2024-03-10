## Write a simply-typed lambda calculus in a week

This tutorial aims at guiding you to implement a (fairly simple) functional programming language embedded in Rust (as a DSL) within a week.

Specifically, we will adapt from the original syntax of lambda calculus introduced by Alonzo Church in 1930s, i.e.,

```
e ::= x         -- variable
    | λx.e      -- lambda abstraction
    | e e       -- application
```

The general roadmap of this tutorial is as follows, of course, it's under construction. (especially for type system and corresponding handouts)

- **day 1 ([handout](https://zhihu.com) available)**: you will learn the basic **syntax** of the language with the design of how to embed our (currently) untyped lambda calculus using `Rust Enum`, do some simple encoding exercises, while also having the chance to add more syntax rules yourself (and of course, play with them).

- **day 2**: you will learn about the **semantic** of utlc, as well as the operational rules we will use throughout this tutorial, after which you also need to implement three useful utility functions that can help you do better in day 3. 

- **day 3**: you will first learn about two evaluation strategies, call-by-value and call-by-name. after this you will start to implement the core part of the first half of our tutorial - to achieve the ability to actually evaluate / interpret / use the untyped lambda calculus we have seen so far. (and a little spoiler here: I promise this would be a lot fun)

- **day 4**: it's time to introduce some "spicier" features into our utlc, which is the ability of **recursion**. to achieve this, we will utilize `yCombinator`, and don't worry, you will learn how it works when we get there.

- **[WIP] day 5-7**: introduce a **sound** type systems and make our untyped lambda calulus typed.

Regarding official handout(s), currently they are only available (in Chinese) on my [zhihu article page](https://www.zhihu.com/people/dawn-36-29-53/posts).

Any pull request / issues / advice will be greatly appreciated!