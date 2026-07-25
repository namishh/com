---
title: 2. Templating 
date: 24 July 2026
draft: true
---

### Introduction

Originally I was hoping that there already be some library like [tera](https://github.com/keats/tera) for zig. But... I was disappointed when there were none. There was a [templ](https://github.com/a-h/templ)-like alternative, but it was more of domain specific html-like language rather than a templating language. If templating was just replacing variables with strings, it would be as easy as finding as an `.replaceAll()` or some equivalent.

<br>

But I wanted the whole package with conditionals, loops, extend, etc, so it was clearly more complex than that. Looking at tera, the way is to do the entire lexer-parser-ast process, which I was very reluctant and lazy to do until this message dropped in a group chat:

![zr](/static/images/zr-pc0.png)

### Parser Combinatorics

Combining smaller parsers to create a more complex one, that is quiet what it is. You create simple parsers to match basic units like letters, digits, specific strings and then add combinators like sequences, choice, many to create increasingly complex parsers, and then there are functions like .map, or .chain which is used to take the parsed text into neat data structures. Javascript has a library called [arcsecond](https://github.com/francisrstokes/arcsecond) for that, and my aim was to create something similar like this:

```js title="arcsecond"
// taken from arcsecond readme
const newParser = composeParsers ([
  str ('world'),
  char (' '),
  str ('hello')
]);

newParser.run('hello world')
// -> {
//      isError: false,
//      result: "world",
//      index: 11,
//      data: null
//    }
```

Before I could start working on the actual templating library, I took some time in a small [demo repo](https://github.com/namishh/parser-combinatorics) to see how this plays out and for my final parser architecture, I came up with this:

![zr-pp](/static/images/zr-pp.png)
