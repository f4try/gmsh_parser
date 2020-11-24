use std::error::Error;
use crate::entity_parser::*;
pub struct ElementPaser;

impl ElementPaser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$Elements"
    }
    pub fn parse<'a>(lines:&'a [&'a str],entities:&Vec<Entity>) -> Result<(Vec<Element>,&'a [&'a str]),ElementPaserError>{
        if !Self::is_start(&lines[0]){
            return Err(ElementPaserError)
        }
        // println!("*********");
        // let [numPoints,numCurves,numSurface,numVolumes] = Self::nums_from_line(lines[1]).map_err(|_|ElementPaserError)?;
        let [numEntityBlocks,numElements,minElementTag,maxElementTag] = Self::nums_from_line(lines[1]).map_err(|_|ElementPaserError)?;
        // println!("*********");
        // let is_head:bool = true;
        let mut i:usize = 2;
        let mut elements:Vec<Element> = vec![];
        let mut entityDim:usize;
        let mut entityTag:usize=0;
        let mut elementType:usize = 2;
        let mut numElementsInBlock:usize = 0;
        while !Self::is_end(&lines[i]){
            // println!("{}:{}",&lines[i],numElementsInBlock);
            if numElementsInBlock<=0{
                let entityInfo = Self::nums_from_line(lines[i]).map_err(|_|ElementPaserError)?;
                entityDim = entityInfo[0];
                entityTag = entityInfo[1];
                elementType = entityInfo[2];
                numElementsInBlock = entityInfo[3];
                i+=1;
                continue;
            }
            let element:Element = Element::element_from_line(&lines[i],elementType,entityTag,entities)?;
            elements.push(element); 
            numElementsInBlock-=1;
            i+=1;
        }
        // let elements = lines[2..(num_elements+2)].iter().map(|line| Element::from_line(line)).collect::<Result<Vec<Element>,ElementPaserError>>()?;
        // if !Self::is_end(&lines[num_elements+2]){
        //     return Err(ElementPaserError)
        // }
        Ok((elements,&lines[i+1..]))

    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndElements"
    }
    pub fn nums_from_line(line: &str) -> Result<[usize;4], ElementPaserError> {

        let parsed_nums = line
            .split_whitespace()
            .map(|num_str| num_str.parse().map_err(|_| ElementPaserError))
            .collect::<Result<Vec<usize>, ElementPaserError>>()?;
        let mut parsed_nums_iter = parsed_nums.into_iter();
        // let tag = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        let mut nums = [0; 4];
        for i in 0..4{
            nums[i] = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        }
        // println!("*********");
        // for (_, num) in (0..4).zip(nums.iter_mut()) {
        //     *num = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        // }
        Ok(nums)
    }
}
#[derive(Debug)]
pub struct Element {
    pub id: usize,
    pub element: ElementType,
    pub tags: [usize; 3],
}
#[derive(Debug)]
pub enum ElementType {
    Line([usize; 2]),
    Triangle([usize; 3]),
    Quadrangle([usize; 4]),
    Tetrahedron([usize; 4]),
}
use std::convert::TryFrom;
impl ElementType {
    pub fn new(element_type: usize, node_number_list: &[usize]) -> Result<Self, ElementPaserError> {
        match element_type {
            1 => {
                let nodes: [usize; 2] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Line(nodes))
            },
            2 => {
                let nodes: [usize; 3] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Triangle(nodes))
            },
            3 => {
                let nodes: [usize; 4] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Quadrangle(nodes))
            },
            4 => {
                let nodes: [usize; 4] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Tetrahedron(nodes))
            },
            _ => Err(ElementPaserError),
        }
    }
    fn to_fixed_array<'a, A: TryFrom<&'a [usize]>>(
        list: &'a [usize],
    ) -> Result<A, ElementPaserError> {
        TryFrom::try_from(list).map_err(|_| ElementPaserError)
    }
}

impl Element {
    pub fn element_from_line(line: &str,element_type:usize,entity_tag:usize,entities:&Vec<Entity>) -> Result<Self, ElementPaserError> {
        let parsed_nums = line
            .split_whitespace()
            .map(|num_str| num_str.parse().map_err(|_| ElementPaserError))
            .collect::<Result<Vec<usize>, ElementPaserError>>()?;
        let mut parsed_nums_iter = parsed_nums.into_iter();
        let id = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        // let element_type = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        // let number_of_tags = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        let element = ElementType::new(element_type, &parsed_nums_iter.collect::<Vec<usize>>())?;
        for 
        let mut tags = [0,entity_tag,0];
        // for (_, tag) in (0..number_of_tags).zip(tags.iter_mut()) {
        //     *tag = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        // }
        Ok(Self { id, element,tags})
    }
}

use std::fmt;
#[derive(Debug)]
pub struct ElementPaserError;
impl fmt::Display for ElementPaserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Element")
    }
}
impl Error for ElementPaserError {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_element_parse_oneline() {
//         let line = "1 15 2 0 1 1";
//         Element::from_line(line).unwrap();
//     }
//     fn test_element_parser() {
//         let line = "1 15 2 0 1 1";
//         println!("{:?}", Element::from_line(line));
//         panic!("");
//     }
// }
