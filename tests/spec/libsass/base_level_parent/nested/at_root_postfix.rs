//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-postfix.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  @at-root {\r\
            \n    &post {\r\
            \n      foo {\r\
            \n        bar: baz;\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "testpost foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}