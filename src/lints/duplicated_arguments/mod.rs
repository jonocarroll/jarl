pub(crate) mod duplicated_arguments;

#[cfg(test)]
mod tests {
    use crate::utils_test::*;

    #[test]
    fn test_no_lint_duplicated_arguments() {
        assert!(no_lint("fun(arg = 1)", "duplicated_arguments"));
        assert!(no_lint("fun('arg' = 1)", "duplicated_arguments"));
        assert!(no_lint("fun(`arg` = 1)", "duplicated_arguments"));
        assert!(no_lint("'fun'(arg = 1)", "duplicated_arguments"));
        assert!(no_lint(
            "(function(x, y) x + y)(x = 1)",
            "duplicated_arguments"
        ));
        assert!(no_lint("dt[i = 1]", "duplicated_arguments"));
    }

    #[test]
    fn test_lint_duplicated_arguments() {
        let expected_message = "Avoid duplicate arguments in function";
        assert!(expect_lint(
            "fun(arg = 1, arg = 2)",
            expected_message,
            "duplicated_arguments"
        ));
        assert!(expect_lint(
            "fun(arg = 1, 'arg' = 2)",
            expected_message,
            "duplicated_arguments"
        ));
        assert!(expect_lint(
            "fun(arg = 1, `arg` = 2)",
            expected_message,
            "duplicated_arguments"
        ));
        assert!(expect_lint(
            "'fun'(arg = 1, arg = 2)",
            expected_message,
            "duplicated_arguments"
        ));
        assert!(expect_lint(
            "list(a = 1, a = 2)",
            expected_message,
            "duplicated_arguments"
        ));
        // TODO
        // assert!(expect_lint(
        //     "dt[i = 1, i = 2]",
        //     expected_message,
        //     "duplicated_arguments"
        // ));
    }

    #[test]
    fn test_duplicated_arguments_accepted_functions() {
        assert!(no_lint(
            "dplyr::mutate(x, a = 1, a = 2)",
            "duplicated_arguments"
        ));
        assert!(no_lint(
            "transmute(x, a = 1, a = 2)",
            "duplicated_arguments"
        ));
    }

    #[test]
    fn test_duplicated_arguments_with_interceding_comments() {
        let expected_message = "Avoid duplicate arguments in function";

        assert!(expect_lint(
            "fun(
        arg # xxx
        = 1,
        arg # yyy
        = 2
      )",
            expected_message,
            "duplicated_arguments"
        ));
        assert!(expect_lint(
            "fun(
        arg = # xxx
        1,
        arg = # yyy
        2
      )",
            expected_message,
            "duplicated_arguments"
        ));
    }
}
