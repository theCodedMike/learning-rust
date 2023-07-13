# Composite

**Composite** is a structural design pattern that allows composing objects into a tree-like structure and work with the 
it as if it was a singular object.

Composite became a pretty popular solution for the most problems that require building a tree structure. Composite's 
great feature is the ability to run methods recursively over the whole tree structure and sum up the results.


## Files and Folders
Let's try to understand the Composite pattern with an example of an operating system's file system. In the file system, 
there are two types of objects: files and folders. There are cases when files and folders should be treated to be the 
same way. This is where the Composite pattern comes in handy.

`File` and `Directory` are both of the `trait Component` with a single `search` method. For a file, it will just look 
into the contents of the file; for a folder, it will go through all files of that folder to find that keyword.


### How to Run

```bash
cargo run --example composite
```

### Output

```
Searching recursively for keyword rose in folder Folder 2
Searching for keyword rose in file File 2
Searching for keyword rose in file File 3
Searching recursively for keyword rose in folder Folder 1
Searching for keyword rose in file File 1
```

## Reference

[Composite in Rust](https://refactoring.guru/design-patterns/composite/rust/example)