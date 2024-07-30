pub mod parser;
pub mod encode;
pub mod header;
pub mod value;
pub mod macros;
pub mod children;

use std::str::FromStr;

use children::Children;
use header::Header;


#[derive(Debug)]
pub struct Svg{
    header: Header,
    height: u32,
    width: u32,
    view_box: (u32, u32, u32, u32),
    fill: String,
    xmlns: String,
    children: Vec<Children>
}

impl FromStr for Svg{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, header) = Header::from_str(s).unwrap();
        
        Ok(Svg{
            header,
            height: todo!(),
            width: todo!(),
            view_box: todo!(),
            fill: todo!(),
            xmlns: todo!(),
            children: todo!(),
        })
    }
    
}


#[cfg(test)]
mod test_svg{
    use std::fs::read_to_string;

    #[test]
    fn test_svg(){
        use super::Svg;
        let s = read_to_string("E:/Rust/try/makepad/Gen-UI/gen/components/resources/icons/alarm.svg").unwrap();
        let svg: Svg = s.parse().unwrap();
        dbg!(svg);
    }
}

