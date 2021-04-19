//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-saturate-out-of-range.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\r\
             \n  bar: saturate(red, 125);\r\
             \n}"
        )
        .unwrap_err(),
        "Error: $amount: Expected 125 to be within 0 and 100.\
         \n  ,\
         \n2 |   bar: saturate(red, 125);\
         \n  |        ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
