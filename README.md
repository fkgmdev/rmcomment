# RMCOMMENT
Quick tool to remove comments from any file, assuming comments are C style i.e.
``` rust
// let a = String::from("hello world!");
```
I made this because i was annoyed with manually clearing out commented, useless code from my projects.
Compatible with better comments, will not remove lines with specified comment types.
Usage: remcomment <filepath>
