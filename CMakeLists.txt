cmake_minimum_required(VERSION 3.10)
project(osupp)
add_compile_definitions(OSUPP_VERSION_MAJOR=0)
add_compile_definitions(OSUPP_VERSION_MINOR=1)
add_compile_definitions(OSUPP_VERSION_REVISION=0)

if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
    add_compile_options (-fdiagnostics-color=always)
elseif ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
    add_compile_options (-fcolor-diagnostics -gdwarf-4)
endif ()
add_compile_options(-pthread)


find_package(SFML COMPONENTS system window graphics)
include(thirdparty/cmrc/CMakeRC.cmake)

cmrc_add_resource_library(resources res/background.jpg res/hitcircle.png)

set(LIBS SFML) # define our required libraries

file(GLOB_RECURSE SOURCES "src/*.cpp") # include all source files
add_executable(${PROJECT_NAME} ${CMAKE_CURRENT_LIST_DIR}/src/main.cpp ${SOURCES}) # compile
target_link_libraries(${PROJECT_NAME} sfml-graphics sfml-window sfml-system resources) # link

file(COPY res DESTINATION .) # copy our resources to build folder

# Debian build
if (UNIX)
set(CPACK_DEBIAN_PACKAGE_NAME "osupp")
set(CPACK_DEBIAN_PACKAGE_DESCRIPTION "Unofficial rewrited osu! client built on C++ and SFML")
set(CPACK_DEBIAN_PACKAGE_DEPENDS "libsfml-dev")
set(CPACK_DEBIAN_FILE_NAME ${CPACK_PACKAGE_NAME})
set(CPACK_PACKAGE_CONTACT "radekw8733@gmail.com")
set(CPACK_DEBIAN_PACKAGE_MAINTAINER "Radoslaw Wolanski")
set(CPACK_PACKAGE_VERSION_MAJOR OSUPP_VERSION_MAJOR)
set(CPACK_PACKAGE_VERSION_MINOR OSUPP_VERSION_MINOR)
set(CPACK_PACKAGE_VERSION_PATCH OSUPP_VERSION_REVISION)
set(CPACK_PACKAGE_VERSION "${CPACK_PACKAGE_VERSION_MAJOR}.${CPACK_PACKAGE_VERSION_MINOR}.${CPACK_PACKAGE_VERSION_PATCH}")
install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION "bin" COMPONENT applications)
install(DIRECTORY ${CMAKE_SOURCE_DIR}/res DESTINATION "share/osupp")
install(FILES ${CMAKE_SOURCE_DIR}/osupp.desktop DESTINATION share/applications)
endif (UNIX)

# PACKAGING
include(CPack)