use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[grammar = "lua.pest"]
struct LuaParser;

fn main() -> Result<()> {
    let mut file = File::open("english.lua")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut parse = LuaParser::parse(Rule::tlfile, &contents)?;
    //eprint!("{:#?}", parse);
    // Allocate a bit extra for the result
    let mut output = String::with_capacity(contents.len() * 11 / 10);
    process(parse.next().unwrap(), &mut output);
    print!("{}", output);
    Ok(())
}

fn process(pair: Pair<Rule>, str: &mut String) {
    //println!("{}", pair);
    match pair.as_rule() {
        Rule::tlfile => {
            pair.into_inner().for_each(|p| process(p, str));
        }
        Rule::createlang | Rule::comment => {
            str.push_str(pair.as_str());
        }
        Rule::tlstr => {
            let mut items = pair.into_inner();
            let key = items.next().unwrap();
            let value = items.next().unwrap();
            str.push_str("L.");
            str.push_str(key.as_str());
            str.push_str(" = ");
            process(value, str);
            str.push('\n');
        }
        Rule::newline => str.push('\n'),
        Rule::tlkey => todo!(),
        Rule::tl_value => unreachable!(),
        Rule::tl_singleline_value => {
            str.push_str(pair.as_str());
        }
        Rule::tl_multiline_value => {
            str.push_str(pair.as_str());
        }
        Rule::EOI => (),
        Rule::comment_contents => unreachable!(),
    }
}
