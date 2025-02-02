//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("from_other/extend/_other.scss", "a {@extend missing}\n")
        .mock_file("from_other/runtime/_other.scss", "a {b: 1px + 1em}\n")
        .mock_file("from_other/syntax/_other.scss", "a {b: }\n")
        .mock_file("load/loop/_other.scss", "@use \"sass:meta\";\n@include meta.load-css(\"input\");\n")
        .mock_file("member/global/_other.scss", "$c: d;\n")
        .mock_file("member/namespace/_other.scss", "$c: d;\n")
        .mock_file("with/conflict/_left.scss", "$a: left;\n")
        .mock_file("with/conflict/_midstream.scss", "@use \"left\" as *;\n@use \"right\" as *;\n\n$a: c !default;\n")
        .mock_file("with/conflict/_right.scss", "$a: right;\n")
        .mock_file("with/multi_configuration/double_load/both_configured/_other.scss", "$a: c !default;\n")
        .mock_file("with/multi_configuration/double_load/through_forward/_forwarded.scss", "// This file defines no variables, but it still may not be loaded both with and\n// without configuration.\n")
        .mock_file("with/multi_configuration/double_load/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: c !default;\n")
        .mock_file("with/multi_configuration/double_load/unconfigured_first/_other.scss", "$a: c !default;\n")
        .mock_file("with/multi_configuration/use_and_load/both_configured/_other.scss", "$a: c !default;\n")
        .mock_file("with/multi_configuration/use_and_load/load_first/_loads.scss", "@use \"sass:meta\";\n@include meta.load-css(\"other\", $with: (a: b));\n")
        .mock_file("with/multi_configuration/use_and_load/load_first/_other.scss", "$a: c !default;\n")
        .mock_file("with/multi_configuration/use_and_load/through_forward/_forwarded.scss", "// This file defines no variables, but it still may not be loaded both with and\n// without configuration.\n")
        .mock_file("with/multi_configuration/use_and_load/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: c !default;\n")
        .mock_file("with/multi_configuration/use_and_load/unconfigured_first/_other.scss", "$a: c !default;\n")
        .mock_file("with/namespace/_midstream.scss", "@use \"upstream\";\nupstream.$a: c !default;\n")
        .mock_file("with/namespace/_upstream.scss", "$a: d;\n")
        .mock_file("with/nested/_other.scss", "c {$a: d !default}\n")
        .mock_file("with/not_default/_other.scss", "$a: c;\n")
        .mock_file("with/repeated_variable/_other.scss", "$a-b: c !default;\n")
        .mock_file("with/through_forward/as/_forwarded.scss", "$a: d !default;\n")
        .mock_file("with/through_forward/as/_used.scss", "@forward \"forwarded\" as c-*;\n")
        .mock_file("with/through_forward/hide/_forwarded.scss", "$a: d !default;\n")
        .mock_file("with/through_forward/hide/_used.scss", "@forward \"forwarded\" hide $a;\n")
        .mock_file("with/through_forward/show/_forwarded.scss", "$a: d !default;\n")
        .mock_file("with/through_forward/show/_used.scss", "@forward \"forwarded\" show $b;\n")
        .mock_file("with/through_forward/with/_forwarded.scss", "$a: d !default;\n")
        .mock_file("with/through_forward/with/_used.scss", "@forward \"forwarded\" with ($a: c);\n")
        .mock_file("with/undefined/_other.scss", "// This file defines no variables.\n")
}

mod from_other {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("from_other")
    }

    #[test]
    #[ignore] // missing error
    fn extend() {
        let runner = runner().with_cwd("extend");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
            ),
            "Error: The target selector was not found.\
         \nUse \"@extend missing !optional\" to avoid this error.\
         \n  ,\
         \n1 | a {@extend missing}\
         \n  |    ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:4  load-css()\
         \n  input.scss 2:1   root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn runtime() {
        let runner = runner().with_cwd("runtime");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
            ),
            "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | a {b: 1px + 1em}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:7  load-css()\
         \n  input.scss 2:1   root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn syntax() {
        let runner = runner().with_cwd("syntax");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
            ),
            "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: }\
         \n  |       ^\
         \n  \'\
         \n  _other.scss 1:7  load-css()\
         \n  input.scss 2:1   root stylesheet",
        );
    }
}
mod load {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("load")
    }

    #[test]
    #[ignore] // wrong error
    fn test_loop() {
        let runner = runner().with_cwd("loop");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
            ),
            "Error: Module loop: input.scss is already being loaded.\
         \n  ,\
         \n2 | @include meta.load-css(\"input\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 2:1  load-css()\
         \n  input.scss 2:1   root stylesheet",
        );
    }
    #[test]
    fn missing() {
        let runner = runner().with_cwd("missing");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
            ),
            "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
