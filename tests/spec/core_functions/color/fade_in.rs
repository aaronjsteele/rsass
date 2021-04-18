//! Tests auto-converted from "sass-spec/spec/core_functions/color/fade_in.hrx"

mod error {
    mod bounds {

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "alpha", error tests are not supported yet.

        // Ignoring "color", error tests are not supported yet.
    }
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: fade-in(rgba(red, 0.5), 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        crate::rsass(
            "a {b: fade-in(rgba(red, 0.5), 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: fade-in(rgba(red, 0.5), 0.14)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 0, 0.64);\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: fade-in(rgba(red, 0.5), 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: fade-in($color: rgba(red, 0.5), $amount: 0.14)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 0, 0.64);\
        \n}\
        \n"
    );
}
#[test]
fn opacify() {
    assert_eq!(
        crate::rsass(
            "a {b: opacify($color: rgba(red, 0.5), $amount: 0.14)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 0, 0.64);\
        \n}\
        \n"
    );
}