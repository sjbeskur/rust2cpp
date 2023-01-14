#ifndef RUSTCPP_H
#define RUSTCPP_H

#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

typedef uint8_t byte;
extern "C" int set_image_buffer(byte *buffer, int size, u_int32_t width, u_int32_t height);

extern "C" float proof_of_life(int one, int two);

// fails
// extern "C" std::vector<float> add_to_vector(float x);
// extern "C" const char *get_string(int one, int two);

#endif