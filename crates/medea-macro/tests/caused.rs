#![forbid(non_ascii_idents, unsafe_code)]

use medea_jason::utils::Caused;

struct MyError {}

#[test]
fn derives_for_structure() {
    #[derive(Caused)]
    #[cause(error = "MyError")]
    struct TestError;

    let err = TestError;
    assert!(err.cause().is_none());
}

#[test]
fn derives_for_enum_with_error() {
    #[derive(Caused)]
    #[cause(error = "MyError")]
    enum TestError {
        Foo,
        Bar(MyError),
    }

    let err = TestError::Foo;
    assert!(err.cause().is_none());

    let err = TestError::Bar(MyError {});
    assert!(err.cause().is_some());
}

#[test]
fn derives_for_enum_with_nested_error() {
    #[derive(Caused)]
    #[cause(error = "MyError")]
    enum CausedError {
        Baz(MyError),
    }

    #[derive(Caused)]
    #[cause(error = "MyError")]
    enum TestError {
        Foo,
        Bar(#[cause] CausedError),
    }

    let cause = CausedError::Baz(MyError {});

    let err = TestError::Foo;
    assert!(err.cause().is_none());

    let err = TestError::Bar(cause);
    assert!(err.cause().is_some());
}

#[test]
fn derives_for_non_default_name_error() {
    struct SomeError;

    #[derive(Caused)]
    #[cause(error = "SomeError")]
    enum CausedError {
        Baz(SomeError),
    }

    #[derive(Caused)]
    #[cause(error = "SomeError")]
    enum TestError {
        Foo,
        Bar(#[cause] CausedError),
    }

    let cause = CausedError::Baz(SomeError {});

    let err = TestError::Foo;
    assert!(err.cause().is_none());

    let err = TestError::Bar(cause);
    assert!(err.cause().is_some());
}
