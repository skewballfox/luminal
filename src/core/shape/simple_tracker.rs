use std::marker::PhantomData;

use petgraph::stable_graph::NodeIndex;

use crate::prelude::Graph;

use super::Shape;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dim {
    Known(usize),
    Unknown,
}

impl Default for Dim {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Debug, Copy)]
pub struct ShapeTracker {
    pub orig_shape: [Dim; 6],
    pub n_dims: usize,
    pub permuted_inds: [usize; 6],
}

impl Default for ShapeTracker {
    fn default() -> Self {
        Self {
            orig_shape: Default::default(),
            n_dims: 0,
            permuted_inds: [0, 1, 2, 3, 4, 5],
        }
    }
}

impl ShapeTracker {
    pub fn shape(&self) -> Vec<Dim> {
        (0..self.n_dims)
            .map(|i| self.orig_shape[self.permuted_inds[i]])
            .collect()
    }

    pub fn contiguous(mut self) -> Self {
        self.permuted_inds = Default::default();
        self
    }

    pub fn is_contiguous(&self) -> bool {
        self.permuted_inds.iter().enumerate().all(|(a, b)| a == *b)
    }

    /// Convert a logical to a physical index
    pub fn index(&self, logical_index: usize) -> Option<usize> {
        let mut m_acc = 1;
        let mut s_acc = 0;
        for i in (0..self.n_dims).rev() {
            let d = match self.orig_shape[self.permuted_inds[i]] {
                Dim::Known(n) => n,
                Dim::Unknown => panic!("Running index on an unknown dimension"),
            };
            m_acc *= d;
            s_acc += m_acc;
        }
        Some(s_acc)
    }

    pub fn n_elements(&self) -> usize {
        self.orig_shape
            .iter()
            .filter_map(|i| if let Dim::Known(n) = i { Some(n) } else { None })
            .product()
    }

    pub fn expand(&mut self, index: usize, dim: Dim) {
        self.orig_shape[self.n_dims] = dim;
        for x in index..(self.permuted_inds.len() - 1) {
            self.permuted_inds[x + 1] = self.permuted_inds[x];
        }
        self.permuted_inds[index] = self.n_dims;
        self.n_dims += 1;
    }

    pub fn remove_dim(&mut self, dim: usize) {
        for i in self.permuted_inds[dim]..(self.orig_shape.len() - 1) {
            self.orig_shape[i] = self.orig_shape[i + 1];
        }
        for i in dim..(self.orig_shape.len() - 1) {
            self.permuted_inds[i] = self.permuted_inds[i + 1];
        }
        self.n_dims -= 1;
        self.permuted_inds
            .iter_mut()
            .take(self.n_dims)
            .for_each(|i| {
                if *i > dim {
                    *i -= 1
                }
            });
    }
}

#[derive(Clone, Copy)]
pub struct GraphTensor<S: Shape> {
    pub id: NodeIndex,
    pub graph_ref: *mut Graph,
    pub(crate) _phantom: PhantomData<S>,
    pub shape: ShapeTracker,
}

// #[cfg(test)]
// mod tests {
//     use super::ShapeTracker;

//     #[test]
//     fn test_shape_tracker() {
//         let start = ShapeTracker::new(vec![Dim::Unknown, Dim::Unknown, Dim::Known(128)]); // A shape of batch x seq x embed
//                                                                                           // Strides: ([Unk[0] * Unk[1] * 128, Unk[1] * 128, 1])
//         start = start.permute([0, 2, 1]);
//         // Strides: ([Unk[0] * Unk[1] * 128, 1, Unk[1] * 128])
//         start = start.reshape() // Non-contiguous, so we need a contiguous call first
//     }
// }