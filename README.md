# Changesetti

Changesetti is a language-agnostic package release tool written in Rust. It is heavily inspired by [Changesets](https://github.com/changesets/changesets) architecture and workflows.

It aims to be as simple as can be to use out of the box.

## Should I use Changesetti?
Lol, probably not right now unless you'd like to help test partially working software and/or give feedback. Changesetti is very much in its infancy, is not fully featured or tested, can break in various ways, and is coincidentally my first Rust application. It was born out of a need for a standard automated release workflow that could work across languages for publishing their respective packages..... and also as a way to teach myself Rust.

The ultimate end goal of Changesetti is to support a wide array of languages, using single package or multi package repos (monorepos), but for the first iterations, will only support single package repos.

## Features
&#9745; = Complete

&#9744; = Planned

### Changeset Workflow
Language | Single package | Monorepo
--------|--------|----------
Javascript/Node | &#9744; | &#9744;
Ruby | &#9744; | &#9744;
Go | &#9744; | &#9744;
Rust | &#9744; | &#9744;

More langagues TBD...

### Automation
&#9744; Github Bot for Changeset detection

&#9744; Github Action for PRs + Release

### Other Features
? FFI wrapper packages - Changesetti is intentionally not a dependency of your project. But to explore Rust FFI I may create these in the future.





