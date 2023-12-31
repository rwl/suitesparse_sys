LDL: a sparse LDL' factorization and solve package.
    Written in C, with both a C and MATLAB mexFunction interface. 

LDL, Copyright (c) 2005-2022 by Timothy A. Davis. All Rights Reserved.
SPDX-License-Identifier: LGPL-2.1+

These routines are not terrifically fast (they do not use dense matrix kernels),
but the code is very short and concise.  The purpose is to illustrate the
algorithms in a very concise and readable manner, primarily for educational
purposes.  Although the code is very concise, this package is slightly faster
than the built-in sparse Cholesky factorization in MATLAB 6.5 (chol), when
using the same input permutation.

Requires SuiteSparse_config, in the ../SuiteSparse_config directory relative to
this directory.

Quick start (Unix, or Windows with Cygwin):

    To compile, test, and install LDL, you may wish to first obtain a copy of
    AMD from http://www.suitesparse.com and place it in the
    ../AMD directory, relative to this directory.  Next, type "make", which
    will compile the LDL library and three demo main programs (one of which
    requires AMD).  It will also compile the LDL MATLAB mexFunction (if you
    have MATLAB).  Typing "make clean" will remove non-essential files.
    AMD v3.0 or later is required.  Its use is optional.

Quick start (for MATLAB users);

    To compile, test, and install the LDL mexFunctions (ldlsparse and
    ldlsymbol), start MATLAB in this directory and type ldl_install.
    This works on any system supported by MATLAB.

--------------------------------------------------------------------------------

LDL License: see LDL/Doc/License.txt for the license

Availability:

    http://www.suitesparse.com

Acknowledgements:

    This work was supported by the National Science Foundation, under
    grant CCR-0203270.

    Portions of this work were done while on sabbatical at Stanford University
    and Lawrence Berkeley National Laboratory (with funding from the SciDAC
    program).  I would like to thank Gene Golub, Esmond Ng, and Horst Simon
    for making this sabbatical possible.  I would like to thank Pete Stewart
    for his comments on a draft of this software and paper.

--------------------------------------------------------------------------------
Files and directories in this distribution:
--------------------------------------------------------------------------------

    Documentation, and compiling:

	README.txt	this file
	Makefile	for compiling LDL
	ChangeLog	changes since V1.0 (Dec 31, 2003)
	License.txt	license

	ldl_userguide.pdf   user guide in PDF
	ldl_userguide.ps    user guide in postscript
	ldl_userguide.tex   user guide in Latex
	ldl.bib		    bibliography for user guide

    The LDL library itself:

	ldl.c		the C-callable routines
	ldl.h		include file for any code that calls LDL

    A simple C main program that demonstrates how to use LDL:

	ldlsimple.c	a stand-alone C program, uses the basic features of LDL
	ldlsimple.out	output of ldlsimple

	ldllsimple.c	long integer version of ldlsimple.c

    Demo C program, for testing LDL and providing an example of its use

	ldlmain.c	a stand-alone C main program that uses and tests LDL
	Matrix		a directory containing matrices used by ldlmain.c
	ldlmain.out	output of ldlmain
	ldlamd.out	output of ldlamd (ldlmain.c compiled with AMD)
	ldllamd.out	output of ldllamd (ldlmain.c compiled with AMD, long)

    MATLAB-related, not required for use in a regular C program

	Contents.m	a list of the MATLAB-callable routines
	ldl.m		MATLAB help file for the LDL mexFunction
	ldldemo.m	MATLAB demo of how to use the LDL mexFunction
	ldldemo.out	diary output of ldldemo
	ldltest.m	to test the LDL mexFunction
	ldltest.out	diary output of ldltest
	ldlmex.c	the LDL mexFunction for MATLAB
	ldlrow.m	the numerical algorithm that LDL is based on
	ldlsymbolmex.c	symbolic factorization using LDL (see SYMBFACT, ETREE)
	ldlsymbol.m	help file for the LDLSYMBOL mexFunction

	ldl_install.m	compile, install, and test LDL functions
	ldl_make.m	compile LDL (ldlsparse and ldlsymbol)

	ldlsparse.m	help for ldlsparse

See ldl.c for a description of how to use the code from a C program.  Type
"help ldl" in MATLAB for information on how to use LDL in a MATLAB program.
