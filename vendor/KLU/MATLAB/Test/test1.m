function test1 (nmat)
%test1: KLU test
% Example:
%   test1
%
% See also klu

% KLU, Copyright (c) 2004-2022, University of Florida.  All Rights Reserved.
% Authors: Timothy A. Davis and Ekanathan Palamadai.
% SPDX-License-Identifier: LGPL-2.1+

clear functions
% rand ('state', 0) ;

index = ssget ;
f = find (index.nrows == index.ncols & index.isReal) ;
[ignore i] = sort (index.nnz (f)) ;                                         %#ok
f = f (i) ;

h = waitbar (0, 'KLU test 1 of 5') ;

if (nargin < 1)
    nmat = 500 ;
end
nmat = min (nmat, length (f)) ;

% just use the first 100 matrices
nmat = min (nmat, 100) ;

f = f (1:nmat) ;

% f = 274
% f = 101       ; % MATLAB condest is poor

nmat = length (f) ;

conds_klu = ones (1,nmat) ;
conds_matlab = ones (1,nmat) ;

clf

% try

    for k = 1:nmat

        waitbar (k/nmat, h) ;

        i = f (k) ;
%       try
            c = -1 ;
            blocks = 0 ;
            rho = 0 ;
            c2 = 0 ;
            r1 = 0 ;
            r2 = 0 ;
            err = 0 ;

            Prob = ssget (i,index) ;
            A = Prob.A ;
            c = condest (A) ;
            fprintf ('condest %8.2e :', c) ;
            if (c > 1e20)
                fprintf ('skipped\n') ;
                continue
            end
            % klu (A)
            % [L,U,p,q,R,F,r,info] = klu (A) ;

            [LU, info, c2] = klu (A) ;

            L = LU.L ;
            U = LU.U ;
            p = LU.p ;
            q = LU.q ;
            R = LU.R ;
            F = LU.F ;
            r = LU.r ;
            blocks = length (r) - 1 ;

            n = size (A,1) ;
            b = rand (n,1) ;
            x = klu (LU,'\',b) ;
            err = norm (A*x-b,1) / norm (A,1) ;

            % info
            rho = lu_normest (R\A(p,q) - F, L, U) ;
            r1 = info.rcond ;
            r2 = full (min (abs (diag (U))) / max (abs (diag (U)))) ;

            if (r1 ~= r2)
                fprintf ('!\n') ;
                pause
            end

            conds_klu (k) = c2 ;
            conds_matlab (k) = c ;

%       catch me
%           disp (me.message) ;
%       end

        fprintf (...
        'blocks %6d err %8.2e %8.2e rcond %8.2e %8.2e err %8.2e\n', ...
        blocks, rho, c2, r1, r2, err) ;

    end

    k = nmat ;
    plot (1:k, log10 (conds_klu (1:k) ./ conds_matlab (1:k)), 'o') ;
    drawnow

% catch me
%     disp (me.message) ;
% end

close (h) ;
