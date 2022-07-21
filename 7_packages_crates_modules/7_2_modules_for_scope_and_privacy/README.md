# Defining Modules to Control Scope and Privacy

`use` keyword: brings path into scope
`pub` keyword: make items public
`as` keyword: 

## Modules Quick Reference

- Private vs Public: code in module is private by default
    - `pub mod`: declares module public
- `use` keyword: 
    - creates shortcuts to items to reduce repition of long paths.
    - example:  In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with use `crate::garden::vegetables::Asparagus;` and then only need to write `Asparagus` to make use of that type in the scope.
