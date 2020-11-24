use gmsh_parser::*;
// use fem3d::gmsh_parser::meshformat_parser::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
fn main() -> Result<(),Box<dyn Error>>{
    // let file = File::open("hexahedron.msh")?;
    let file = File::open("rectangle.msh")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut lines:Vec<&str> = contents.lines().collect();
    // let (meshformat,rest) = MeshFormatParser::parse_meshformat(&lines)?;
    // let (physical_name,rest) = PhysicalNameParser::parse(rest)?;
    // let (entities,rest) = EntityPaser::parse(rest)?;
    // let (nodes,rest) = NodePaser::parse(rest)?;
    // let (elements,rest) = ElementPaser::parse(rest)?;
    // let test = MeshFormat::from_line("2 3 3");
    // let test = PhysicalName::from_line("2 3 \"sss3\"");
    // println!("{:?}",test);
    // println!("{:?}",rest);
    let (meshformat,physical_name,entities,nodes,elements) = parse(&lines)?;
    println!("{:?}",meshformat);
    println!("{:?}",physical_name);
    println!("{:?}",entities);
    println!("{:?}",nodes);
    println!("{:?}",elements);
    Ok(())
}
