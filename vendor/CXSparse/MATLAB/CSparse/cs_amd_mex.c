// CXSparse/MATLAB/CSparse/cs_amd_mex: approximate minimum degree ordering
// CXSparse, Copyright (c) 2006-2022, Timothy A. Davis. All Rights Reserved.
// SPDX-License-Identifier: LGPL-2.1+
#include "cs_mex.h"
/* cs_amd: approximate minimum degree ordering */
void mexFunction
(
    int nargout,
    mxArray *pargout [ ],
    int nargin,
    const mxArray *pargin [ ]
)
{
    cs_dl Amatrix, *A ;
    int64_t *P, order ;
    if (nargout > 1 || nargin < 1 || nargin > 2)
    {
        mexErrMsgTxt ("Usage: p = cs_amd(A,order)") ;
    }
    A = cs_dl_mex_get_sparse (&Amatrix, 0, 0, pargin [0]) ;    /* get A */
    order = (nargin > 1) ? mxGetScalar (pargin [1]) : 1 ;   /* get ordering */
    order = CS_MAX (order, 1) ;
    order = CS_MIN (order, 3) ;
    P = cs_dl_amd (order, A) ;                      /* min. degree ordering */
    pargout [0] = cs_dl_mex_put_int (P, A->n, 1, 1) ;  /* return P */
}
