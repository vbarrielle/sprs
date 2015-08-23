//! High level construction of sparse matrices by stacking, by block, ...

use std::ops::{Deref};
use std::default::Default;
use sparse::csmat::{CsMatVec, CsMatView, CompressedStorage};
use errors::SprsError;

/// Stack the given matrices into a new one, using the most efficient stacking
/// direction (ie vertical stack for CSR matrices, horizontal stack for CSC)
pub fn same_storage_fast_stack<N>(
    mats: &[CsMatView<N>]) -> Result<CsMatVec<N>, SprsError>
where N: Copy {
    if mats.len() == 0 {
        return Err(SprsError::EmptyStackingList);
    }
    let inner_dim = mats[0].inner_dims();
    if ! mats.iter().all(|x| x.inner_dims() == inner_dim) {
        return Err(SprsError::IncompatibleDimensions);
    }
    let storage_type = mats[0].storage();
    if ! mats.iter().all(|x| x.storage() == storage_type) {
        return Err(SprsError::IncompatibleStorages);
    }

    let outer_dim = mats.iter().map(|x| x.outer_dims()).fold(0, |x, y| x + y);
    let nnz = mats.iter().map(|x| x.nb_nonzero()).fold(0, |x, y| x + y);

    let mut res = CsMatVec::empty(storage_type, inner_dim);
    res.reserve_outer_dim_exact(outer_dim);
    res.reserve_nnz_exact(nnz);
    for mat in mats {
        for (_, vec) in mat.outer_iterator() {
            res = res.append_outer_csvec(vec.borrowed());
        }
    }

    Ok(res)
}

/// Construct a sparse matrix by vertically stacking other matrices
pub fn vstack<N>(mats: &[CsMatView<N>]) -> Result<CsMatVec<N>, SprsError>
where N: Copy + Default {
    if mats.iter().all(|x| x.is_csr()) {
        return same_storage_fast_stack(mats);
    }

    let mats_csr: Vec<_> = mats.iter().map(|x| x.to_csr()).collect();
    let mats_csr_views: Vec<_> = mats_csr.iter().map(|x| x.borrowed()).collect();
    same_storage_fast_stack(&mats_csr_views)
}

/// Construct a sparse matrix by horizontally stacking other matrices
pub fn hstack<N>(mats: &[CsMatView<N>]) -> Result<CsMatVec<N>, SprsError>
where N: Copy + Default {
    if mats.iter().all(|x| x.is_csc()) {
        return same_storage_fast_stack(mats);
    }

    let mats_csc: Vec<_> = mats.iter().map(|x| x.to_csc()).collect();
    let mats_csc_views: Vec<_> = mats_csc.iter().map(|x| x.borrowed()).collect();
    same_storage_fast_stack(&mats_csc_views)
}

/// Specify a sparse matrix by constructing it from blocks of other matrices
/// 
/// # Examples
/// ```
/// // a and b are sparse matrices
/// let c = bmat(&[[Some(a), None], [None, Some(b)]]);
pub fn bmat<'a, N, OuterArray, InnerArray>(mats: &OuterArray)
-> Result<CsMatVec<N>, SprsError>
where N: 'a + Copy + Default,
      OuterArray: 'a + AsRef<[InnerArray]>,
      InnerArray: 'a + AsRef<[Option<CsMatView<'a, N>>]> {
    // start by checking if our input is well formed (no column or line of None)
    unimplemented!();
}

#[cfg(test)]
mod test {
    use sparse::csmat::CsMat;
    use sparse::CompressedStorage::{CSR};
    use test_data::{mat1, mat2, mat3, mat4};
    use errors::SprsError::*;

    fn mat1_vstack_mat2() -> CsMat<f64, Vec<usize>, Vec<f64>> {
        let indptr = vec![0, 2, 4, 5, 6, 7, 11, 13, 13, 15, 17];
        let indices = vec![2, 3, 3, 4, 2, 1, 3, 0, 1, 2, 4, 0, 3, 2, 3, 1, 2];
        let data = vec![3., 4., 2., 5., 5., 8., 7., 6., 7., 3., 3.,
                        8., 9., 2., 4., 4., 4.];
        CsMat::from_vecs(CSR, 10, 5, indptr, indices, data).unwrap()
    }

    #[test]
    fn same_storage_fast_stack_failures() {
        let res: Result<CsMat<f64, _, _>, _> =
            super::same_storage_fast_stack(&[]);
        assert_eq!(res, Err(EmptyStackingList));
        let a = mat1();
        let c = mat3();
        let d = mat4();
        let res: Result<CsMat<f64, _, _>, _> =
            super::same_storage_fast_stack(&[]);
        let res = super::same_storage_fast_stack(&[a.borrowed(), c.borrowed()]);
        assert_eq!(res, Err(IncompatibleDimensions));
        let res = super::same_storage_fast_stack(&[a.borrowed(), d.borrowed()]);
        assert_eq!(res, Err(IncompatibleStorages));
    }

    #[test]
    fn same_storage_fast_stack_ok() {
        let a = mat1();
        let b = mat2();
        let res = super::same_storage_fast_stack(&[a.borrowed(), b.borrowed()]);
        let expected = mat1_vstack_mat2();
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn vstack_trivial() {
        let a = mat1();
        let b = mat2();
        let res = super::vstack(&[a.borrowed(), b.borrowed()]);
        let expected = mat1_vstack_mat2();
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn hstack_trivial() {
        let a = mat1().transpose_into();
        let b = mat2().transpose_into();
        let res = super::hstack(&[a.borrowed(), b.borrowed()]);
        let expected = mat1_vstack_mat2().transpose_into();
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn vstack_with_conversion() {
        let a = mat1().to_csc();
        let b = mat2();
        let res = super::vstack(&[a.borrowed(), b.borrowed()]);
        let expected = mat1_vstack_mat2();
        assert_eq!(res, Ok(expected));
    }
}
