const INVALID_VALUE: u8 = 255;
const SIZE: i64 = 64;
pub(crate) fn encode(mut num: i64) -> String {
    let mut res = Vec::<char>::new();
    loop {
        res.push(ENCODE_MAP[(num % SIZE) as usize]);
        num /= SIZE;
        if num <= 0 {
            break;
        }
    }
    res.into_iter().rev().collect()
}
pub(crate) fn decode(input: &str) -> Result<i64, ()> {
    let mut res = 0;
    for val in input.chars() {
        let temp = *DECODE_MAP.get(val as usize).ok_or(())?;
        if temp == INVALID_VALUE {
            return Err(());
        }
        res = res * SIZE + (temp as i64);
    }
    Ok(res)
}

// "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";
const ENCODE_MAP: &[char; 64] = &[
    '0', // input 0 (0x0) => '0' (0x30)
    '1', // input 1 (0x1) => '1' (0x31)
    '2', // input 2 (0x2) => '2' (0x32)
    '3', // input 3 (0x3) => '3' (0x33)
    '4', // input 4 (0x4) => '4' (0x34)
    '5', // input 5 (0x5) => '5' (0x35)
    '6', // input 6 (0x6) => '6' (0x36)
    '7', // input 7 (0x7) => '7' (0x37)
    '8', // input 8 (0x8) => '8' (0x38)
    '9', // input 9 (0x9) => '9' (0x39)
    'A', // input 10 (0xa) => 'A' (0x41)
    'B', // input 11 (0xb) => 'B' (0x42)
    'C', // input 12 (0xc) => 'C' (0x43)
    'D', // input 13 (0xd) => 'D' (0x44)
    'E', // input 14 (0xe) => 'E' (0x45)
    'F', // input 15 (0xf) => 'F' (0x46)
    'G', // input 16 (0x10) => 'G' (0x47)
    'H', // input 17 (0x11) => 'H' (0x48)
    'I', // input 18 (0x12) => 'I' (0x49)
    'J', // input 19 (0x13) => 'J' (0x4a)
    'K', // input 20 (0x14) => 'K' (0x4b)
    'L', // input 21 (0x15) => 'L' (0x4c)
    'M', // input 22 (0x16) => 'M' (0x4d)
    'N', // input 23 (0x17) => 'N' (0x4e)
    'O', // input 24 (0x18) => 'O' (0x4f)
    'P', // input 25 (0x19) => 'P' (0x50)
    'Q', // input 26 (0x1a) => 'Q' (0x51)
    'R', // input 27 (0x1b) => 'R' (0x52)
    'S', // input 28 (0x1c) => 'S' (0x53)
    'T', // input 29 (0x1d) => 'T' (0x54)
    'U', // input 30 (0x1e) => 'U' (0x55)
    'V', // input 31 (0x1f) => 'V' (0x56)
    'W', // input 32 (0x20) => 'W' (0x57)
    'X', // input 33 (0x21) => 'X' (0x58)
    'Y', // input 34 (0x22) => 'Y' (0x59)
    'Z', // input 35 (0x23) => 'Z' (0x5a)
    'a', // input 36 (0x24) => 'a' (0x61)
    'b', // input 37 (0x25) => 'b' (0x62)
    'c', // input 38 (0x26) => 'c' (0x63)
    'd', // input 39 (0x27) => 'd' (0x64)
    'e', // input 40 (0x28) => 'e' (0x65)
    'f', // input 41 (0x29) => 'f' (0x66)
    'g', // input 42 (0x2a) => 'g' (0x67)
    'h', // input 43 (0x2b) => 'h' (0x68)
    'i', // input 44 (0x2c) => 'i' (0x69)
    'j', // input 45 (0x2d) => 'j' (0x6a)
    'k', // input 46 (0x2e) => 'k' (0x6b)
    'l', // input 47 (0x2f) => 'l' (0x6c)
    'm', // input 48 (0x30) => 'm' (0x6d)
    'n', // input 49 (0x31) => 'n' (0x6e)
    'o', // input 50 (0x32) => 'o' (0x6f)
    'p', // input 51 (0x33) => 'p' (0x70)
    'q', // input 52 (0x34) => 'q' (0x71)
    'r', // input 53 (0x35) => 'r' (0x72)
    's', // input 54 (0x36) => 's' (0x73)
    't', // input 55 (0x37) => 't' (0x74)
    'u', // input 56 (0x38) => 'u' (0x75)
    'v', // input 57 (0x39) => 'v' (0x76)
    'w', // input 58 (0x3a) => 'w' (0x77)
    'x', // input 59 (0x3b) => 'x' (0x78)
    'y', // input 60 (0x3c) => 'y' (0x79)
    'z', // input 61 (0x3d) => 'z' (0x7a)
    '-', // input 62 (0x3e) => '-' (0x2d)
    '_', // input 63 (0x3f) => '_' (0x5f)
];
const DECODE_MAP: &[u8; 256] = &[
    INVALID_VALUE, // input 0 (0x0)
    INVALID_VALUE, // input 1 (0x1)
    INVALID_VALUE, // input 2 (0x2)
    INVALID_VALUE, // input 3 (0x3)
    INVALID_VALUE, // input 4 (0x4)
    INVALID_VALUE, // input 5 (0x5)
    INVALID_VALUE, // input 6 (0x6)
    INVALID_VALUE, // input 7 (0x7)
    INVALID_VALUE, // input 8 (0x8)
    INVALID_VALUE, // input 9 (0x9)
    INVALID_VALUE, // input 10 (0xa)
    INVALID_VALUE, // input 11 (0xb)
    INVALID_VALUE, // input 12 (0xc)
    INVALID_VALUE, // input 13 (0xd)
    INVALID_VALUE, // input 14 (0xe)
    INVALID_VALUE, // input 15 (0xf)
    INVALID_VALUE, // input 16 (0x10)
    INVALID_VALUE, // input 17 (0x11)
    INVALID_VALUE, // input 18 (0x12)
    INVALID_VALUE, // input 19 (0x13)
    INVALID_VALUE, // input 20 (0x14)
    INVALID_VALUE, // input 21 (0x15)
    INVALID_VALUE, // input 22 (0x16)
    INVALID_VALUE, // input 23 (0x17)
    INVALID_VALUE, // input 24 (0x18)
    INVALID_VALUE, // input 25 (0x19)
    INVALID_VALUE, // input 26 (0x1a)
    INVALID_VALUE, // input 27 (0x1b)
    INVALID_VALUE, // input 28 (0x1c)
    INVALID_VALUE, // input 29 (0x1d)
    INVALID_VALUE, // input 30 (0x1e)
    INVALID_VALUE, // input 31 (0x1f)
    INVALID_VALUE, // input 32 (0x20)
    INVALID_VALUE, // input 33 (0x21)
    INVALID_VALUE, // input 34 (0x22)
    INVALID_VALUE, // input 35 (0x23)
    INVALID_VALUE, // input 36 (0x24)
    INVALID_VALUE, // input 37 (0x25)
    INVALID_VALUE, // input 38 (0x26)
    INVALID_VALUE, // input 39 (0x27)
    INVALID_VALUE, // input 40 (0x28)
    INVALID_VALUE, // input 41 (0x29)
    INVALID_VALUE, // input 42 (0x2a)
    INVALID_VALUE, // input 43 (0x2b)
    INVALID_VALUE, // input 44 (0x2c)
    62,            // input 45 (0x2d char '-') => 62 (0x3e)
    INVALID_VALUE, // input 46 (0x2e)
    INVALID_VALUE, // input 47 (0x2f)
    0,             // input 48 (0x30 char '0') => 0 (0x0)
    1,             // input 49 (0x31 char '1') => 1 (0x1)
    2,             // input 50 (0x32 char '2') => 2 (0x2)
    3,             // input 51 (0x33 char '3') => 3 (0x3)
    4,             // input 52 (0x34 char '4') => 4 (0x4)
    5,             // input 53 (0x35 char '5') => 5 (0x5)
    6,             // input 54 (0x36 char '6') => 6 (0x6)
    7,             // input 55 (0x37 char '7') => 7 (0x7)
    8,             // input 56 (0x38 char '8') => 8 (0x8)
    9,             // input 57 (0x39 char '9') => 9 (0x9)
    INVALID_VALUE, // input 58 (0x3a)
    INVALID_VALUE, // input 59 (0x3b)
    INVALID_VALUE, // input 60 (0x3c)
    INVALID_VALUE, // input 61 (0x3d)
    INVALID_VALUE, // input 62 (0x3e)
    INVALID_VALUE, // input 63 (0x3f)
    INVALID_VALUE, // input 64 (0x40)
    10,            // input 65 (0x41 char 'A') => 10 (0xa)
    11,            // input 66 (0x42 char 'B') => 11 (0xb)
    12,            // input 67 (0x43 char 'C') => 12 (0xc)
    13,            // input 68 (0x44 char 'D') => 13 (0xd)
    14,            // input 69 (0x45 char 'E') => 14 (0xe)
    15,            // input 70 (0x46 char 'F') => 15 (0xf)
    16,            // input 71 (0x47 char 'G') => 16 (0x10)
    17,            // input 72 (0x48 char 'H') => 17 (0x11)
    18,            // input 73 (0x49 char 'I') => 18 (0x12)
    19,            // input 74 (0x4a char 'J') => 19 (0x13)
    20,            // input 75 (0x4b char 'K') => 20 (0x14)
    21,            // input 76 (0x4c char 'L') => 21 (0x15)
    22,            // input 77 (0x4d char 'M') => 22 (0x16)
    23,            // input 78 (0x4e char 'N') => 23 (0x17)
    24,            // input 79 (0x4f char 'O') => 24 (0x18)
    25,            // input 80 (0x50 char 'P') => 25 (0x19)
    26,            // input 81 (0x51 char 'Q') => 26 (0x1a)
    27,            // input 82 (0x52 char 'R') => 27 (0x1b)
    28,            // input 83 (0x53 char 'S') => 28 (0x1c)
    29,            // input 84 (0x54 char 'T') => 29 (0x1d)
    30,            // input 85 (0x55 char 'U') => 30 (0x1e)
    31,            // input 86 (0x56 char 'V') => 31 (0x1f)
    32,            // input 87 (0x57 char 'W') => 32 (0x20)
    33,            // input 88 (0x58 char 'X') => 33 (0x21)
    34,            // input 89 (0x59 char 'Y') => 34 (0x22)
    35,            // input 90 (0x5a char 'Z') => 35 (0x23)
    INVALID_VALUE, // input 91 (0x5b)
    INVALID_VALUE, // input 92 (0x5c)
    INVALID_VALUE, // input 93 (0x5d)
    INVALID_VALUE, // input 94 (0x5e)
    63,            // input 95 (0x5f char '_') => 63 (0x3f)
    INVALID_VALUE, // input 96 (0x60)
    36,            // input 97 (0x61 char 'a') => 36 (0x24)
    37,            // input 98 (0x62 char 'b') => 37 (0x25)
    38,            // input 99 (0x63 char 'c') => 38 (0x26)
    39,            // input 100 (0x64 char 'd') => 39 (0x27)
    40,            // input 101 (0x65 char 'e') => 40 (0x28)
    41,            // input 102 (0x66 char 'f') => 41 (0x29)
    42,            // input 103 (0x67 char 'g') => 42 (0x2a)
    43,            // input 104 (0x68 char 'h') => 43 (0x2b)
    44,            // input 105 (0x69 char 'i') => 44 (0x2c)
    45,            // input 106 (0x6a char 'j') => 45 (0x2d)
    46,            // input 107 (0x6b char 'k') => 46 (0x2e)
    47,            // input 108 (0x6c char 'l') => 47 (0x2f)
    48,            // input 109 (0x6d char 'm') => 48 (0x30)
    49,            // input 110 (0x6e char 'n') => 49 (0x31)
    50,            // input 111 (0x6f char 'o') => 50 (0x32)
    51,            // input 112 (0x70 char 'p') => 51 (0x33)
    52,            // input 113 (0x71 char 'q') => 52 (0x34)
    53,            // input 114 (0x72 char 'r') => 53 (0x35)
    54,            // input 115 (0x73 char 's') => 54 (0x36)
    55,            // input 116 (0x74 char 't') => 55 (0x37)
    56,            // input 117 (0x75 char 'u') => 56 (0x38)
    57,            // input 118 (0x76 char 'v') => 57 (0x39)
    58,            // input 119 (0x77 char 'w') => 58 (0x3a)
    59,            // input 120 (0x78 char 'x') => 59 (0x3b)
    60,            // input 121 (0x79 char 'y') => 60 (0x3c)
    61,            // input 122 (0x7a char 'z') => 61 (0x3d)
    INVALID_VALUE, // input 123 (0x7b)
    INVALID_VALUE, // input 124 (0x7c)
    INVALID_VALUE, // input 125 (0x7d)
    INVALID_VALUE, // input 126 (0x7e)
    INVALID_VALUE, // input 127 (0x7f)
    INVALID_VALUE, // input 128 (0x80)
    INVALID_VALUE, // input 129 (0x81)
    INVALID_VALUE, // input 130 (0x82)
    INVALID_VALUE, // input 131 (0x83)
    INVALID_VALUE, // input 132 (0x84)
    INVALID_VALUE, // input 133 (0x85)
    INVALID_VALUE, // input 134 (0x86)
    INVALID_VALUE, // input 135 (0x87)
    INVALID_VALUE, // input 136 (0x88)
    INVALID_VALUE, // input 137 (0x89)
    INVALID_VALUE, // input 138 (0x8a)
    INVALID_VALUE, // input 139 (0x8b)
    INVALID_VALUE, // input 140 (0x8c)
    INVALID_VALUE, // input 141 (0x8d)
    INVALID_VALUE, // input 142 (0x8e)
    INVALID_VALUE, // input 143 (0x8f)
    INVALID_VALUE, // input 144 (0x90)
    INVALID_VALUE, // input 145 (0x91)
    INVALID_VALUE, // input 146 (0x92)
    INVALID_VALUE, // input 147 (0x93)
    INVALID_VALUE, // input 148 (0x94)
    INVALID_VALUE, // input 149 (0x95)
    INVALID_VALUE, // input 150 (0x96)
    INVALID_VALUE, // input 151 (0x97)
    INVALID_VALUE, // input 152 (0x98)
    INVALID_VALUE, // input 153 (0x99)
    INVALID_VALUE, // input 154 (0x9a)
    INVALID_VALUE, // input 155 (0x9b)
    INVALID_VALUE, // input 156 (0x9c)
    INVALID_VALUE, // input 157 (0x9d)
    INVALID_VALUE, // input 158 (0x9e)
    INVALID_VALUE, // input 159 (0x9f)
    INVALID_VALUE, // input 160 (0xa0)
    INVALID_VALUE, // input 161 (0xa1)
    INVALID_VALUE, // input 162 (0xa2)
    INVALID_VALUE, // input 163 (0xa3)
    INVALID_VALUE, // input 164 (0xa4)
    INVALID_VALUE, // input 165 (0xa5)
    INVALID_VALUE, // input 166 (0xa6)
    INVALID_VALUE, // input 167 (0xa7)
    INVALID_VALUE, // input 168 (0xa8)
    INVALID_VALUE, // input 169 (0xa9)
    INVALID_VALUE, // input 170 (0xaa)
    INVALID_VALUE, // input 171 (0xab)
    INVALID_VALUE, // input 172 (0xac)
    INVALID_VALUE, // input 173 (0xad)
    INVALID_VALUE, // input 174 (0xae)
    INVALID_VALUE, // input 175 (0xaf)
    INVALID_VALUE, // input 176 (0xb0)
    INVALID_VALUE, // input 177 (0xb1)
    INVALID_VALUE, // input 178 (0xb2)
    INVALID_VALUE, // input 179 (0xb3)
    INVALID_VALUE, // input 180 (0xb4)
    INVALID_VALUE, // input 181 (0xb5)
    INVALID_VALUE, // input 182 (0xb6)
    INVALID_VALUE, // input 183 (0xb7)
    INVALID_VALUE, // input 184 (0xb8)
    INVALID_VALUE, // input 185 (0xb9)
    INVALID_VALUE, // input 186 (0xba)
    INVALID_VALUE, // input 187 (0xbb)
    INVALID_VALUE, // input 188 (0xbc)
    INVALID_VALUE, // input 189 (0xbd)
    INVALID_VALUE, // input 190 (0xbe)
    INVALID_VALUE, // input 191 (0xbf)
    INVALID_VALUE, // input 192 (0xc0)
    INVALID_VALUE, // input 193 (0xc1)
    INVALID_VALUE, // input 194 (0xc2)
    INVALID_VALUE, // input 195 (0xc3)
    INVALID_VALUE, // input 196 (0xc4)
    INVALID_VALUE, // input 197 (0xc5)
    INVALID_VALUE, // input 198 (0xc6)
    INVALID_VALUE, // input 199 (0xc7)
    INVALID_VALUE, // input 200 (0xc8)
    INVALID_VALUE, // input 201 (0xc9)
    INVALID_VALUE, // input 202 (0xca)
    INVALID_VALUE, // input 203 (0xcb)
    INVALID_VALUE, // input 204 (0xcc)
    INVALID_VALUE, // input 205 (0xcd)
    INVALID_VALUE, // input 206 (0xce)
    INVALID_VALUE, // input 207 (0xcf)
    INVALID_VALUE, // input 208 (0xd0)
    INVALID_VALUE, // input 209 (0xd1)
    INVALID_VALUE, // input 210 (0xd2)
    INVALID_VALUE, // input 211 (0xd3)
    INVALID_VALUE, // input 212 (0xd4)
    INVALID_VALUE, // input 213 (0xd5)
    INVALID_VALUE, // input 214 (0xd6)
    INVALID_VALUE, // input 215 (0xd7)
    INVALID_VALUE, // input 216 (0xd8)
    INVALID_VALUE, // input 217 (0xd9)
    INVALID_VALUE, // input 218 (0xda)
    INVALID_VALUE, // input 219 (0xdb)
    INVALID_VALUE, // input 220 (0xdc)
    INVALID_VALUE, // input 221 (0xdd)
    INVALID_VALUE, // input 222 (0xde)
    INVALID_VALUE, // input 223 (0xdf)
    INVALID_VALUE, // input 224 (0xe0)
    INVALID_VALUE, // input 225 (0xe1)
    INVALID_VALUE, // input 226 (0xe2)
    INVALID_VALUE, // input 227 (0xe3)
    INVALID_VALUE, // input 228 (0xe4)
    INVALID_VALUE, // input 229 (0xe5)
    INVALID_VALUE, // input 230 (0xe6)
    INVALID_VALUE, // input 231 (0xe7)
    INVALID_VALUE, // input 232 (0xe8)
    INVALID_VALUE, // input 233 (0xe9)
    INVALID_VALUE, // input 234 (0xea)
    INVALID_VALUE, // input 235 (0xeb)
    INVALID_VALUE, // input 236 (0xec)
    INVALID_VALUE, // input 237 (0xed)
    INVALID_VALUE, // input 238 (0xee)
    INVALID_VALUE, // input 239 (0xef)
    INVALID_VALUE, // input 240 (0xf0)
    INVALID_VALUE, // input 241 (0xf1)
    INVALID_VALUE, // input 242 (0xf2)
    INVALID_VALUE, // input 243 (0xf3)
    INVALID_VALUE, // input 244 (0xf4)
    INVALID_VALUE, // input 245 (0xf5)
    INVALID_VALUE, // input 246 (0xf6)
    INVALID_VALUE, // input 247 (0xf7)
    INVALID_VALUE, // input 248 (0xf8)
    INVALID_VALUE, // input 249 (0xf9)
    INVALID_VALUE, // input 250 (0xfa)
    INVALID_VALUE, // input 251 (0xfb)
    INVALID_VALUE, // input 252 (0xfc)
    INVALID_VALUE, // input 253 (0xfd)
    INVALID_VALUE, // input 254 (0xfe)
    INVALID_VALUE, // input 255 (0xff)
];