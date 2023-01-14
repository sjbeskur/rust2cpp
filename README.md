# Rust2Cpp

This project exists as an example of how to call functions in a C++ library from Rust.   The solution within this repository currently contains two projects, Once CPP project and one Rust project.

The CPP project is located under the cpp directory and contains a simple CMake configuration to build a shared library  called "cppoxide".   

The Rust project is just located within the main directory and calls the C++ shared library through the use of Rust FFI interface.

