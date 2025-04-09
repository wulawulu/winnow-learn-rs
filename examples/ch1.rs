use winnow::Result;
use winnow::Parser;


pub fn do_nothing_parser<'s>(_input :&mut &'s str)->Result<&'s str>{
    Ok("")
}

fn main(){
    let mut input = "0x1a2b Hello";

    let output = do_nothing_parser.parse_next(&mut input).unwrap();

    assert_eq!(input,"0x1a2b Hello");
    assert_eq!(output,"");
}