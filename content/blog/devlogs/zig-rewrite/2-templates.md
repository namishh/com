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
