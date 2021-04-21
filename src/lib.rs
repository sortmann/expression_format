//! A crate to format and print strings with an embedded rust expression, 
//! similar to [f-string formatting in Python](https://docs.python.org/3/tutorial/inputoutput.html).
//! # Examples
//!
//! Using `:?` modifier.
//! ```
//! use expression_format::ex_format;
//! let v = vec![1, 2, 3];
//! assert_eq!(ex_format!("v = {:?v}"), "v = [1, 2, 3]");
//! ```
//!
//! ---
//!
//! Using other modifiers ([`std:fmt`](https://doc.rust-lang.org/std/fmt/index.html) for details).
//! ```
//! use expression_format::ex_format;
//! // Space after format specs if it doesn't ends in ?
//! assert_eq!(ex_format!(r#"Hello {:-<5 "x"}!"#), "Hello x----!");
//! assert_eq!(ex_format!("{:.5 12.3}"), "12.30000");
//! assert_eq!(ex_format!("{:#010x 27}!"), "0x0000001b!");
//! ```
//! No support for `*` and `$` parameters.
//! 
//! ---
//! 
//! Printing the contents of fields.
//! ```
//! use expression_format::ex_format;
//! let arg = ["ipsum", "sit"];
//! assert_eq!(ex_format!("lorem {arg[0]} dolor {arg[1]} amet"), "lorem ipsum dolor sit amet");
//! ```
//!
//! ---
//!
//! Short version of [`ex_format!`](macro.ex_format.html) with a complex expression.
//! ```
//! use expression_format::short::exf;
//!
//! assert_eq!(
//!     exf!(r#"Hello { { // Space after the first { since {{ is an escape sequence.
//!             let first = "Wo";
//!             let second = "rld";
//!             let mut result = String::from(first);
//!             result.push_str(second);
//!             result
//!      }}!"#),
//!     "Hello World!"
//! );
//! ```
//!
//! ---
//!
//! Print to standard output with a new line.
//! ```
//! use expression_format::short::expl; // Short name version of ex_println!
//! #[derive(Debug)]
//! struct Point {x: i32, y: i32}
//!
//! expl!("value of point = {:?Point {x: 1 + 2, y: 3 * 4 }}");
//! // stdout: value of point = Point { x: 3, y: 12 }
//! ```
//!
//! ---
//!
//! Escape brackets with `{{` and `}}`.
//! ```
//! use expression_format::short::exf;
//! let value = 10;
//! assert_eq!(exf!("{{value}} = {value}"), "{value} = 10");
//! ```

/// Formats and prints to std error any valid rust expression in a string.
///
/// Same as [`eprint!`](https://doc.rust-lang.org/std/macro.eprint.html) but with embedded parameters.
pub use expression_format_impl::ex_eprint;
/// Formats and prints to std error any valid rust expression in a string with a new line at the end.
///
/// Same as [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html) but with embedded parameters.
pub use expression_format_impl::ex_eprintln;
/// Formats any valid rust expression in a string.
///
/// Same as [`format!`](https://doc.rust-lang.org/std/macro.format.html) but with embedded parameters.
///
/// # Example
/// ```
/// use expression_format::ex_format;
/// let arg = "ipsum";
/// assert_eq!(ex_format!("lorem {arg}"), "lorem ipsum");
/// ```
pub use expression_format_impl::ex_format;
/// Formats and prints to std out any valid rust expression in a string.
///
/// Same as [`print!`](https://doc.rust-lang.org/std/macro.print.html) but with embedded parameters.
pub use expression_format_impl::ex_print;
/// Formats and prints to std out any valid rust expression in a string with a new line at the end.
///
/// Same as [`println!`](https://doc.rust-lang.org/std/macro.println.html) but with embedded parameters.
pub use expression_format_impl::ex_println;

/// Short name versions
pub mod short {
    /// Short name version of [`ex_eprint!`](../macro.ex_eprint.html)
    pub use expression_format_impl::ex_eprint as exep;
    /// Short name version of [`ex_format!`](../macro.ex_format.html)
    pub use expression_format_impl::ex_format as exf;
    /// Short name version of [`ex_print!`](../macro.ex_print.html)
    pub use expression_format_impl::ex_print as exp;
    /// Short name version of [`ex_println!`](../macro.ex_println.html)
    pub use expression_format_impl::ex_println as expl;
    /// Short name version of [`ex_eprintln!`](../macro.ex_eprintln.html)
    pub use expression_format_impl::ex_eprintln as exepl;
}

