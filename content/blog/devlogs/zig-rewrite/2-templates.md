---
title: 2. Templating
date: 24 July 2026
draft: true
---

### Introduction

Originally I was hoping that there already be some library like [tera](https://github.com/keats/tera) for zig. But... I was disappointed when there were none. There was a [templ](https://github.com/a-h/templ)-like alternative, but it was more of domain specific html-like language rather than a templating engine. If templating was just replacing variables with strings, it would be as easy as finding as an `.replaceAll()` or some equivalent.

<br>

But I wanted the whole package with conditionals, loops, extend, etc, so it was clearly more complex than that. Looking at tera, the way is to do the entire lexer-parser-ast process, which I was very reluctant and lazy to do until this message dropped in a group chat:

![zr](/static/images/zr-pc0.png)

### Parser Combinatorics

Combining smaller parsers to create a more complex one, that is quiet what it is. You create simple parsers to match basic units like letters, digits, specific strings and then add combinators like sequences, choice, many to create increasingly complex parsers, and then there are functions like .map, or .chain which is used to take the parsed text into neat data structures. Javascript has a library called [arcsecond](https://github.com/francisrstokes/arcsecond) for that, and my aim was to create something similar like this:

```js title="arcsecond"
// taken from arcsecond readme
const newParser = composeParsers([str("world"), char(" "), str("hello")]);

newParser.run("hello world");
// -> {
//      isError: false,
//      result: "world",
//      index: 11,
//      data: null
//    }
```

Before I could start working on the actual templating library, I took some time in a small [demo repo](https://github.com/namishh/parser-combinatorics) to see how this plays out and for my final parser architecture, me and gpt came up with this:

![zr-pp](/static/images/zr-pp.png)

### Basic Working

So what do the smaller parsers look like? My Parser is not actually a struct, but a tagged union, meaning it can store one value from multiple type fields.

```zig title="parser"
const Parser = union(enum) {
    string: []const u8,
    digits: []const u8,
    /// so on
}
```

When it is time to parse a smaller chunk, I use a switch state to figure out its type and do the parsing. For making a string parser, i.e it should exactly match the string I am providing it:

```zig
fn parse(self: Parser, state: *ParserState) Error!Span {
    switch (self) {
        .string => |expected| {
            const remaining = state.remaining();

            if (remaining.len < expected.len or !std.mem.eql(u8, remaining[0..expected.len], expected)) {
                state.record_expected(expected);
                return error.CouldNotMatch;
            }
            state.index += expected.len;
        }
    }
}

// creating a helper function
fn str(expected: []const u8) Parser {
    .{.string = expected};
}
```

The code is pretty much self explanatory, it just checks if the expected value matches the start of remaining value (or the first n chars of remaining, where n is the lenght of expected value) and if it is true, we increase the index, else we throw an error.

We can test this with

```zig
test "string parser" {
    const parser = str("hello");

    switch (parser.run("helo")) {
        .success => |result| {
            test.expect("false");
        },
        .err => |_| {
            // error found at index 0: expected hello, found helo
            try testing.expectEqual(@as(usize, 0), e.index);
            try testing.expectEqualStrings("hello", e.expected);
            try testing.expectEqualStrings("helo", e.found);
        },
    }
}
```

To make a more complex building block, we can have a sequence type, which we can use something like -

```zig
const parser = sequence(&.{ str("hello"),str(" "), str("world") });

// inside the tagged union

.sequence => |parsers| {
    const checkpoint = state.index;
    for (parsers) |parser| {
        _ = parser.parse(state) catch |err| {
            state.index = checkpoint;
            return err;
        };
    }
}
```

### Other Basic Building Blocks
