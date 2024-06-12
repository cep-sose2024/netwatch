---
title: "Pre-Commit Hook"
date: 2024-05-08 00:00:00 +0800
categories: [Rust]
---

# Pre-Commit Hooks

You may have come across the saying during  your younger days or throughout your life: "Quality over Quantity."
As members of the Netwatch team, we want to assure you that we are dedicated to work on both quality and quantity. In this article, I will explain how we ensure that the quality of our commits remains consistent.
To understand our methods and motivations, it's important to dive into the steps we take to guarantee the quality of our work. One of the tools we use is Pre-Commit hooks, which allow us to execute basic scripts before each commit. These scripts help us address potential issues and eliminate any suspicions regarding the quality of our work.

## Our tools:

We use as a basic tools something called rustfmt.

### [Rustfmt](https://github.com/rust-lang/rustfmt): 
1. Rustfmt is a tool for formatting Rust code according to style guidelines. 
Which helps us to keep our programming style in check.



### [Clippy](https://doc.rust-lang.org/nightly/clippy/):
2. Clippy is a collection of lints to catch common mistakes in Rust code. 
It is an excellent tool to run on Rust code in general. 
It can also help with performance, 
because a number of the lints relate to code patterns that can cause sub-optimal performance.



### [Cargo Check](https://doc.rust-lang.org/cargo/commands/cargo-check.html):

Check a local package and all of its dependencies for errors. 
This will essentially compile the packages without performing the final step of code generation, 
which is faster than running cargo build .

### [commitizen](https://github.com/commitizen/cz-cli):
Commitizen is release management tool designed for teams. 
Commitizen assumes your team uses a standard way of committing rules and from that foundation, 
it can bump your project's version, create the changelog, and update files.


