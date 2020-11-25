use std::error::Error;
pub struct NodePaser;

impl NodePaser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$Nodes"
    }
    // pub fn start_parse_all_nodes<'a>(lines:&[&'a str]) -> Result<(Vec<Node>,Vec<&'a str>),Box<dyn Error>>{
    //     let num_nodes:usize = lines[0].parse()?;
    //     let nodes = lines[1..(num_nodes+1)].iter().map(|line| Node::from_line(line).unwrap()).collect();
    //     Ok((nodes,lines[num_nodes+1..].into()))
    // }
    pub fn parse<'a>(lines:&'a [&'a str]) -> Result<(Vec<Node>,&'a [&'a str]),NodeParseError>{
        if !NodePaser::is_start(&lines[0]){
            return Err(NodeParseError)
        }
        let nums = Self::nums_from_line(&lines[1])?;
        let numEntityBlocks = nums[0];
        let numNodes= nums[1];
        let minNodeTag = nums[2];
        let maxNodeTag = nums[3];

        let mut i = 2;
        let mut is_id:bool = true;
        // let mut is_id:bool=true;
        let mut id:usize = 0;
        let mut entity_info=[0;4];
        let mut entityDim:usize;
        let mut entityTag:usize;
        let mut parametric:usize;
        let mut numNodesInBlock:usize=0;
        let mut numNodesInBlockRemain:usize = 0;
        let mut nodes:Vec<Node> = vec![];
        // println!("************");
        while !Self::is_end(&lines[i]){
            // println!("{}:{:?}",&lines[i],numNodesInBlockRemain);
            if numNodesInBlockRemain == 0{
                entity_info = Self::nums_from_line(&lines[i])?;
                entityDim = entity_info[0];
                entityTag= entity_info[1];
                parametric= entity_info[2];
                numNodesInBlock= entity_info[3];
                numNodesInBlockRemain = numNodesInBlock;
                i+=1;
                is_id =true;
                continue;
            }
            if is_id{
                id = Self::id_from_line(&lines[i])?;
                i+=1;
                numNodesInBlockRemain-=1;
                if numNodesInBlockRemain==0{
                    is_id = false;
                    id -= numNodesInBlock;
                    numNodesInBlockRemain = numNodesInBlock;
                }
                continue;
            }
            id +=1;
            let node = Node::node_from_line(id, &lines[i])?;
            nodes.push(node);
            numNodesInBlockRemain-=1;
            i+=1;
        }

        // let num_nodes::usize = lines[1].parse().map_err(|_|NodeParseError)?;
        // let nodes = lines[2..(num_nodes+2)].iter().map(|line| Node::from_line(line).unwrap()).collect();
        // if !NodePaser::is_end(&lines[num_nodes+2]){
        //     return Err(NodeParseError)
        // }
        Ok((nodes,&lines[i+1..]))
    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndNodes"
    }
    pub fn nums_from_line(line: &str) -> Result<[usize;4],NodeParseError > {
        let parsed_nums = line
            .split_whitespace()
            .map(|num_str| num_str.parse().map_err(|_| NodeParseError))
            .collect::<Result<Vec<usize>, NodeParseError>>()?;
        let mut parsed_nums_iter = parsed_nums.into_iter();
        let mut nums = [0; 4];
        for i in 0..4{
            nums[i] = parsed_nums_iter.next().ok_or(NodeParseError)?;
        }
        Ok(nums)
    }
    pub fn id_from_line(line: &str) -> Result<usize,NodeParseError > {
        let id:usize = line.parse().unwrap();
        Ok(id)
    }
}
#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub coord: [f64; 3],
}
impl Node {
    pub fn node_from_line(id:usize,line: &str) -> Result<Self, NodeParseError> {
        let mut iter = line.split_whitespace();
        // let id_str: &str = iter.next().ok_or(NodeParseError)?;
        // let id: usize = id_str.parse().map_err(|_|NodeParseError)?;
        let mut coord = [0.0f64; 3];
        for (coord_str, coord) in iter.zip(coord.iter_mut()) {
            *coord = coord_str.parse().map_err(|_|NodeParseError)?;
        }
        Ok(Self { id, coord })
    }
}

use std::fmt;
#[derive(Debug)]
pub struct NodeParseError;
impl fmt::Display for NodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Node")
    }
}
impl Error for NodeParseError {}
