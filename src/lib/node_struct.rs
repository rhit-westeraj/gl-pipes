use super::util::*;

fn to_vec_idx(coord: Coordinate, size: Coordinate) -> usize {
    let (x, y, z) = coord;
    let (x_size, y_size, z_size) = size;
    (x * y_size * z_size + y * z_size + z) as usize
}

pub struct NodeStruct {
    pub size: Coordinate,
    pub array: Vec<bool>,
}

impl NodeStruct {
    pub fn new(size: Coordinate) -> NodeStruct {
        if (size.0 <= 0 || size.1 <= 0 || size.2 <= 0) {
            panic!(
                "Attempting to create a NodeStruct with negative/zero size: {:?}",
                size
            );
        }
        let vec_size = size.0 * size.1 * size.2;
        let mut vec = Vec::with_capacity(vec_size as usize);
        for _ in 0..vec_size {
            vec.push(false);
        }
        NodeStruct {
            size: size,
            array: vec,
        }
    }

    pub fn get(&self, coord: Coordinate) -> bool {
        let (x, y, z) = coord;
        let (x_size, y_size, z_size) = self.size;
        if x < 0 || y < 0 || z < 0 || x >= x_size || y >= y_size || z >= z_size {
            panic!(
                "Attempting to access coordinate out of bounds: {:?}, this is of size {:?}",
                coord, self.size
            );
        }
        self.array[(x * y_size * z_size + y * z_size + z) as usize]
    }

    pub fn set(&mut self, coord: Coordinate, val: bool) {
        let (x, y, z) = coord;
        let (x_size, y_size, z_size) = self.size;
        if x < 0 || y < 0 || z < 0 || x >= x_size || y >= y_size || z >= z_size {
			panic!(
				"Attempting to access coordinate out of bounds: {:?}, this is of size {:?}",
				coord, self.size
			);
        }
        self.array[(x * y_size * z_size + y * z_size + z) as usize] = val;
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn node_struct_new_all_zero_input_panic() {
        let result = panic::catch_unwind(|| {
            super::NodeStruct::new((0, 0, 0));
        });
        assert!(result.is_err());
    }

    #[test]
    fn node_struct_new_any_zero_input_panic() {
        let result = panic::catch_unwind(|| {
            super::NodeStruct::new((1, 0, 1));
        });
        assert!(result.is_err());
    }

    #[test]
    fn node_struct_new_negative_input_panic() {
        let result = panic::catch_unwind(|| {
            super::NodeStruct::new((-1, 1, 1));
        });
        assert!(result.is_err());
    }

    #[test]
    fn node_struct_new_positive_input_no_panic() {
        let result = panic::catch_unwind(|| {
            super::NodeStruct::new((1, 1, 1));
        });
        assert!(result.is_ok());
    }

    #[test]
    fn node_struct_get_out_of_bounds_panic() {
        let mut node_struct = super::NodeStruct::new((1, 1, 1));
        let result = panic::catch_unwind(|| {
            node_struct.get((1, 1, 1));
        });
        assert!(result.is_err());
    }

    #[test]
    fn node_struct_get_in_bounds_no_panic() {
        let mut node_struct = super::NodeStruct::new((1, 1, 1));
        let result = panic::catch_unwind(|| {
            node_struct.get((0, 0, 0));
        });
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic]
    fn node_struct_set_out_of_bounds_panic() {
        let mut node_struct = super::NodeStruct::new((1, 1, 1));
        node_struct.set((1, 1, 1), true);
    }

    #[test]
    fn node_struct_set_in_bounds_no_panic() {
        let mut node_struct = super::NodeStruct::new((1, 1, 1));
        node_struct.set((0, 0, 0), true);
    }

    #[test]
    fn node_struct_set_in_bounds_get_same_value() {
        let mut node_struct = super::NodeStruct::new((1, 1, 1));
        node_struct.set((0, 0, 0), true);
        assert_eq!(node_struct.get((0, 0, 0)), true);
    }

    #[test]
    fn node_struct_larger_size_set_in_bounds_get_same_value() {
        let mut node_struct = super::NodeStruct::new((2, 2, 2));
        node_struct.set((1, 1, 1), true);
        assert_eq!(node_struct.get((1, 1, 1)), true);
    }

    #[test]
    fn node_struct_larger_size_set_multiple_in_bounds_get_same_value() {
        let mut node_struct = super::NodeStruct::new((4, 4, 4));
        node_struct.set((2, 2, 2), true);
        node_struct.set((1, 1, 0), true);
        assert_eq!(node_struct.get((2, 2, 2)), true);
        assert_eq!(node_struct.get((1, 1, 0)), true);
    }
}
