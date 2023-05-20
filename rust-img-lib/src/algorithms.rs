pub mod iconpacker;

use crate::utils::PackError;

/// A struct that represents a rectangle for the packing algorithm.
#[derive(Clone, Copy, Debug)]
pub struct PackingRectangle
{
    pub width: u32,
    pub height: u32,
    pub id: u64
}

/// A trait representing a generic Packing Algorithm.
/// This trait allows for any packing algorithm to be provided as a "backend" to pack the Rectangles
pub trait Packer
{
    /// The function that runs the packing algorithm on the given `PackingRect`s. 
    /// Outputs a 3-tuple consisting of the final width, final height and a Vector of `FitRects`, which contain the items that were being packed
    fn pack(&mut self) -> Result<(u32, u32, Vec<FitRect>), PackError>;
}


/// A struct representing the placement of a rectangle in the final spritesheet.
#[derive(Debug)]
pub struct FitRect
{
    /// `x` position of where the image must be placed
    pub x: u32,
    /// `y` position of where the image must be placed
    pub y: u32,
    /// width of the image
    pub width: u32,
    /// height of the image
    pub height: u32,
    /// Id which is used to keep track of which image this `FitRect` represents.
    pub id: u64
}

impl FitRect
{
    /// Constructs a new `FitRect`
    pub fn new(x: u32, y: u32, width: u32, height: u32, id: u64) -> Self
    {
        Self { x, y, width, height, id }
    }
}
