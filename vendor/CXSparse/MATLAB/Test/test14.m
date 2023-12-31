function test14
%TEST14 test cs_droptol
%
% Example:
%   test14
% See also: testall

% CXSparse, Copyright (c) 2006-2022, Timothy A. Davis. All Rights Reserved.
% SPDX-License-Identifier: LGPL-2.1+

rand ('state', 0) ;

for trial = 1:100
    m = fix (100 * rand (1)) ;
    n = fix (100 * rand (1)) ;
    d = 0.1*rand (1) ;
    A = sprandn (m,n,d) ;
    [i j x] = find (A) ;
    A = sparse (i,j,2*x-1) ;
    fprintf ('test14 m %3d n %3d nz %d\n', m, n, nnz (A)) ;

    for cmplex = 0:double(~ispc)

        if (cmplex)
            A = A + 1i * sparse (i,j,2*rand(size(x))-1) ;
        end

        % using CSparse
        tol = 0.5 ;
        B = cs_droptol (A, tol) ;

        % using MATLAB
        C = A .* (abs (A) > tol) ;
    %    [m n] = size (A) ;
    %    s = abs (A) > tol ;
    %    [i j] = find (s) ;
    %    x = A (find (s)) ;
    %    A = sparse (i, j, x, m, n) ;

        if (norm (C-B,1) > 0)
            error ('!') ;
        end
    end
end
