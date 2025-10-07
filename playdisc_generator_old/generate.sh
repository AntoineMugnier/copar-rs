#!/bin/bash

python3  ./main.py ./input/$1
cp ./output/playdisc.h ../components/rtl8852bs_driver/procedures/init/inc
cp ./output/playdisc.c ../components/rtl8852bs_driver/procedures/init/src

