use std::error::Error;
pub struct EntityPaser;

impl EntityPaser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$Entities"
    }
    pub fn parse<'a>(lines:&'a [&'a str]) -> Result<(Vec<Entity>,&'a [&'a str]),EntityPaserError>{
        if !Self::is_start(&lines[0]){
            return Err(EntityPaserError)
        }
        // let [numPoints,numCurves,numSurface,numVolumes] = Self::nums_from_line(lines[1]).map_err(|_|EntityPaserError)?;
        // let is_head:bool = true;
        let mut i:usize = 2;
        let mut entities:Vec<Entity> = vec![];
        let nums = Self::nums_from_line(lines[1]).map_err(|_|EntityPaserError)?;
        let mut numPoints:usize = nums[0];
        let mut numCurves:usize = nums[1];
        let mut numSurfaces:usize = nums[2];
        let mut numVolumes:usize = nums[3];
        while !Self::is_end(&lines[i]){
            if numPoints>0{
                let entity = Entity::entity_from_line(lines[i],1)?;
                entities.push(entity); 
                numPoints-=1;
                i+=1;
                continue;
            }
            if numCurves>0{
                let entity = Entity::entity_from_line(lines[i],2)?;
                entities.push(entity); 
                numCurves-=1;
                i+=1;
                continue;
            }
            if numSurfaces>0{
                let entity = Entity::entity_from_line(lines[i],3)?;
                entities.push(entity); 
                numSurfaces-=1;
                i+=1;
                continue;
            }
            if numVolumes>0{
                let entity = Entity::entity_from_line(lines[i],4)?;
                entities.push(entity); 
                numVolumes-=1;
                i+=1;
                continue;
            }
            i+=1;
        }
        // let entities = lines[2..(num_entities+2)].iter().map(|line| Entity::from_line(line)).collect::<Result<Vec<Entity>,EntityPaserError>>()?;
        // if !Self::is_end(&lines[num_entities+2]){
        //     return Err(EntityPaserError)
        // }
        Ok((entities,&lines[i+1..]))

    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndEntities"
    }
    pub fn nums_from_line(line: &str) -> Result<[usize;4], EntityPaserError> {
        let parsed_nums = line
            .split_whitespace()
            .map(|num_str| num_str.parse().map_err(|_| EntityPaserError))
            .collect::<Result<Vec<usize>, EntityPaserError>>()?;
        let mut parsed_nums_iter = parsed_nums.into_iter();
        // let tag = parsed_nums_iter.next().ok_or(EntityPaserError)?;
        let mut nums = [0; 4];
        for i in 0..4{
            nums[i] = parsed_nums_iter.next().ok_or(EntityPaserError)?;
        }
        // for (_, num) in (0..4).zip(nums.iter_mut()) {
        //     *num = parsed_nums_iter.next().ok_or(EntityPaserError)?;
        // }
        Ok(nums)
    }
}
#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub entity: EntityType,
    pub tags:[usize;4]
}
#[derive(Debug)]
pub enum EntityType {
    Point([f64; 3]),
    Curve([f64; 6]),
    Surface([f64; 6]),
    Volume([f64; 6]),
}
use std::convert::TryFrom;
impl EntityType {
    pub fn new(entity_type: usize, node_number_list: &[f64]) -> Result<Self, EntityPaserError> {
        match entity_type {
            1 => {
                let nodes: [f64; 3] = Self::to_fixed_array(node_number_list)?;
                Ok(EntityType::Point(nodes))
            },
            2 => {
                let nodes: [f64; 6] = Self::to_fixed_array(node_number_list)?;
                Ok(EntityType::Curve(nodes))
            },
            3 => {
                let nodes: [f64; 6] = Self::to_fixed_array(node_number_list)?;
                Ok(EntityType::Surface(nodes))
            },
            4 => {
                let nodes: [f64; 6] = Self::to_fixed_array(node_number_list)?;
                Ok(EntityType::Volume(nodes))
            },
            _ => Err(EntityPaserError),
        }
    }
    fn to_fixed_array<'a, A: TryFrom<&'a [f64]>>(
        list: &'a [f64],
    ) -> Result<A, EntityPaserError> {
        TryFrom::try_from(list).map_err(|_| EntityPaserError)
    }
}

impl Entity {
    pub fn entity_from_line(line: &str,entity_type:usize) -> Result<Self, EntityPaserError> {
        let mut parsed_str_iter = line.split_whitespace();
        let id: usize = parsed_str_iter
            .next()
            .ok_or(EntityPaserError)
            .map(|dim_str| dim_str.parse::<usize>().map_err(|_| EntityPaserError))??;
        // let entity_type = parsed_nums_iter.next().ok_or(EntityPaserError)?;
        // let number_of_tags = parsed_nums_iter.next().ok_or(EntityPaserError)?;
        let num_xyz = match entity_type{
            1=>3,
            _=>6,
        };
        let mut xyz = vec![0.; num_xyz];
        for (_, i) in (0..num_xyz).zip(xyz.iter_mut()) {
                *i = parsed_str_iter
                        .next()
                        .ok_or(EntityPaserError)
                        .map(|dim_str| dim_str.parse::<f64>().map_err(|_| EntityPaserError))??;
        }
        
        let entity = EntityType::new(entity_type, &xyz)?;
        let num_tags = match entity_type{
            1=>2,
            _=>4,
        };
        // println!("*********");
        let mut tags:[usize;4] = [0;4];
        for (_, i) in (0..num_tags).zip(tags.iter_mut()) {
                *i = parsed_str_iter
                        .next()
                        .ok_or(EntityPaserError)
                        .map(|dim_str| dim_str.parse::<usize>().map_err(|_| EntityPaserError))??;
        }
        println!("*********");
        Ok(Self { id, entity,tags})
    }
}

use std::fmt;
#[derive(Debug)]
pub struct EntityPaserError;
impl fmt::Display for EntityPaserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Entity")
    }
}
impl Error for EntityPaserError {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_entity_parse_oneline() {
//         let line = "1 15 2 0 1 1";
//         Entity::from_line(line).unwrap();
//     }
//     fn test_entity_parser() {
//         let line = "1 15 2 0 1 1";
//         println!("{:?}", Entity::from_line(line));
//         panic!("");
//     }
// }
