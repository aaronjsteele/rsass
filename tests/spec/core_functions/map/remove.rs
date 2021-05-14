//! Tests auto-converted from "sass-spec/spec/core_functions/map/remove.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn positional_and_named() {
        assert_eq!(
            runner().err("a {b: map-remove((c: d, e: f), c, $key: e)}\n"),
            "Error: Argument $key was passed both by position and by name.\
         \n  ,\
         \n1 | a {b: map-remove((c: d, e: f), c, $key: e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: map-remove()}\n"),
            "Error: Missing argument $map.\
         \n  ,--> input.scss\
         \n1 | a {b: map-remove()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function remove($map) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn map() {
            assert_eq!(
                runner().err("a {b: map-remove(1)}\n"),
                "Error: $map: 1 is not a map.\
         \n  ,\
         \n1 | a {b: map-remove(1)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod found {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner()
                .ok("a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 1))}\n"),
            "a {\
         \n  b: (3: 4, 5: 6);\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner()
                .ok("a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 5))}\n"),
            "a {\
         \n  b: (1: 2, 3: 4);\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner()
                .ok("a {b: inspect(map-remove((1: 2, 3: 4, 5: 6), 3))}\n"),
            "a {\
         \n  b: (1: 2, 5: 6);\
         \n}\n"
        );
    }
    mod multiple {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn all() {
            assert_eq!(
        runner().ok(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8, 9: 10), 1, 5, 9))}\n"
        ),
        "a {\
         \n  b: (3: 4, 7: 8);\
         \n}\n"
    );
        }
        #[test]
        fn some() {
            assert_eq!(
        runner().ok(
            "a {b: inspect(map-remove((1: 2, 3: 4, 5: 6, 7: 8), 1, 5, 9))}\n"
        ),
        "a {\
         \n  b: (3: 4, 7: 8);\
         \n}\n"
    );
        }
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("a {b: inspect(map-remove((c: d), c))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: inspect(map-remove($map: (c: d), $key: c))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
mod not_found {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: inspect(map-remove((), 1))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("a {b: inspect(map-remove((c: d), e, f, g))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
    #[test]
    fn no_keys() {
        assert_eq!(
            runner().ok("a {b: inspect(map-remove((c: d)))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("a {b: inspect(map-remove((c: d), d))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
}
