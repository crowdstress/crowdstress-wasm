use crowdstress_common::primitives::{Point, Section};

#[derive(Serialize, Deserialize)]
pub struct Human {
    pub coords: Point,
    pub panic: u8,
    pub target_section: Option<Section>,
    pub passed_exits: Vec<String>,
}
