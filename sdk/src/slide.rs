use serde::{Deserialize, Serialize};

pub type Slides = Vec<Slide>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Slide {
    pub number: u32,
    pub src: String,
    pub caption: String,
}
