# About

This is a library that accelerates the G-ORE algorithm through GPU



# Dependency

cmake >= 3.17

C++ >= 11

cuda toolkit >= 11.2


# Test
Encrypt character types with a length of 8-Byte, and compare the performance of G-ORE's parallel execution scheme with the original scheme at different data magnitudes.

| data size | Parallel G-ORE time cost (ms) | G-ORE time cost (ms) |
|-----------|-------------------------------|----------------------|
| 100000    | 1065.796                      | 2753.792             |
| 500000    | 1075.695                      | 13699.662            |
| 1000000   | 1116.549                      | 27728.569            |
| 5000000   | 1286.060                      | 138597.827           |
| 10000000  | 1546.802                      | 274757.115           |
| 50000000  | 2945.165                      | 1378606.5610         |
| 100000000 | 4802.006                      | > 0.5h               |