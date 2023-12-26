#![allow(unused_variables)]
#![allow(dead_code)]
#![forbid(unsafe_code)]

extern crate numrs;
use numrs::matrix;
use numrs::matrix::Matrix;

struct AesKey {
    key: Matrix<u8>,
}

impl AesKey {
    fn new(key: &[u8]) -> Self {
        //key初始为4*4的矩阵
        let mut key_matrix = matrix::from_elems(4, 4, key);
        key_matrix.transpose();

        //将key填充为4*(4*11)的矩阵
        let mut round_key = Matrix::new(4, 4 * 11, 0u8);
        key_expansion(&mut round_key, &key_matrix);
        return Self {
            key: round_key,
        }
    }
}


pub struct AesEcb {
    aes_key: AesKey,
}

impl AesEcb {
    pub fn new(key: &[u8]) -> Self {
        return Self {
            aes_key: AesKey::new(key),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> String {
        let mut ciphertext = String::new();
    
        //每16位进行一次加密
        let mut index = 0;
        loop {
            let mut state: Matrix<u8>;
            //初始化一个4*4的全0矩阵
            state = matrix::from_elems(4, 4, &[0u8; 16]);
            for i in 0..state.num_rows() {
                for j in 0..state.num_cols() {
                    if index >= plaintext.len() {
                        break;
                    }
                    //将byte_array的值填充进state
                    state.set(j, i, plaintext[index]);
                    index += 1;
                }
            }
            //加密该明文段
            encrypt_state_block(&mut state, &self.aes_key.key);
            //append密文
            encrypted_append(&mut ciphertext, &state);
    
            if index >= plaintext.len() {
                break;
            }
        }
        ciphertext
    }
}

fn get_sbox() -> Matrix<u8> {
    let sbox = [
        0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB,
        0x76, 0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4,
        0x72, 0xC0, 0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71,
        0xD8, 0x31, 0x15, 0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2,
        0xEB, 0x27, 0xB2, 0x75, 0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6,
        0xB3, 0x29, 0xE3, 0x2F, 0x84, 0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB,
        0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF, 0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45,
        0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8, 0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5,
        0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2, 0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44,
        0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73, 0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A,
        0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB, 0xE0, 0x32, 0x3A, 0x0A, 0x49,
        0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79, 0xE7, 0xC8, 0x37, 0x6D,
        0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08, 0xBA, 0x78, 0x25,
        0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A, 0x70, 0x3E,
        0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E, 0xE1,
        0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
        0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB,
        0x16,
    ];
    let sbox = matrix::from_elems(16, 16, &sbox);
    sbox
}

fn get_rcon_col(col: usize) -> Matrix<u8> {
    //对于AES-128，用11个值的常量数组
    let rcon = [
        0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
    ];
    //取对应的值转换为一维矩阵
    let rcon = matrix::from_elems(4, 1, &[rcon[col], 0, 0, 0]);
    rcon
}


fn matrix_row_rotate(m: &mut Matrix<u8>, row: usize, iters: usize) {
    //将某一行的某个位置的数移动到最左侧
    //其他数字跟着移动，总体顺序不变
    //比如将[a0,a1,a2,a3]的a0移到最左，变成：[a1,a2,a3,a0]
    let row_nums = [
        m.get(row, (0 + iters) % 4),
        m.get(row, (1 + iters) % 4),
        m.get(row, (2 + iters) % 4),
        m.get(row, (3 + iters) % 4),
    ];

    for col in 0..4 {
        m.set(row, col, row_nums[col]);
    }
}

fn shift_rows(state: &mut Matrix<u8>) {
    for depth in 1..state.num_rows() {
        matrix_row_rotate(state, depth, depth);
    }
}

fn mix_single_column(col: &mut [u8; 4]) {
    let mut a = [0u8; 4];
    let mut b = [0u8; 4];

    //a[n]是输入数组的复制
    //a[n] ^ b[n]是将a[n]乘了3，可能存在溢出
    for i in 0..4 {
        a[i] = col[i];
        //h是col[i]的最高位
        let h = ((col[i] as i8) >> 7) as u8;
        b[i] = col[i] << 1;
        b[i] ^= 0x1b & h;
    }
    col[0] = b[0] ^ a[3] ^ a[2] ^ b[1] ^ a[1]; // 2 * a0 + a3 + a2 + 3 * a1
    col[1] = b[1] ^ a[0] ^ a[3] ^ b[2] ^ a[2]; // 2 * a1 + a0 + a3 + 3 * a2
    col[2] = b[2] ^ a[1] ^ a[0] ^ b[3] ^ a[3]; // 2 * a2 + a1 + a0 + 3 * a3
    col[3] = b[3] ^ a[2] ^ a[1] ^ b[0] ^ a[0]; // 2 * a3 + a2 + a1 + 3 * a0
}

fn mix_columns(state: &mut Matrix<u8>) {
    for col in 0..state.num_cols() {
        //取出每一列
        let mut m_col = [
            state.get(0, col),
            state.get(1, col),
            state.get(2, col),
            state.get(3, col),
        ];

        //mix column
        mix_single_column(&mut m_col);
        for row in 0..state.num_rows() {
            state.set(row, col, m_col[row]);
        }
    }
}

fn xor_matricies(m1: &mut Matrix<u8>, m2: &Matrix<u8>) {
    let v1 = m1.get_vec();
    let v2 = m2.get_vec();
    let length = m1.num_rows() * m1.num_cols();

    let mut temp_vec = Vec::new();
    let mut index = 0;

    for i in 0..length {
        temp_vec.push(v1[i] ^ v2[i]);
    }
    for i in 0..m1.num_rows() {
        for j in 0..m1.num_cols() {
            let val = temp_vec[index];
            m1.set(i, j, val);
            index += 1;
        }
    }
}

fn key_expansion(round_key: &mut Matrix<u8>, key: &Matrix<u8>) {
    // round_key的第一个4*4的矩阵块就是key
    for i in 0..round_key.num_rows() {
        for j in 0..4 {
            round_key.set(i, j, key.get(i, j));
        }
    }

    //处理剩下的10个4*4的矩阵
    for i in 4..4 * 11 {
        //先将前一个列的值的复制过来，变成1*4的一维矩阵
        let mut col = matrix::from_elems(
            1,
            4,
            &[
                round_key.get(0, i - 1),
                round_key.get(1, i - 1),
                round_key.get(2, i - 1),
                round_key.get(3, i - 1),
            ],
        );

        //如果是一个(4*4)矩阵块的第一列
        if i % 4 == 0 {
            //将第二位挪动到最左边
            matrix_row_rotate(&mut col, 0, 1);
            //将该列进行字符替换，打乱为新的列
            sub_bytes(&mut col);

            //对最左的位置的数进行异或运算
            //rcon_col形式为[a, 0, 0, 0]
            xor_matricies(&mut col, &get_rcon_col(i / 4));
        }

        //与上一个4*4的矩阵块对应的位置的列进行异或运算
        let init_col = matrix::from_elems(
            1,
            4,
            &[
                round_key.get(0, i - 4),
                round_key.get(1, i - 4),
                round_key.get(2, i - 4),
                round_key.get(3, i - 4),
            ],
        );
        xor_matricies(&mut col, &init_col);

        //将结果矩阵转置、更新到round_key的列
        for j in 0..round_key.num_rows() {
            round_key.set(j, i, col.get(0, j));
        }
    }
}


fn sub_bytes(state: &mut Matrix<u8>) {
    //根据s-box将原本的值替换为另一个不同的值
    let sbox = get_sbox();
    for i in 0..state.num_rows() {
        for j in 0..state.num_cols() {
            let byte = state.get(i, j);
            //取当前值的高四位和低四位作为映射的索引
            let hex_col = byte & 0x0f;
            let hex_row = (byte & 0xf0) / 0x10;
            state.set(i, j, sbox.get(hex_row as usize, hex_col as usize));
        }
    }
}

fn xor_round_key(state: &mut Matrix<u8>, round_key: &Matrix<u8>, round: usize) {
    let mut round_key_chunk = Matrix::new(4, 4, 0u8);
    //取出key的第round+1个(4*4)的矩阵
    for i in 0..state.num_rows() {
        for j in round * 4..round * 4 + 4 {
            round_key_chunk.set(i, j % 4, round_key.get(i, j));
        }
    }
    //逐位与明文进行异或
    xor_matricies(state, &round_key_chunk);
}

fn encrypt_state_block(state: &mut Matrix<u8>, round_key: &Matrix<u8>) {
    //将明文与Key中的每一个4*4的矩阵块进行异或
    xor_round_key(state, round_key, 0);
    for round in 1..11 {
        //打乱中间结果
        sub_bytes(state);
        shift_rows(state);
        if round != 10 {
            mix_columns(state);
        }
        xor_round_key(state, round_key, round);
    }
}

fn encrypted_append(string: &mut String, state: &Matrix<u8>) {
    for i in 0..state.num_rows() {
        for j in 0..state.num_cols() {
            string.push_str(&format!("{:02x}", state.get(j, i)));
        }
    }
}

#[allow(dead_code)]
fn print_matrix(m: &Matrix<u8>) {
    for i in 0..m.num_rows() {
        print!("|");
        for j in 0..m.num_cols() {
            print!("{:02x}|", m.get(i, j))
        }
        println!("");
    }
    println!();
}
