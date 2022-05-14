//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/control-else.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@if (false) {\r\
             \n} @else {\r\
             \n  @import \'_include\';\r\
             \n}\r\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n3 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:3  root stylesheet",
    );
}
