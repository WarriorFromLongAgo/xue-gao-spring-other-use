extern crate hex;
use std::convert::TryInto;

// MD5 的 S 常量，每个轮次使用的左旋位数
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
];

// MD5 的 K 常量，每个轮次使用的加法常数

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

// MD5 结构体，包含处理数据所需的各项参数
struct MD5 {
    data: Vec<u8>, // 输入的数据
    bit_len: u64,  // 数据的位长度
    buffer: [u8; 64], // 缓冲区，处理每个 512 位的数据块
    state: [u32; 4],  // 存储 MD5 的 4 个 32 位初始哈希值
}

impl MD5 {
    // 初始化 MD5 结构体
    // new 初始化 MD5 结构体并设置初始状态。
    fn new() -> MD5 {
        MD5 {
            data: Vec::new(),
            bit_len: 0,
            buffer: [0; 64],
            state: [
                0x67452301, // A
                0xefcdab89, // B
                0x98badcfe, // C
                0x10325476, // D
            ],
        }
    }

    // 更新缓冲区，处理输入数据
    // 更新缓冲区和位长度，当缓冲区满 64 字节时调用 transform。
    fn update(&mut self, input: &[u8]) {
        for &byte in input {
            self.buffer[self.bit_len as usize % 64] = byte;
            self.bit_len += 8; // 位长度增加 8 位（1 字节）
            if self.bit_len % 512 == 0 {
                // 当缓冲区达到 512 位（64 字节）时，进行转换
                self.transform();
            }
        }
    }

    // 将缓冲区中的数据转换为 16 个 32 位的字，并执行 MD5 的主要算法
    // 将缓冲区数据转换为 16 个 32 位的字，执行 MD5 主要算法并更新状态。
    fn transform(&mut self) {
        let mut m = [0u32; 16];
        for (i, chunk) in self.buffer.chunks(4).enumerate() {
            m[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                48..=63 => (c ^ (b | !d), (7 * i) % 16),
                _ => panic!("Invalid index"),
            };
            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(m[g]).rotate_left(S[i]));
            a = temp;
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    // 处理填充，并生成最终的哈希值
    // 处理填充并生成最终的 16 字节哈希值。
    fn finalize(&mut self) -> [u8; 16] {
        let bit_len = self.bit_len;
        self.update(&[0x80]); // 追加单个 1 位，后跟 7 个 0 位，即 0x80
        while self.bit_len % 512 != 448 {
            self.update(&[0x00]); // 追加 0 位，直到位长度等于 448（模 512）
        }
        self.update(&bit_len.to_le_bytes()); // 追加原始消息长度的低 64 位
        self.update(&(bit_len >> 32).to_le_bytes()); // 追加原始消息长度的高 64 位（总共 128 位）

        let mut digest = [0u8; 16]; // 最终的哈希值
        for (i, &state) in self.state.iter().enumerate() {
            digest[i * 4..(i + 1) * 4].copy_from_slice(&state.to_le_bytes());
        }

        digest
    }
}

#[test]
fn md5_1() {
    let mut md5 = MD5::new();
    md5.update(b"hello world");
    let result = md5.finalize();
    println!("{}", hex::encode(result));
    assert_eq!("7471c488eb8d3e801b0b892a825c0f2b", hex::encode(result));
}

