// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2025 yyxxryrx.
/*!
* # Quick Timer
*
* > A simple timer macro for Rust.
*
* ## Usage
* ```toml
* [dependencies]
* quick-timer = "0.1.0"
* ```
*
* ## Example
* ```rust
* use quick_timer::{timer, timer_silent};
* use std::any::Any;
* fn main() {
*     // timer macro will don't timing in release, it will only return the result of the code block
*     // if you want it run in release mode, you can add `release_also` feature in your Cargo.toml
*     // No Tag
*     timer! {
*         println!("You can do somethings here");
*     }
*
*     timer!({
*         println!("You can do somethings here");
*     });
*
*     timer!(block: {
*         println!("You can do somethings here");
*     });
*
*     // With Tag
*     timer!(# Tag {
*         println!("You can do somethings here")
*     });
*
*     timer!(# "A Tag" {
*         println!("You can do somethings here")
*     });
*
*     timer! {
*         # Tag
*         println!("You can do somethings here")
*     }
*
*     timer! {
*         # "A Tag"
*         println!("You can do somethings here")
*     }
*
*     timer!(tag: Tag, block: {
*         println!("You can do somethings here");
*     });
*
*     timer!(tag: "A Tag", block: {
*         println!("You can do somethings here");
*     });
*
*     let result = timer! {
*         // And you can return the result
*         1 + 1
*     };
*     assert_eq!(result, 2);
*
*     // timer_silent macro will do timing in release and debug, and will return the result of the code block
*     // Don't print the duration, return it
*     let (result, duration) = timer_silent! {
*         println!("You can do somethings here");
*         // you can return the result too
*         1 + 1
*     };
*
*     assert_eq!(result, 2);
*     assert_eq!(
*         duration.type_id(),
*         std::any::TypeId::of::<std::time::Duration>()
*     );
* }
* ```
*/

#[macro_export]
#[cfg(any(debug_assertions, feature = "release_also"))]
/// Times the execution of a code block in debug mode or when `release_also` feature is enabled.
///
/// This macro measures the execution time of a code block and prints the result to stdout.
/// In release mode without the `release_also` feature, this macro is disabled and simply
/// executes the code block without timing.
///
/// # Syntax
///
/// ```rust
/// // Basic usage - times a block of code
/// use quick_timer::timer;
///
/// timer! {
///     // your code here
/// }
///
/// // With a custom tag
/// timer!(# "My Tag" {
///     // your code here
/// });
///
/// // Alternative syntax
/// timer! {
///     # "My Tag"
///     // your code here
/// }
///
/// // Alternative syntax with tag
/// timer!(tag: "My Tag", block: {
///     // your code here
/// });
/// ```
///
/// # Examples
///
/// ```
/// use quick_timer::timer;
///
/// // Basic usage
/// timer! {
///     let x = 1 + 2;
///     println!("Computed: {}", x);
/// }
///
/// // With a tag
/// timer!(# "Calculation" {
///     let y = 3 * 4;
///     println!("Computed: {}", y);
/// });
///
/// // With result
///
/// let result = timer! {
///     1 + 1
/// };
///
/// assert_eq!(result, 2);
/// ```
macro_rules! timer {
    // Times a block with a literal string tag
    (tag: $tag:literal, block: $block:block) => {{
        let line = line!();
        let start = ::std::time::Instant::now();
        let result = $block;
        println!(
            "in {} line {} {}: {} ms",
            file!(),
            line,
            $tag,
            start.elapsed().as_millis()
        );
        result
    }};
    // Times a block with an identifier tag
    (tag: $tag:ident, block: $block:block) => {{
        let line = line!();
        let start = ::std::time::Instant::now();
        let result = $block;
        println!(
            "in {} line {} {}: {} ms",
            file!(),
            line,
            stringify!($tag),
            start.elapsed().as_millis()
        );
        result
    }};
    // Times a block with default "Timer" tag
    (block: $block:block) => {
        $crate::timer!(tag: "Timer", block: $block)
    };
    // Times a block with a literal string tag using shorthand syntax
    (#$tag:literal $block:block) => {
        $crate::timer!(tag: $tag, block: $block)
    };
    // Times a block with an identifier tag using shorthand syntax
    (#$tag:ident $block:block) => {
        $crate::timer!(tag: $tag, block: $block)
    };
    // Times a block with a literal string tag using shorthand syntax (braceless form)
    (#$tag:literal $($tt:tt)*) => {
        $crate::timer!(tag: $tag, block: {
            $($tt)*
        })
    };
    // Times a block with an identifier tag using shorthand syntax (braceless form)
    (#$tag:ident $($tt:tt)*) => {
        $crate::timer!(tag: $tag, block: {
            $($tt)*
        })
    };
    // Times a block with default tag (braceless form)
    ($($tt:tt)*) => {
        $crate::timer!(block: {
            $($tt)*
        })
    };
}

