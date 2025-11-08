use quick_timer::{timer, timer_silent};
use std::any::Any;
fn main() {
    // timer macro will don't timing in release, it will only return the result of the code block
    // if you want it run in release mode, you can add `release_also` feature in your Cargo.toml
    // No Tag
    timer! {
        println!("You can do somethings here");
    }

    timer!({
        println!("You can do somethings here");
    });

    timer!(block: {
        println!("You can do somethings here");
    });

    // With Tag
    timer!(# Tag {
        println!("You can do somethings here")
    });

    timer!(# "A Tag" {
        println!("You can do somethings here")
    });

    timer! {
        # Tag
        println!("You can do somethings here")
    }

    timer! {
        # "A Tag"
        println!("You can do somethings here")
    }

    timer!(tag: Tag, block: {
        println!("You can do somethings here");
    });

    timer!(tag: "A Tag", block: {
        println!("You can do somethings here");
    });

    let result = timer! {
        // And you can return the result
        1 + 1
    };
    assert_eq!(result, 2);

    // timer_silent macro will do timing in release and debug, and will return the result of the code block
    // Don't print the duration, return it
    let (result, duration) = timer_silent! {
        println!("You can do somethings here");
        // you can return the result too
        1 + 1
    };

    assert_eq!(result, 2);
    assert_eq!(
        duration.type_id(),
        std::any::TypeId::of::<std::time::Duration>()
    );
}
