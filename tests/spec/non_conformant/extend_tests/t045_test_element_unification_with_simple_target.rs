//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/045_test_element_unification_with_simple_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a .foo.bar {a: b}\
            \n*|a {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a *|a.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}