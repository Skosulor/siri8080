
# Intel 8080 tests

cpudiag is an test rom source from [superzazu](https://github.com/superzazu/8080) and comes originally from Microcosm Associates.

The original rom assumes a start address (PC location) of 0x100 instead of 0x0 causing all branches to be off. A (lazy) fix of the rom is included in this directory where 256 0's is simply prepended. 

