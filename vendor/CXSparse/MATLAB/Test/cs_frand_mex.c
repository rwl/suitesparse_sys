// CXSparse/MATLAB/Test/cs_frand_mex: random sparse finite-element matrix
// CXSparse, Copyright (c) 2006-2022, Timothy A. Davis. All Rights Reserved.
// SPDX-License-Identifier: LGPL-2.1+
#include "cs_mex.h"
/* A = cs_frand (n,nel,s) creates an n-by-n sparse matrix consisting of nel
 * finite elements, each of which are of size s-by-s with random symmetric
 * nonzero pattern, plus the identity matrix.
 * See also MATLAB/Demo/private/frand.m */

cs_dl *cs_dl_frand (int64_t n, int64_t nel, int64_t s)
{
    int64_t ss = s*s, nz = nel*ss, e, i, j, *P ;
    cs_dl *A, *T = cs_dl_spalloc (n, n, nz, 1, 1) ;
    if (!T) return (NULL) ;
    P = cs_dl_malloc (s, sizeof (int64_t)) ;
    if (!P) return (cs_dl_spfree (T)) ;
    for (e = 0 ; e < nel ; e++)
    {
        for (i = 0 ; i < s ; i++) P [i] = rand () % n ;
        for (j = 0 ; j < s ; j++)
        {
            for (i = 0 ; i < s ; i++)
            {
                cs_dl_entry (T, P [i], P [j], rand () / (double) RAND_MAX) ;
            }
        }
    }
    for (i = 0 ; i < n ; i++) cs_dl_entry (T, i, i, 1) ;
    A = cs_dl_compress (T) ;
    cs_dl_spfree (T) ;
    return (cs_dl_dupl (A) ? A : cs_dl_spfree (A)) ;
}

void mexFunction
(
    int nargout,
    mxArray *pargout [ ],
    int nargin,
    const mxArray *pargin [ ]
)
{
    int64_t n, nel, s ;
    cs_dl *A, *AT ;
    if (nargout > 1 || nargin != 3)
    {
        mexErrMsgTxt ("Usage: C = cs_frand(n,nel,s)") ;
    }
    n = mxGetScalar (pargin [0]) ;
    nel = mxGetScalar (pargin [1]) ;
    s = mxGetScalar (pargin [2]) ;

    n = CS_MAX (1,n) ;
    nel = CS_MAX (1,nel) ;
    s = CS_MAX (1,s) ;

    AT = cs_dl_frand (n, nel, s) ;
    A = cs_dl_transpose (AT, 1) ;
    cs_dl_spfree (AT) ;
    cs_dl_dropzeros (A) ;

    pargout [0] = cs_dl_mex_put_sparse (&A) ;
}
