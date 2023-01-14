# Rust2Cpp

This project exists as an example of how to call functions in a C++ library from Rust.   The solution within this repository currently contains two projects, Once CPP project and one Rust project.

The CPP project is located under the cpp directory and contains a simple CMake configuration to build a shared library  called "cppoxide".

The Rust project is just located within the main directory and calls the C++ shared library through the use of Rust FFI interface.

## CPP Instructions

Build the Cpp project by running the following command from the cpp directory.

```bash
cmake -B target -S . && make
cd target 
sudo make install
```

This will make and compile the Cpp project and install the shared library into your default lib directory.

## Very IMPORTANT NOTE: 
In order for rust to find you shared libraries you must make sure that you have set the proper environment variable:  

```bash
export LD_LIBRARY_PATH=/usr/local/lib
```

## Rust Instructions

The Rust program takes and image file as input, reads the image bytes, and passes this array to the cppoxide library for analysis.

Build and run the Rust program by running the following commands:

```bash
cargo build
cargo run </path/to/some/image>
```
