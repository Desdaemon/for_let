use for_let::for_let;

fn main() {
    let foos = [None, None, None, Some("foo")];

    // Without the macro
    for ref foo in foos {
        if let Some(foo) = foo {
            if foo == &"foo" {
                println!("Got a 'foo'!");
            }
        }
    }

    // A bit more concise
    for ref foo in foos {
        if let Some("foo") = foo {
            println!("Got a 'foo'!");
        }
    }

    // But we can do better!
    for_let!(Some("foo") in foos {
        println!("Got a 'foo'!");
    });

    // Guards are fair game
    for_let!(Some(foo) if foo == "foo" in foos {
        println!("Got a 'foo'!");
    });

    // For comparison, a filter chain
    foos.iter()
        .filter(|foo| matches!(foo, Some("foo")))
        .for_each(|_| println!("Got a 'foo'!"));
}
