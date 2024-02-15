use pest::Parser;
use pest_derive::Parser;

#[derive(Debug)]
#[derive(Parser)]
#[grammar = "clojure.pest"]
pub struct LispParser;

fn main() {
    let successful_parse = LispParser::parse(Rule::lisp_expr, "( kebab-function arg1 arg2 )");
    // let successful_parse = LispParser::parse(Rule::symbol , "kebab-word");
    println!("{:#?}", successful_parse);
}
