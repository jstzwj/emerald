# CMakeList.txt : Top-level CMake project file, do global configuration
# and include sub-projects here.
#
cmake_minimum_required (VERSION 3.8)

project ("emerald")

set(CMAKE_CXX_STANDARD 11)

find_package(Vulkan REQUIRED)


# Include sub-projects.
add_subdirectory ("src")
