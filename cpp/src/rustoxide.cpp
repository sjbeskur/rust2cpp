#include "rustoxide.hpp"
#include <iostream>
#include <cstring>

using namespace std;

extern "C" int set_image_buffer(byte *buffer, int size)
{

    byte arr[size];
    memcpy(arr, buffer, size); // better not to copy if we don't have to

    cout << size << endl;
    cout << sizeof(arr) << endl;

    return sizeof(arr) == size;
}

extern "C" float proof_of_life(int one, int two)
{
    return (one + 2) / (two + 2) * 1000;
}
