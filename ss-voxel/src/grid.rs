use std::vec;

use bevy::prelude::UVec3;

/// Uncompressed 3d grid of voxels packed as indexed array.
#[derive(Debug, Clone)]
pub struct Grid<T> {
    /// Size of grid
    pub size: UVec3,
    /// X,Z,Y ordered linear. 0 means empty.
    pub indecies: Vec<u32>,
    /// Store of values that are indexed in indecies
    pub palette: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    /// Create empty grid of specified size
    pub fn new(size: UVec3) -> Self {
        Grid {
            size,
            indecies: vec![0; vec_volume(size)],
            palette: vec![],
        }
    }

    /// Generate grid from function that iterates over each voxel
    pub fn from_function<F>(size: UVec3, mut body: F) -> Self
    where
        F: FnMut(UVec3) -> Option<T>,
    {
        let mut indecies = vec![0; vec_volume(size)];
        let mut palette = vec![];

        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    let vi = UVec3::new(x, y, z);
                    if let Some(v) = body(vi) {
                        let existing = palette.iter().position(|e| e == &v);
                        if let Some(i) = existing {
                            indecies[linear_index(size, vi)] = i;
                        } else {
                            let i = palette.len();
                            indecies[linear_index(size, vi)] = i;
                            palette.push(v);
                        }
                    }
                }
            }
        }

        Grid {
            size,
            indecies: vec![0; vec_volume(size)],
            palette: vec![],
        }
    }
}

fn vec_volume(v: UVec3) -> usize {
    v.x as usize * v.y as usize * v.z as usize
}

fn linear_index(size: UVec3, v: UVec3) -> usize {
    let sx = size.x as usize;
    let sz = size.z as usize;
    v.x as usize + v.z as usize * sx + v.y as usize * (sx * sz)
}
