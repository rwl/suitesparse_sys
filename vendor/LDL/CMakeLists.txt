#-------------------------------------------------------------------------------
# SuiteSparse/LDL/CMakeLists.txt:  cmake for LDL
#-------------------------------------------------------------------------------

# LDL, Copyright (c) 2005-2022 by Timothy A. Davis. All Rights Reserved.
# SPDX-License-Identifier: LGPL-2.1+

#-------------------------------------------------------------------------------
# get the version
#-------------------------------------------------------------------------------

cmake_minimum_required ( VERSION 3.19 )

set ( LDL_DATE "June 16, 2023" )
set ( LDL_VERSION_MAJOR 3 )
set ( LDL_VERSION_MINOR 0 )
set ( LDL_VERSION_SUB   4 )

message ( STATUS "Building LDL version: v"
    ${LDL_VERSION_MAJOR}.
    ${LDL_VERSION_MINOR}.
    ${LDL_VERSION_SUB} " (" ${LDL_DATE} ")" )

#-------------------------------------------------------------------------------
# SuiteSparse policies
#-------------------------------------------------------------------------------

set ( CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH}
    ${CMAKE_SOURCE_DIR}/cmake_modules
    ${CMAKE_SOURCE_DIR}/../AMD/cmake_modules
    ${CMAKE_SOURCE_DIR}/../SuiteSparse_config/cmake_modules )

include ( SuiteSparsePolicy )

#-------------------------------------------------------------------------------
# define the project
#-------------------------------------------------------------------------------

project ( ldl
    VERSION "${LDL_VERSION_MAJOR}.${LDL_VERSION_MINOR}.${LDL_VERSION_SUB}"
    LANGUAGES C )

#-------------------------------------------------------------------------------
# find library dependencies
#-------------------------------------------------------------------------------

find_package ( SuiteSparse_config 7.1.0 REQUIRED )
find_package ( AMD 3.0.4 REQUIRED )

#-------------------------------------------------------------------------------
# Configure ldl.h with version number
#-------------------------------------------------------------------------------

configure_file ( "Config/ldl.h.in"
    "${PROJECT_SOURCE_DIR}/Include/ldl.h"
    NEWLINE_STYLE LF )
configure_file ( "Config/ldl_version.tex.in"
    "${PROJECT_SOURCE_DIR}/Doc/ldl_version.tex"
    NEWLINE_STYLE LF )

#-------------------------------------------------------------------------------
# include directories
#-------------------------------------------------------------------------------

include_directories ( Source Include ${SUITESPARSE_CONFIG_INCLUDE_DIR}
    ${AMD_INCLUDE_DIR} )

#-------------------------------------------------------------------------------
# dynamic ldl library properties
#-------------------------------------------------------------------------------

file ( GLOB LDL_SOURCES "Source/*.c" )

add_library ( ldl SHARED ${LDL_SOURCES} )

set_target_properties ( ldl PROPERTIES
    VERSION ${LDL_VERSION_MAJOR}.${LDL_VERSION_MINOR}.${LDL_VERSION_SUB}
    C_STANDARD 11
    C_STANDARD_REQUIRED ON
    SOVERSION ${LDL_VERSION_MAJOR}
    PUBLIC_HEADER "Include/ldl.h"
    WINDOWS_EXPORT_ALL_SYMBOLS ON )

#-------------------------------------------------------------------------------
# static ldl library properties
#-------------------------------------------------------------------------------

if ( NOT NSTATIC )
    add_library ( ldl_static STATIC ${LDL_SOURCES} )

    set_target_properties ( ldl_static PROPERTIES
        VERSION ${LDL_VERSION_MAJOR}.${LDL_VERSION_MINOR}.${LDL_VERSION_SUB}
        C_STANDARD 11
        C_STANDARD_REQUIRED ON
        OUTPUT_NAME ldl
        SOVERSION ${LDL_VERSION_MAJOR} )

    if ( MSVC )
        set_target_properties ( ldl_static PROPERTIES
            OUTPUT_NAME ldl_static )
    endif ( )
endif ( )

#-------------------------------------------------------------------------------
# add the library dependencies
#-------------------------------------------------------------------------------

# libm:
if ( NOT WIN32 )
    target_link_libraries ( ldl PRIVATE m )
    if ( NOT NSTATIC )
        target_link_libraries ( ldl_static PUBLIC m )
    endif ( )
endif ( )

target_link_libraries ( ldl PRIVATE
    ${SUITESPARSE_CONFIG_LIBRARIES} ${AMD_LIBRARIES} )

if ( NOT NSTATIC )
    target_link_libraries ( ldl_static PUBLIC
    ${SUITESPARSE_CONFIG_STATIC} ${AMD_STATIC} )
endif ( )

#-------------------------------------------------------------------------------
# LDL installation location
#-------------------------------------------------------------------------------

install ( TARGETS ldl
    LIBRARY DESTINATION ${SUITESPARSE_LIBDIR}
    ARCHIVE DESTINATION ${SUITESPARSE_LIBDIR}
    RUNTIME DESTINATION ${SUITESPARSE_BINDIR}
    PUBLIC_HEADER DESTINATION ${SUITESPARSE_INCLUDEDIR} )
install ( FILES ${CMAKE_SOURCE_DIR}/cmake_modules/FindLDL.cmake
    DESTINATION ${SUITESPARSE_LIBDIR}/cmake/SuiteSparse 
    COMPONENT Development )
if ( NOT NSTATIC )
    install ( TARGETS ldl_static
        ARCHIVE DESTINATION ${SUITESPARSE_LIBDIR} )
endif ( )

#-------------------------------------------------------------------------------
# Demo library and programs
#-------------------------------------------------------------------------------

option ( DEMO "ON: Build the demo programs.  OFF (default): do not build the demo programs." off )
if ( DEMO )

    #---------------------------------------------------------------------------
    # demo library
    #---------------------------------------------------------------------------

    message ( STATUS "Also compiling the demos in LDL/Demo" )

    #---------------------------------------------------------------------------
    # Demo programs
    #---------------------------------------------------------------------------

    add_executable ( ldlsimple      "Demo/ldlsimple.c" )
    add_executable ( ldllsimple     "Demo/ldllsimple.c" )
    add_executable ( ldlmain        "Demo/ldlmain.c" )
    add_executable ( ldllmain       "Demo/ldllmain.c" )

    # Libraries required for Demo programs
    target_link_libraries ( ldlsimple   PUBLIC ldl ${SUITESPARSE_CONFIG_LIBRARIES} )
    target_link_libraries ( ldllsimple  PUBLIC ldl )
    target_link_libraries ( ldlmain     PUBLIC ldl )
    target_link_libraries ( ldllmain    PUBLIC ldl )

    if ( AMD_FOUND )
        message ( STATUS "AMD found for ldlamd and ldllamd" )
        add_executable ( ldlamd     "Demo/ldlamd.c" )
        add_executable ( ldllamd    "Demo/ldllamd.c" )
        target_link_libraries ( ldlamd  PUBLIC ldl ${AMD_LIBRARIES} )
        target_link_libraries ( ldllamd PUBLIC ldl ${AMD_LIBRARIES} )
    endif ( )

else ( )

    message ( STATUS "Skipping the demos in LDL/Demo" )

endif ( )

#-------------------------------------------------------------------------------
# report status
#-------------------------------------------------------------------------------

include ( SuiteSparseReport )

