use crate::utils::parse_rules_cli;
use air_r_parser::RParserOptions;

#[derive(Clone)]
pub struct Config<'a> {
    pub rules: Vec<&'a str>,
    pub should_fix: bool,
    pub parser_options: RParserOptions,
}

pub fn build_config(rules_cli: &str, should_fix: bool, parser_options: RParserOptions) -> Config {
    let rules = parse_rules_cli(rules_cli);

    Config { rules, should_fix, parser_options }
}
