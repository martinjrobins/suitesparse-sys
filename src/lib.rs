#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    clippy::all
)]

use std::default::Default;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


impl Default for klu_common {
    fn default() -> Self {
        klu_common_struct {
            tol: Default::default(),
            memgrow: Default::default(),
            initmem_amd: Default::default(),
            initmem: Default::default(),
            maxwork: Default::default(),
            btf: Default::default(),
            ordering: Default::default(),
            scale: Default::default(),
            user_order: Default::default(),
            user_data: std::ptr::null_mut(),
            halt_if_singular: Default::default(),
            status: Default::default(),
            nrealloc: Default::default(),
            structural_rank: Default::default(),
            numerical_rank: Default::default(),
            singular_col: Default::default(),
            noffdiag: Default::default(),
            flops: Default::default(),
            rcond: Default::default(),
            condest: Default::default(),
            rgrowth: Default::default(),
            work: Default::default(),
            memusage: Default::default(),
            mempeak: Default::default(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn klu_simple() {
        let n = 5i32;
        let mut Ap = vec![0, 2, 5, 9, 10, 12];
        let mut Ai = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
        let mut Ax = vec![2., 3., 3., -1., 4., 4., -3., 1., 2., 2., 6., 1.];
        let mut b = vec![8., 45., -3., 3., 19.];

        let mut Common = klu_common::default();
        unsafe { klu_defaults(&mut Common) };
        let mut Symbolic = unsafe { klu_analyze(n, Ap.as_mut_ptr(), Ai.as_mut_ptr(), &mut Common) };
        let mut Numeric = unsafe { klu_factor(Ap.as_mut_ptr(), Ai.as_mut_ptr(), Ax.as_mut_ptr(), Symbolic, &mut Common) };
        unsafe { klu_solve(Symbolic, Numeric, n, 1, b.as_mut_ptr(), &mut Common) };
        unsafe { klu_free_symbolic(&mut Symbolic, &mut Common) };
        unsafe { klu_free_numeric(&mut Numeric, &mut Common) };

        let expect = vec![1., 2., 3., 4., 5.];
        for i in 0..(n as usize) {
            assert!((b[i] - expect[i]).abs() < 1e-10);
        }
    }
}
