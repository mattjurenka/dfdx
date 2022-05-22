use crate::prelude::*;
use rand::{distributions::Distribution, Rng};

pub trait Randomize {
    fn randomize<R: Rng, D: Distribution<f32>>(&mut self, rng: &mut R, dist: &D);
}

macro_rules! tensor_impl {
    ($typename:ident, [$($Vs:tt),*]) => {
impl<$(const $Vs: usize, )* H> Randomize for $typename<$($Vs, )* H> {
    fn randomize<R: Rng, D: Distribution<f32>>(&mut self, rng: &mut R, dist: &D) {
        <Self as HasDevice>::Device::fill(self.mut_data(), &mut || dist.sample(rng));
    }
}
    };
}

tensor_impl!(Tensor0D, []);
tensor_impl!(Tensor1D, [M]);
tensor_impl!(Tensor2D, [M, N]);
tensor_impl!(Tensor3D, [M, N, O]);
tensor_impl!(Tensor4D, [M, N, O, P]);

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use rand_distr::Standard;

    #[test]
    fn test_randomize() {
        let mut t = Tensor1D::<100>::zeros();
        assert_eq!(t.data(), &[0.0; 100]);

        t.randomize(&mut thread_rng(), &Standard);
        assert!(t.data() != &[0.0; 100]);
    }
}