#[macro_export]
#[cfg(not(any(debug_assertions, feature = "release_also")))]
/// Times the execution of a code block in debug mode or when `release_also` feature is enabled.
///
/// This macro measures the execution time of a code block and prints the result to stdout.
/// In release mode without the `release_also` feature, this macro is disabled and simply
/// executes the code block without timing.
///
/// # Syntax
///
/// ```rust
/// // Basic usage - times a block of code
/// timer! {
///     // your code here
/// }
///
/// // With a custom tag
/// timer!(# "My Tag" {
///     // your code here
/// });
///
/// // Alternative syntax with tag
/// timer!(tag: "My Tag", block: {
///     // your code here
/// });
/// ```
///
/// # Examples
///
/// ```
/// use quick_timer::timer;
///
/// // Basic usage
/// timer! {
///     let x = 1 + 2;
///     println!("Computed: {}", x);
/// }
///
/// // With a tag
/// timer!(# "Calculation" {
///     let y = 3 * 4;
///     println!("Computed: {}", y);
/// });
///
/// // With result
///
/// let result = timer! {
///     1 + 1
/// };
///
/// assert_eq!(result, 2);
/// ```
macro_rules! timer {
    // Executes a block without timing (literal tag version)
    (tag: $tag:literal, block: $block:block) => {
        $block
    };
    // Executes a block without timing (identifier tag version)
    (tag: $tag:ident, block: $block:block) => {
        $block
    };
    // Executes a block without timing (default block version)
    (block: $block:block) => {
        $block
    };
    // Executes a block without timing (duplicate rule for consistency)
    (block: $block:block) => {
        $crate::timer!(block: $block)
    };
    // Executes a block without timing (shorthand literal tag syntax)
    (#$tag:literal $block:block) => {
        $crate::timer!(block: $block)
    };
    // Executes a block without timing (shorthand identifier tag syntax)
    (#$tag:ident $block:block) => {
        $crate::timer!(block: $block)
    };
    // Executes a block without timing (shorthand literal tag syntax, braceless form)
    (#$tag:literal $($tt:tt)*) => {
        $crate::timer!(block: {
            $($tt)*
        })
    };
    // Executes a block without timing (shorthand identifier tag syntax, braceless form)
    (#$tag:ident $($tt:tt)*) => {
        $crate::timer!(block: {
            $($tt)*
        })
    };
    // Executes a block without timing (default braceless form)
    ($($tt:tt)*) => {
        $crate::timer!(block: {
            $($tt)*
        })
    };
}

#[macro_export]
/// Times the execution of a code block and returns both the result and the duration.
///
/// This macro measures the execution time of a code block and returns a tuple containing
/// the result of the code block and the duration of its execution. Unlike the `timer!` macro,
/// `timer_silent!` always performs timing regardless of the build configuration and does not
/// print anything to stdout.
///
/// # Syntax
///
/// ```rust
/// // Basic usage - times a block of code
/// let (result, duration) = timer_silent! {
///     // your code here
/// };
/// ```
///
/// # Examples
///
/// ```
/// use quick_timer::timer_silent;
///
/// // Basic usage
/// let (result, duration) = timer_silent! {
///     let x = 1 + 2;
///     x * 3
/// };
/// assert_eq!(result, 9);
/// println!("Execution took {:?}", duration);
///
/// // With side effects
/// let (result, duration) = timer_silent! {
///     println!("This will be printed");
///     42
/// };
/// assert_eq!(result, 42);
/// ```
macro_rules! timer_silent {
    (block: $block:block) => {{
        let start = ::std::time::Instant::now();
        let result = $block;
        (result, start.elapsed())
    }};
    ($block:block) => {
        $crate::timer_silent!(block: $block)
    };
    ($($tt:tt)*) => {
        $crate::timer_silent!(block: {
            $($tt)*
        })
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn test_timer() {
        timer!(block: {
            println!("Hello, world!");
        });
    }

    #[test]
    fn test_timer_silent() {
        let (result, duration) = timer_silent!(block: {
            println!("Hello, world!");
        });
        println!("Duration: {:?}", duration);
        assert_eq!(result, ());
        assert_eq!(
            duration.type_id(),
            std::any::TypeId::of::<std::time::Duration>()
        );
    }
}
