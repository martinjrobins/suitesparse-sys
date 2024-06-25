<div align="center">
<a href="https://github.com/martinjrobins/diffsol/actions/workflows/rust.yml">
    <img src="https://github.com/martinjrobins/diffsol/actions/workflows/rust.yml/badge.svg" alt="CI build status badge">
</a>
</div>

# suitesparse-sys

This crate provides bindings to the SuiteSparse library. The possible features are:

- `build_vendor`: Build the SuiteSparse library from source (currently version v7.7.0)
- `build_static_libraries`: Build the SuiteSparse library as static libraries

Each library in the SuiteSparse library is a separate feature. The features are:

- `mongoose`: Bindings to the Mongoose library.
- `btf`: Bindings to the BTF library.
- `amd`: Bindings to the Approximate Minimum Degree library (AMD).
- `camd`: Bindings to the Constrained Approximate Minimum Degree library (CAMD).
- `ccolamd`: Bindings to the Constrained Column Approximate Minimum Degree library (CCOLAMD).
- `colamd`: Bindings to the Column Approximate Minimum Degree library (COLAMD).
- `cholmod`: Bindings to the Cholesky Modified library (CHOLMOD).
- `cxsparse`: Bindings to the CXSparse library.
- `ldl`: Bindings to the LDL library.
- `klu`: Bindings to the KLU library.
- `umfpack`: Bindings to the UMFPACK library.
- `paru`: Bindings to the ParU library.
- `rbio`: Bindings to the RBio library.
- `spqr`: Bindings to the SPQR library.
- `spex`: Bindings to the SPEx library.
- `graphblas`: Bindings to the GraphBLAS library.
- `lagraph`: Bindings to the LA-Graph library.

The default feature list builds the SuiteSparse library from source and includes the KLU solver and its dependencies. The default feature list is:

- [`klu`, `amd`, `colamd`, `btf`, `static_libraries`, `build_vendor`]


If you wish to use a pre-installed version of SuiteSparse, you can disable the `build_vendor` feature set the `SUITESPARSE_LIBRARY_DIR` and `SUITESPARSE_INCLUDE_DIR` environment variables to the appropriate paths.
