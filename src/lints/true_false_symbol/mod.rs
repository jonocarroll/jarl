pub(crate) mod true_false_symbol;

#[cfg(test)]
mod tests {
    use crate::utils_test::*;

    #[test]
    fn test_lint_true_false_symbol() {
        let expected_message = "can be confused with variable names";
        assert!(expect_lint("T", expected_message, "true_false_symbol"));
        assert!(expect_lint("F", expected_message, "true_false_symbol"));
        assert!(expect_lint("T = 42", expected_message, "true_false_symbol"));
        assert!(expect_lint("F = 42", expected_message, "true_false_symbol"));
        assert!(expect_lint(
            "for (i in 1:10) {x <- c(T, TRUE, F, FALSE)}",
            expected_message,
            "true_false_symbol"
        ));
        assert!(expect_lint(
            "DF$bool <- T",
            expected_message,
            "true_false_symbol"
        ));
        assert!(expect_lint(
            "S4@bool <- T",
            expected_message,
            "true_false_symbol"
        ));
        assert!(expect_lint(
            "sum(x, na.rm = T)",
            expected_message,
            "true_false_symbol"
        ));
    }

    #[test]
    fn test_no_lint_true_false_symbol() {
        assert!(no_lint("TRUE", "true_false_symbol",));
        assert!(no_lint("FALSE", "true_false_symbol",));
        assert!(no_lint("T()", "true_false_symbol",));
        assert!(no_lint("F()", "true_false_symbol",));
        assert!(no_lint("x <- \"T\"", "true_false_symbol",));
        assert!(no_lint("mtcars$F", "true_false_symbol",));
        assert!(no_lint("mtcars$T", "true_false_symbol",));
    }
    #[test]
    fn test_true_false_symbol_in_formulas() {
        let expected_message = "can be confused with variable names";
        // TODO
        // assert!(expect_lint(
        //     "lm(weight ~ var + foo(x, arg = T), data)",
        //     expected_message,
        //     "true_false_symbol"
        // ));

        assert!(no_lint("lm(weight ~ T, data)", "true_false_symbol"));
        assert!(no_lint("lm(weight ~ F, data)", "true_false_symbol"));
        assert!(no_lint("lm(weight ~ T + var", "true_false_symbol"));
        assert!(no_lint("lm(weight ~ A + T | var", "true_false_symbol"));
        assert!(no_lint("lm(weight ~ var | A + T", "true_false_symbol"));
        // TODO
        // assert!(no_lint(
        //     "lm(weight ~ var + var2 + T, data)",
        //     "true_false_symbol"
        // ));
        assert!(no_lint("lm(T ~ weight, data)", "true_false_symbol"));
    }

    // TODO
    // #[test]
    // fn test_true_false_symbol_in_function_args() {
    //     assert!(no_lint("myfun <- function(T) {}", "true_false_symbol"));
    //     assert!(no_lint("myfun <- function(F) {}", "true_false_symbol"));
    // }

    // #[test]
    // fn test_true_false_symbol_in_named_vectors() {
    //     assert!(no_lint("c(T = 'foo', F = 'foo')", "true_false_symbol"));
    // }
}
