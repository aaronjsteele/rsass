//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-alone.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@at-root {\
             \n  & {\
             \n    foo {\
             \n      bar: baz;\
             \n    }\
             \n  }\
             \n}"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n2 |   & {\
         \n  |   ^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
