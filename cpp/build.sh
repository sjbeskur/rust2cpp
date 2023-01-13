#!/bin/bash
## Build the C++ "wrapper"

cmake -B target -S .
cd target
make 
echo "------------------------------"
echo "Make complete!"
echo "Now from the target dir run:  "
echo "    'sudo make install'       "
echo "to complete the installation" 
echo "------------------------------"
# sudo make install