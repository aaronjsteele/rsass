//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolation-operators-precedence.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\
             \n  a01: (#{a}+5.0% + 2);\
             \n  a02: (#{a}+ 5.0% + 2);\
             \n  a03: (#{a}  +5.0% + 2);\
             \n  a04: (#{a} +  5.0% + 2);\
             \n  b01: (5 + 2.0%+#{a});\
             \n  b02: (5 + 2.0%+ #{a});\
             \n  b03: (5 + 2.0%  +#{a});\
             \n  b04: (5 + 2.0% +  #{a});\
             \n  c01: (#{a} +5.0% + 2);\
             \n  c02: (#{a} -5.0% + 2);\
             \n  c03: (#{a} /5.0% + 2);\
             \n  c04: (#{a} *5.0% + 2);\
             \n  c05: (#{a} +5.0% - 2);\
             \n  c06: (#{a} -5.0% - 2);\
             \n  c07: (#{a} /5.0% - 2);\
             \n  c08: (#{a} *5.0% - 2);\
             \n  c09: (#{a} +5.0% / 2);\
             \n  c10: (#{a} -5.0% / 2);\
             \n  c11: (#{a} /5.0% / 2);\
             \n  c12: (#{a} *5.0% / 2);\
             \n  c13: (#{a} +5.0% * 2);\
             \n  c14: (#{a} -5.0% * 2);\
             \n  c15: (#{a} /5.0% * 2);\
             \n  c16: (#{a} *5.0% * 2);\
             \n  d01: (5 + 2.0% +#{a});\
             \n  d02: (5 + 2.0% -#{a});\
             \n  d03: (5 + 2.0% /#{a});\
             \n  d04: (5 + 2.0% *#{a});\
             \n  d05: (5 - 2.0% +#{a});\
             \n  d06: (5 - 2.0% -#{a});\
             \n  d07: (5 - 2.0% /#{a});\
             \n  d08: (5 - 2.0% *#{a});\
             \n  d09: (5% / 2.0 +#{a});\
             \n  d10: (5% / 2.0 -#{a});\
             \n  d11: (5% / 2.0 /#{a});\
             \n  d12: (5% / 2.0 *#{a});\
             \n  d13: (5 * 2.0% +#{a});\
             \n  d14: (5 * 2.0% -#{a});\
             \n  d15: (5 * 2.0% /#{a});\
             \n  d16: (5 * 2.0% *#{a});\
             \n  e01: (#{a} ==5.0% == 2);\
             \n  e02: (#{a} >5.0% > 2);\
             \n  e03: (#{a} <5.0% < 2);\
             \n  e04: (#{a} >=5.0% >= 2);\
             \n  e05: (#{a} <=5.0% <= 2);\
             \n  e06: (#{a} !=5.0% != 2);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Undefined operation \"a * 5%\".\
         \n   ,\
         \n13 |   c04: (#{a} *5.0% + 2);\
         \n   |         ^^^^^^^^^^\
         \n   \'\
         \n  input.scss 13:9  root stylesheet",
    );
}
