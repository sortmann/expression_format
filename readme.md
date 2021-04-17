A crate to format and print strings with an embedded rust expression.
# Examples
Using `:?` modifier.
```
use expression_format::ex_format;
let v = vec![1, 2, 3];
assert_eq!(ex_format!("v = {:?v}"), "v = [1, 2, 3]");
```
---
Printing the contents of fields.
```
use expression_format::ex_format;
let arg = ["ipsum", "sit"];
assert_eq!(ex_format!("lorem {arg[0]} dolor {arg[1]} amet"), "lorem ipsum dolor sit amet");
```
---
Short version of [`ex_format!`](macro.ex_format.html) with a complex expression.
```
use expression_format::short::exf;
assert_eq!(
    exf!(r#"Hello { { // Space after the first { since {{ is an escape sequence.
            let first = "Wo";
            let second = "rld";
            let mut result = String::from(first);
            result.push_str(second);
            result
     }}!"#),
    "Hello World!"
);
```
---
Print to standard output with a new line.
```
use expression_format::short::expl; // Short name version of ex_println!
#[derive(Debug)]
struct Point {x: i32, y: i32}
expl!("value of point = {:?Point {x: 1 + 2, y: 3 * 4 }}");
// stdout: value of point = Point { x: 3, y: 12 }
```
---
Escape brackets with `{{` and `}}`.
```
use expression_format::short::exf;
let value = 10;
assert_eq!(exf!("{{value}} = {value}"), "{value} = 10");
```