#[cfg(test)]
mod tests {
    use crate::short::exf;

    #[test]
    fn test_single_argument() {
        let r = "ipsum";
        assert_eq!(exf!("lorem {r} dolor"), "lorem ipsum dolor");
    }

    #[test]
    fn test_raw_string() {
        assert_eq!(exf!(r#"lorem {r"{ipsum}"}"#), "lorem {ipsum}");
    }

    #[test]
    fn test_raw_string_with_depth() {
        assert_eq!(exf!(r###"lorem {r##"{ipsum"#}"##}"###), r##"lorem {ipsum"#}"##);
    }

    #[test]
    fn test_no_argument() {
        assert_eq!(exf!("lorem ipsum"), "lorem ipsum");
    }

    #[test]
    fn test_multiple_arguments() {
        let args = ["lorem ", "ipsum ", "dolor"];
        assert_eq!(exf!("{args[0]}{args[1]}{args[2]}"), "lorem ipsum dolor");
    }

    #[test]
    fn test_string() {
        assert_eq!(exf!(r#"lorem {"ip\"sum"}"#), "lorem ip\"sum");
    }

    #[test]
    fn test_char() {
        assert_eq!(exf!(r#"lorem {'\''}ipsum'"#), "lorem 'ipsum'");
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(
            exf!(r#"lorem { {
                // line comment
                "ipsum"
            }}"#),
            "lorem ipsum"
        );
    }

    #[test]
    fn test_struct_in_expression() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        assert_eq!(
            exf!("value of point = {:?Point {x: 1 + 2, y: 3 * 4}}"),
            "value of point = Point { x: 3, y: 12 }"
        );
    }

    #[test]
    fn test_escape_brackets() {
        let arg = "lorem";
        assert_eq!(exf!("{{{arg}}} {{{{ipsum}}}}"), "{lorem} {{ipsum}}");
    }

    #[test]
    fn test_block_comment() {
        assert_eq!(exf!(
            r#"lorem {/*/*inside comment*/still inside comment*/"ipsum"}"#),
            "lorem ipsum");
    }

    #[test]
    fn test_function_with_lifetime() {
        assert_eq!(
            exf!(
                r#"{
                {
                    fn foo() -> &'static str {
                        "lorem"
                    } 
                    foo() 
                }
            } ipsum"#
            ),
            "lorem ipsum"
        );
    }

    #[test]
    fn test_number_format_width() {
        assert_eq!(
            exf!("{:04 42}"),
            "0042"
        );
    }

    #[test]
    fn test_text_format_width() {
        assert_eq!(
            exf!(r#"Hello {:5 "x"}!"#),
            "Hello x    !"
        );
    }

    #[test]
    fn test_text_format_width_left_aligned() {
        assert_eq!(
            exf!(r#"Hello {:<5 "x"}!"#),
            "Hello x    !"
        );
    }

    #[test]
    fn test_text_format_width_left_aligned_with_dash() {
        assert_eq!(
            exf!(r#"Hello {:-<5 "x"}!"#),
            "Hello x----!"
        );
    }

    #[test]
    fn test_text_format_width_center_aligned() {
        assert_eq!(
            exf!(r#"Hello {:^5 "x"}!"#),
            "Hello   x  !"
        );
    }

    #[test]
    fn test_text_format_width_right_aligned() {
        assert_eq!(
            exf!(r#"Hello {:>5 "x"}!"#),
            "Hello     x!"
        );
    }

    #[test]
    fn test_sign() {
        assert_eq!(exf!("Hello {:+ 5}!"), "Hello +5!");
    }

    #[test]
    fn test_sharp_x() {
        assert_eq!(exf!("{:#x 27}!"), "0x1b!");
    }

    #[test]
    fn test_sharp_x_with_width() {
        assert_eq!(exf!("{:#010x 27}!"), "0x0000001b!");
    }

    #[test]
    fn test_precission() {
        assert_eq!(exf!("{:.5 12.3}"), "12.30000");
    }

    #[test]
    fn test_format_alignment_with_char() {
        assert_eq!(exf!(r#"{:'>10 "test"}"#), "''''''test");
    }

    #[test]
    fn test_format_alignment_with_quotes() {
        assert_eq!(exf!(r#"{:"<10 "test"}"#), r#"test"""""""#);
    }

    #[test]
    fn test_format_alignment_with_space() {
        assert_eq!(exf!(r#"{: <10 "test"}"#), r#"test      "#);
    }
}
