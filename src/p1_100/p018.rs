

pub fn problem() {
    let data = 
    "75
    95 64
    17 47 82
    18 35 87 10
    20 04 82 47 65
    19 01 23 75 03 34
    88 02 77 73 07 63 67
    99 65 04 28 06 16 70 92
    41 41 26 56 83 40 80 70 33
    41 48 72 33 47 32 37 16 94 29
    53 71 44 65 25 43 91 52 97 51 14
    70 11 33 28 77 73 17 78 39 68 17 57
    91 71 52 38 17 14 91 43 58 50 27 29 48
    63 66 04 68 89 53 67 30 73 16 69 87 40 31
    04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
    
    // I mean, just do this one on paper.
    // Start with the last row.
    // For each row above it, for each value in that row, add the maximum of the two numbers below it.
    // e.g.:
    // 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
    // 63+62 66+98 04+98 68+27 89+23 53+09 67+70 30+98 73+93 16+93 69+38 87+53 40+60 31+04
    // = 127 164 102 95 112 62 137 128 166 109 107 140 100 35
    // etc.
    // This will take very little effort to finish.
    // In problem 67, we will actually implement something interesting.
}