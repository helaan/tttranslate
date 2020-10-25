use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[grammar = "lua.pest"]
struct LuaParser;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    // The json file contains the data that will be filled into the file
    let mut jsonfile = File::open(&args[1])?;
    let mut json = String::new();
    jsonfile.read_to_string(&mut json)?;
    let translation = serde_json::from_str::<HashMap<String, String>>(&json)?;

    // The template file shows the structure that the final file will take
    let mut tplfile = File::open(&args[2])?;
    let mut tplcontents = String::new();
    tplfile.read_to_string(&mut tplcontents)?;
    let mut parse = LuaParser::parse(Rule::tlfile, &tplcontents)?;
    //eprint!("{:#?}", parse);
    // Assume that the final output is about 110% the size of the template file
    let mut output = String::with_capacity(tplcontents.len() * 11 / 10);
    process(parse.next().unwrap(), &mut output, &translation);
    print!("{}", output);
    Ok(())
}

fn process(pair: Pair<Rule>, str: &mut String, translation: &HashMap<String, String>) {
    //println!("{}", pair);
    match pair.as_rule() {
        Rule::tlfile => {
            pair.into_inner().for_each(|p| process(p, str, translation));
        }
        Rule::createlang | Rule::comment => {
            str.push_str(pair.as_str());
        }
        Rule::tlstr => {
            let mut items = pair.into_inner();
            let key = items.next().unwrap();
            let value = items.next().unwrap();
            let key_str = key.as_str();
            str.push_str("L.");
            str.push_str(key_str);
            str.push_str(" = ");
            get_value(str, key_str, value, translation);
            str.push('\n');
        }
        Rule::newline => str.push('\n'),
        Rule::EOI => (),
        Rule::tlkey
        | Rule::tl_value
        | Rule::tl_singleline_value
        | Rule::tl_multiline_value
        | Rule::comment_contents => unreachable!(),
    }
}

fn get_value(
    str: &mut String,
    key: &str,
    tpl_value: Pair<Rule>,
    translation: &HashMap<String, String>,
) {
    let value = translation.get(key).unwrap();
    match tpl_value.as_rule() {
        Rule::tl_singleline_value => {
            str.push('"');
            str.push_str(value);
            str.push('"');
        }
        Rule::tl_multiline_value => {
            str.push_str("[[");
            str.push_str(value);
            str.push_str("]]");
        }
        _ => unreachable!(),
    }
}
