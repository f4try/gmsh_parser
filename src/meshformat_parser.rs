use std::error::Error;
pub struct MeshFormatParser;
#[derive(Debug)]
pub struct MeshFormat{
    pub version:String,
    pub filetype:usize,
    pub data_size:usize
}

impl MeshFormatParser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$MeshFormat"
    }
    pub fn parse_meshformat<'a>(lines:&'a [&'a str]) -> Result<(MeshFormat,&'a [&'a str]),MeshFormatParseError>{
        if !Self::is_start(lines[0]){
            return Err(MeshFormatParseError)
        }
        let meshformat = MeshFormat::from_line(lines[1])?;
        // println!("************");
        if !Self::is_end(lines[2]){
            return Err(MeshFormatParseError)
        }
        Ok((meshformat,&lines[3..]))
    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndMeshFormat"
    }
}

impl MeshFormat{
    pub fn from_line(line:&str) -> Result<Self,MeshFormatParseError>{
        // Ok(Self{version:(4,2),filetype:0,data_size:8})
        let mut parsed_str_iter = line.split_whitespace();
        let version = parsed_str_iter.next().ok_or(MeshFormatParseError)?;
        let filetype: usize = parsed_str_iter
            .next()
            .ok_or(MeshFormatParseError)
            .map(|dim_str| dim_str.parse::<usize>().map_err(|_| MeshFormatParseError))??;
        let data_size: usize = parsed_str_iter
            .next()
            .ok_or(MeshFormatParseError)
            .map(|tag_str| tag_str.parse::<usize>().map_err(|_| MeshFormatParseError))??;
        // println!("************");
        Ok(Self {
            version: version.to_owned(),
            filetype:filetype,
            data_size:data_size
        })
    }
}
use std::fmt;
#[derive(Debug)]
pub struct MeshFormatParseError;
impl fmt::Display for MeshFormatParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse mesh format")
    }
}
impl Error for MeshFormatParseError {}
