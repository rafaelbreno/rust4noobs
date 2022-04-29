## 00 - Intro

### Summary
- [Repository Introduction](#repository-introduction)
- [Rust Introduction](#rust-introduction)
- [How to install](#how-to-install)

### Repository Introduction
- This repository will be written in the following structure:
```
## Chapter

### Summary
- [Foo](#foo)

### Foo
- Some knowledge
\```
/*
 * Example here
**/

\```

### References
- [Link 1](example.com)
```
- This is a on-rolling repo, meaning that it will be updating as goes.

### Rust Introduction
_"The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control."_

### How To Install
- I (Rafael, the writer) am using _Arch Linux_ as my OS, and to install Rust I'll be using [asdf](https://asdf-vm.com/)
- Adding the plugin:
  - > asdf plugin-add rust https://github.com/code-lever/asdf-rust.git
- Installing the plugin:
  - > asdf install rust latest
- Setting the version:
  - > asdf global rust latest

### References
- [Introduction](https://doc.rust-lang.org/book/ch00-00-introduction.html)
