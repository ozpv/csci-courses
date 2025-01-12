#!/bin/sh
ls -lah code
g++ -o main ./code/main.cpp
chmod +x ./main
./main