mod member {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("member")
    }

    #[test]
    fn global() {
        let runner = runner().with_cwd("global");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@meta.load-css(\"other\");\n\
             \na {b: $c}\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n4 | a {b: $c}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
    #[test]
    fn namespace() {
        let runner = runner().with_cwd("namespace");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@meta.load-css(\"other\");\n\
             \na {b: other.$c}\n"
            ),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n4 | a {b: other.$c}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
}
#[test]
fn too_few_args() {
    let runner = runner().with_cwd("too_few_args");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css();\n"
        ),
        "Error: Missing argument $url.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css();\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    let runner = runner().with_cwd("too_many_args");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", (), \"a\");\n"
        ),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"other\", (), \"a\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
mod test_type {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("type")
    }

    #[test]
    fn url() {
        let runner = runner().with_cwd("url");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(1);\n"
            ),
            "Error: $url: 1 is not a string.\
         \n  ,\
         \n2 | @include meta.load-css(1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    mod with {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("with")
        }

        #[test]
        fn key() {
            let runner = runner().with_cwd("key");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (1: null));\n"
                ),
                "Error: $with key: 1 is not a string.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (1: null));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        fn map() {
            let runner = runner().with_cwd("map");
            assert_eq!(
                runner.err(
                    "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: 1);\n"
                ),
                "Error: $with: 1 is not a map.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: 1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
    }
}
mod with {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("with")
    }

    #[test]
    #[ignore] // missing error
    fn conflict() {
        let runner = runner().with_cwd("conflict");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
            ),
            "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"left\" as *;\
         \n    | ================ includes variable\
         \n2   | @use \"right\" as *;\
         \n    | ================= includes variable\
         \n... |\
         \n4   | $a: c !default;\
         \n    | ^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  _midstream.scss 4:1  load-css()\
         \n  input.scss 2:1       root stylesheet",
        );
    }
    #[test]
    fn core_module() {
        let runner = runner().with_cwd("core_module");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"sass:color\", $with: (a: b));\n"
            ),
            "Error: Built-in module sass:color can\'t be configured.\
         \n  ,\
         \n2 | @include meta.load-css(\"sass:color\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    mod multi_configuration {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("multi_configuration")
        }

        mod double_load {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("double_load")
            }

            #[test]
            #[ignore] // missing error
            fn both_configured() {
                let runner = runner().with_cwd("both_configured");
                assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ============================================== original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn through_forward() {
                let runner = runner().with_cwd("through_forward");
                assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"forwarded\");\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"forwarded\");\
         \n  | =================================== original load\
         \n3 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ================================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  load-css()\
         \n  input.scss 3:1       root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn unconfigured_first() {
                let runner = runner().with_cwd("unconfigured_first");
                assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @include meta.load-css(\"other\");\
         \n  | =============================== original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
            }
        }
        mod use_and_load {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("use_and_load")
            }

            #[test]
            #[ignore] // missing error
            fn both_configured() {
                let runner = runner().with_cwd("both_configured");
                assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"other\" with ($a: b);\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @use \"other\" with ($a: b);\
         \n  | ========================= original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn load_first() {
                let runner = runner().with_cwd("load_first");
                assert_eq!(
        runner.err(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
             \n// we begin loading `used`.\
             \n@use \"loads\";\
             \n@use \"other\" with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> input.scss\
         \n4 | @use \"other\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> _loads.scss\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ============================================== original load\
         \n  \'\
         \n  input.scss 4:1  root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn through_forward() {
                let runner = runner().with_cwd("through_forward");
                assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"forwarded\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n2 | @use \"forwarded\";\
         \n  | ================ original load\
         \n3 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ================================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  load-css()\
         \n  input.scss 3:1       root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn unconfigured_first() {
                let runner = runner().with_cwd("unconfigured_first");
                assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ============ original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
            }
        }
    }
    #[test]
    #[ignore] // wrong error
    fn namespace() {
        let runner = runner().with_cwd("namespace");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn nested() {
        let runner = runner().with_cwd("nested");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not_default() {
        let runner = runner().with_cwd("not_default");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn repeated_variable() {
        let runner = runner().with_cwd("repeated_variable");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\n"
            ),
            "Error: The variable $a-b was configured twice.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    mod through_forward {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("through_forward")
        }

        #[test]
        #[ignore] // missing error
        fn test_as() {
            let runner = runner().with_cwd("as");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn hide() {
            let runner = runner().with_cwd("hide");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn show() {
            let runner = runner().with_cwd("show");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn with() {
            let runner = runner().with_cwd("with");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // missing error
    fn undefined() {
        let runner = runner().with_cwd("undefined");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
