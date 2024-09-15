#!/bin/python
import sys

for (ij, v) in enumerate(sys.argv[2:]):
    i = ij % 4
    j = ij // 4
    print(f'SetOn {int(v) - 1} {sys.argv[1]} {j} {i}')

