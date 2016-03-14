# Earley

[![Build Status](https://travis-ci.org/jnicklas/earley.svg?branch=master)](https://travis-ci.org/jnicklas/earley)

Earley is a Rust library for writing parsers, built on Earley's parsing
algorithm. It is highly experimental and not at all performant or stable.

## Why Earley?

The most common parsers are LALR parsers. These can parse any LL(k) grammar in
linear time. The problem is that it is hard to write LL(k) grammars. Working
out exactly what a parser can and cannot parse is difficult. In contrast,
Earley parsers can parse *any* context free grammar. No shift-reduce conflicts
ever. If it can be expressed as a context free grammar it can be parsed.

The downside is that Earley parsers cannot guarantee linear execution time.
While they execute in cubic time in the general case, they are often much
faster in practice, especially when used with left-recursive grammars, which is
often the most natural way of expressing grammars.

## How does it work?

Here's a (basic example)[examples/basic.rs] of how to use this library.

Here's an example of (how to generate an AST)[examples/ast.rs].

First we construct a grammar, which we then use to parse the input. The most
important part is the `earley_production!` macro. While it is possible to
implement production rules without this macro, doing so is pretty tedious.

## To-Do

There are a lot of things to be done before this becomes actually viable:

- Create a syntax extension which makes writing grammars bearable
- We currently store all predictions and scans, we probably don't need to do
  that. Storing all completions should be sufficient.
- There are lots of clones to get around ownership problems, we should work
  to remove those.
- Optimizations.
- Statically allocated grammars would be nice. Grammars shouldn't have to be
  stack allocated. Is this achievable (lazy_static! notwishtanding)?
- The item table is pretty much a Vec<Vec<>>. This is terrible. It wastes a ton
  of space, is bad for cache locality, blah blah. Should be fixed.
- There's really no need for the BTreeMap of rules, we should just make
  everything pointers to other rules.

## Acknowledgements

This library is very much based on Loup Vaillant's [excellent
tutorial](http://loup-vaillant.fr/tutorials/earley-parsing/) about Earley
parsing. It would not have been possible without it.
