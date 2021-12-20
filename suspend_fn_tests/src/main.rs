use std::sync::BarrierWaitResult;

#[macro_use]
extern crate no_await;

macro_rules! pair {
    ($i1:ident, $i2:ident, $ty:ty, $body: block) => {
        async fn $i1() -> $ty {
            $body
        }
        fn $i2() -> $ty {
            $body
        }
    };
}

struct Foo<T: std::future::Future> {
    bar: T,
}
async fn foo() -> Foo<std::future::Ready<()>> {
    Foo {
        bar: std::future::ready(()),
    }
}

pair!(async_array, sync_array, [u8; 2], { rand::random() });

fn assert(a: &[u8]) {
    assert!(!a.is_empty())
}

#[suspend]
fn test_if_else() -> Result<(), reqwest::Error> {
    for i in async_array() {
        assert(&async_array());
        assert(&sync_array());
        println!("forloop works")
    }

    for i in sync_array() {
        assert(&async_array());
        assert(&sync_array());
        println!("forº works")
    }

    if async_array().is_empty() {
        assert(&async_array());
        assert(&sync_array());
        println!("else if works");
    } else if true {
        assert(&async_array());
        assert(&sync_array());
        println!("else if works");
    } else {
        assert(&async_array());
        assert(&sync_array());
        println!("else if works");
    }

    match () {
        () if async_array().is_empty() => {}
        () if sync_array().is_empty() => {}
        _ => {
            assert(&async_array());
            assert(&sync_array());
            println!("match works");
        }
    }

    let _ = loop {
        assert(&async_array());
        assert(&sync_array());
        println!("loop works");
        if true {
            break async_array();
        } else {
            break sync_array();
        }
    };

    while !async_array().is_empty() {
        assert(&async_array());
        assert(&sync_array());
        println!("while works");
        break;
    }

    while !sync_array().is_empty() {
        async_array().len();
        sync_array().len();
        println!("while works");
        break;
    }

    while let _ = !async_array().is_empty() {
        assert(&async_array());
        assert(&sync_array());
        println!("while works");
        break;
    }

    while let _ = !sync_array().is_empty() {
        async_array().len();
        sync_array().len();
        println!("while works");
        break;
    }

    if let _ = async_array().len() {
        async_array().len();
        sync_array().len();
        println!("if let works");
    } else if let _ = sync_array().len() {
        async_array().len();
        sync_array().len();
        println!("if let works");
    } else {
        async_array().len();
        sync_array().len();
        println!("if let works");
    }
    let matrix = [async_array(), sync_array()];

    static VOID: () = {
        const fn void() {}
        void()
    };

    const _: () = {
        const fn void() {}
        void()
    };
    (async_array().len(), sync_array().len());
    (async_array().len());
    (sync_array().len());
    async {
        println!("async blocks are immediately executed?");
    };

    {
        async_array().is_empty();
        sync_array().is_empty();
    }
    async fn async_fn() {}
    struct Foo<T: std::future::Future> {
        bar: T,
    }
    let x = foo().bar;
    println!("{:?}", x);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("{:?}", test_if_else().await);
}

