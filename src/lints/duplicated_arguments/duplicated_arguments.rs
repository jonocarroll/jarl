use crate::location::Location;
use crate::message::*;
use crate::trait_lint_checker::LintChecker;
use crate::utils::find_row_col;
use air_r_syntax::RSyntaxNode;
use air_r_syntax::*;
use biome_rowan::{AstNode, SyntaxNodeText};

pub struct DuplicatedArguments;

impl Violation for DuplicatedArguments {
    fn name(&self) -> String {
        "duplicated_arguments".to_string()
    }
    fn body(&self) -> String {
        "Avoid duplicate arguments in function calls.".to_string()
    }
}

impl LintChecker for DuplicatedArguments {
    fn check(&self, ast: &RSyntaxNode, loc_new_lines: &[usize], file: &str) -> Vec<Diagnostic> {
        let mut diagnostics: Vec<Diagnostic> = vec![];
        if ast.kind() != RSyntaxKind::R_CALL {
            return diagnostics;
        }

        let call = RCall::cast(ast.clone());
        let function = call.unwrap().function();

        let fun_name = match function.unwrap() {
            AnyRExpression::RNamespaceExpression(x) => x.right().unwrap().text(),
            AnyRExpression::RIdentifier(x) => x.text(),
            AnyRExpression::AnyRValue(x) => x.text(),
            _ => unreachable!(
                "function can only be namespace expression, identifier, or any R value"
            ),
        };

        let whitelisted_funs = ["mutate", "transmute"];
        if whitelisted_funs.contains(&fun_name.as_str()) {
            return diagnostics;
        }

        let named_args = ast
            .descendants()
            .filter(|x| {
                x.kind() == RSyntaxKind::R_ARGUMENT
                    && x.first_child().unwrap().kind() == RSyntaxKind::R_ARGUMENT_NAME_CLAUSE
            })
            .collect::<Vec<_>>();

        if named_args.len() == 0 {
            return diagnostics;
        }

        let arg_names = named_args
            .iter()
            .map(|arg| {
                arg.first_child()
                    .unwrap()
                    .first_child()
                    .unwrap()
                    .text_trimmed()
                    .to_string()
                    .replace(&['\'', '"', '`'][..], "")
            })
            .collect::<Vec<_>>();

        if has_duplicates(&arg_names) {
            let (row, column) = find_row_col(ast, loc_new_lines);
            diagnostics.push(Diagnostic {
                message: DuplicatedArguments.into(),
                filename: file.into(),
                location: Location { row, column },
                fix: Fix::empty(),
            })
        }
        diagnostics
    }
}

fn has_duplicates(v: &[String]) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::new();

    for item in v {
        if !seen.insert(item) {
            return true;
        }
    }

    false
}
