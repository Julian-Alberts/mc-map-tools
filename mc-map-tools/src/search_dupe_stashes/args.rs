use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct SearchDupeStashes {
    /// An area of chunks
    #[arg(short, long, value_parser=parse_area)]
    pub area: Option<Area>,
    /// The radius of chunks to be searched
    #[arg(default_value = "1")]
    pub radius: u32,
    #[command(subcommand)]
    pub mode: Option<SearchDupeStashesMode>,
}

#[derive(Debug, clap::Subcommand)]
pub enum SearchDupeStashesMode {
    /// Gives warnings for every group that has more items than the threshold in a area
    Absolute,
    /// Gives warnings for every group where the groth rate of an item group is higher than the threshold in a area.
    /// Not implemented
    GrothRate(GrothRate),
}

impl Default for SearchDupeStashesMode {
    fn default() -> Self {
        Self::Absolute
    }
}

#[derive(Debug, clap::Parser)]
pub struct GrothRate {
    #[arg(short, long)]
    file_location: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Area {
    /// X value of first point
    pub x1: i64,
    /// Z value of first point
    pub z1: i64,
    /// X value of second point
    pub x2: i64,
    /// Z value of second point
    pub z2: i64,
}

fn parse_area(value: &str) -> Result<Area, String> {
    let Some(((x1, z1), (x2, z2))) = value.split_once(';').and_then(|(pos1, pos2)| parse_point(pos1).zip(parse_point(pos2))) else {
        return Err(String::from("Can not parse provided area. Area must be give as followed: \"<x1>,<z1>;<x2>,<z2>\". Make sure that you have no spaces and all numbers are valid integers."));
    };
    Ok(Area { x1, z1, x2, z2 })
}

fn parse_point(value: &str) -> Option<(i64, i64)> {
    value
        .split_once(',')
        .and_then(|(x, z)| x.parse().ok().zip(z.parse().ok()))
}
