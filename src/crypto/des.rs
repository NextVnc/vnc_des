//! VNC协议DES算法实现
//!
//! 这个模块实现了符合VNC协议标准（RFC 6143）的DES认证算法
//! 注意：这是VNC协议特化的DES算法，与标准DES有所不同

use crate::error::Result;

/// VNC协议特化的DES实现常量和表
/// 字节位数组 - 已反转用于VNC兼容性
const BYTEBIT: [u16; 8] = [0o01, 0o02, 0o04, 0o010, 0o020, 0o040, 0o0100, 0o0200];

const BIGBYTE: [u32; 24] = [
    0x800000, 0x400000, 0x200000, 0x100000, 0x80000, 0x40000, 0x20000, 0x10000, 0x8000, 0x4000,
    0x2000, 0x1000, 0x800, 0x400, 0x200, 0x100, 0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1,
];

const PC1: [u8; 56] = [
    56, 48, 40, 32, 24, 16, 8, 0, 57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59,
    51, 43, 35, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29, 21, 13, 5, 60, 52, 44, 36, 28,
    20, 12, 4, 27, 19, 11, 3,
];

const TOTROT: [u8; 16] = [1, 2, 4, 6, 8, 10, 12, 14, 15, 17, 19, 21, 23, 25, 27, 28];

const PC2: [u8; 48] = [
    13, 16, 10, 23, 0, 4, 2, 27, 14, 5, 20, 9, 22, 18, 11, 3, 25, 7, 15, 6, 26, 19, 12, 1, 40, 51,
    30, 36, 46, 54, 29, 39, 50, 44, 32, 47, 43, 48, 38, 55, 33, 52, 45, 41, 49, 35, 28, 31,
];

// S盒表
const SP1: [u32; 64] = [
    0x01010400, 0x00000000, 0x00010000, 0x01010404, 0x01010004, 0x00010404, 0x00000004, 0x00010000,
    0x00000400, 0x01010400, 0x01010404, 0x00000400, 0x01000404, 0x01010004, 0x01000000, 0x00000004,
    0x00000404, 0x01000400, 0x01000400, 0x00010400, 0x00010400, 0x01010000, 0x01010000, 0x01000404,
    0x00010004, 0x01000004, 0x01000004, 0x00010004, 0x00000000, 0x00000404, 0x00010404, 0x01000000,
    0x00010000, 0x01010404, 0x00000004, 0x01010000, 0x01010400, 0x01000000, 0x01000000, 0x00000400,
    0x01010004, 0x00010000, 0x00010400, 0x01000004, 0x00000400, 0x00000004, 0x01000404, 0x00010404,
    0x01010404, 0x00010004, 0x01010000, 0x01000404, 0x01000004, 0x00000404, 0x00010404, 0x01010400,
    0x00000404, 0x01000400, 0x01000400, 0x00000000, 0x00010004, 0x00010400, 0x00000000, 0x01010004,
];

const SP2: [u32; 64] = [
    0x80108020, 0x80008000, 0x00008000, 0x00108020, 0x00100000, 0x00000020, 0x80100020, 0x80008020,
    0x80000020, 0x80108020, 0x80108000, 0x80000000, 0x80008000, 0x00100000, 0x00000020, 0x80100020,
    0x00108000, 0x00100020, 0x80008020, 0x00000000, 0x80000000, 0x00008000, 0x00108020, 0x80100000,
    0x00100020, 0x80000020, 0x00000000, 0x00108000, 0x00008020, 0x80108000, 0x80100000, 0x00008020,
    0x00000000, 0x00108020, 0x80100020, 0x00100000, 0x80008020, 0x80100000, 0x80108000, 0x00008000,
    0x80100000, 0x80008000, 0x00000020, 0x80108020, 0x00108020, 0x00000020, 0x00008000, 0x80000000,
    0x00008020, 0x80108000, 0x00100000, 0x80000020, 0x00100020, 0x80008020, 0x80000020, 0x00100020,
    0x00108000, 0x00000000, 0x80008000, 0x00008020, 0x80000000, 0x80100020, 0x80108020, 0x00108000,
];

const SP3: [u32; 64] = [
    0x00000208, 0x08020200, 0x00000000, 0x08020008, 0x08000200, 0x00000000, 0x00020208, 0x08000200,
    0x00020008, 0x08000008, 0x08000008, 0x00020000, 0x08020208, 0x00020008, 0x08020000, 0x00000208,
    0x08000000, 0x00000008, 0x08020200, 0x00000200, 0x00020200, 0x08020000, 0x08020008, 0x00020208,
    0x08000208, 0x00020200, 0x00020000, 0x08000208, 0x00000008, 0x08020208, 0x00000200, 0x08000000,
    0x08020200, 0x08000000, 0x00020008, 0x00000208, 0x00020000, 0x08020200, 0x08000200, 0x00000000,
    0x00000200, 0x00020008, 0x08020208, 0x08000200, 0x08000008, 0x00000200, 0x00000000, 0x08020008,
    0x08000208, 0x00020000, 0x08000000, 0x08020208, 0x00000008, 0x00020208, 0x00020200, 0x08000008,
    0x08020000, 0x08000208, 0x00000208, 0x08020000, 0x00020208, 0x00000008, 0x08020008, 0x00020200,
];

const SP4: [u32; 64] = [
    0x00802001, 0x00002081, 0x00002081, 0x00000080, 0x00802080, 0x00800081, 0x00800001, 0x00002001,
    0x00000000, 0x00802000, 0x00802000, 0x00802081, 0x00000081, 0x00000000, 0x00800080, 0x00800001,
    0x00000001, 0x00002000, 0x00800000, 0x00802001, 0x00000080, 0x00800000, 0x00002001, 0x00002080,
    0x00800081, 0x00000001, 0x00002080, 0x00800080, 0x00002000, 0x00802080, 0x00802081, 0x00000081,
    0x00800080, 0x00800001, 0x00802000, 0x00802081, 0x00000081, 0x00000000, 0x00000000, 0x00802000,
    0x00002080, 0x00800080, 0x00800081, 0x00000001, 0x00802001, 0x00002081, 0x00002081, 0x00000080,
    0x00802081, 0x00000081, 0x00000001, 0x00002000, 0x00800001, 0x00002001, 0x00802080, 0x00800081,
    0x00002001, 0x00002080, 0x00800000, 0x00802001, 0x00000080, 0x00800000, 0x00002000, 0x00802080,
];

const SP5: [u32; 64] = [
    0x00000100, 0x02080100, 0x02080000, 0x42000100, 0x00080000, 0x00000100, 0x40000000, 0x02080000,
    0x40080100, 0x00080000, 0x02000100, 0x40080100, 0x42000100, 0x42080000, 0x00080100, 0x40000000,
    0x02000000, 0x40080000, 0x40080000, 0x00000000, 0x40000100, 0x42080100, 0x42080100, 0x02000100,
    0x42080000, 0x40000100, 0x00000000, 0x42000000, 0x02080100, 0x02000000, 0x42000000, 0x00080100,
    0x00080000, 0x42000100, 0x00000100, 0x02000000, 0x40000000, 0x02080000, 0x42000100, 0x40080100,
    0x02000100, 0x40000000, 0x42080000, 0x02080100, 0x40080100, 0x00000100, 0x02000000, 0x42080000,
    0x42080100, 0x00080100, 0x42000000, 0x42080100, 0x02080000, 0x00000000, 0x40080000, 0x42000000,
    0x00080100, 0x02000100, 0x40000100, 0x00080000, 0x00000000, 0x40080000, 0x02080100, 0x40000100,
];

const SP6: [u32; 64] = [
    0x20000010, 0x20400000, 0x00004000, 0x20404010, 0x20400000, 0x00000010, 0x20404010, 0x00400000,
    0x20004000, 0x00404010, 0x00400000, 0x20000010, 0x00400010, 0x20004000, 0x20000000, 0x00004010,
    0x00000000, 0x00400010, 0x20004010, 0x00004000, 0x00404000, 0x20004010, 0x00000010, 0x20400010,
    0x20400010, 0x00000000, 0x00404010, 0x20404000, 0x00004010, 0x00404000, 0x20404000, 0x20000000,
    0x20004000, 0x00000010, 0x20400010, 0x00404000, 0x20404010, 0x00400000, 0x00004010, 0x20000010,
    0x00400000, 0x20004000, 0x20000000, 0x00004010, 0x20000010, 0x20404010, 0x00404000, 0x20400000,
    0x00404010, 0x20404000, 0x00000000, 0x20400010, 0x00000010, 0x00004000, 0x20400000, 0x00404010,
    0x00004000, 0x00400010, 0x20004010, 0x00000000, 0x20404000, 0x20000000, 0x00400010, 0x20004010,
];

const SP7: [u32; 64] = [
    0x00200000, 0x04200002, 0x04000802, 0x00000000, 0x00000800, 0x04000802, 0x00200802, 0x04200800,
    0x04200802, 0x00200000, 0x00000000, 0x04000002, 0x00000002, 0x04000000, 0x04200002, 0x00000802,
    0x04000800, 0x00200802, 0x00200002, 0x04000800, 0x04000002, 0x04200000, 0x04200800, 0x00200002,
    0x04200000, 0x00000800, 0x00000802, 0x04200802, 0x00200800, 0x00000002, 0x04000000, 0x00200800,
    0x04000000, 0x00200800, 0x00200000, 0x04000802, 0x04000802, 0x04200002, 0x04200002, 0x00000002,
    0x00200002, 0x04000000, 0x04000800, 0x00200000, 0x04200800, 0x00000802, 0x00200802, 0x04200800,
    0x00000802, 0x04000002, 0x04200802, 0x04200000, 0x00200800, 0x00000000, 0x00000002, 0x04200802,
    0x00000000, 0x00200802, 0x04200000, 0x00000800, 0x04000002, 0x04000800, 0x00000800, 0x00200002,
];

const SP8: [u32; 64] = [
    0x10001040, 0x00001000, 0x00040000, 0x10041040, 0x10000000, 0x10001040, 0x00000040, 0x10000000,
    0x00040040, 0x10040000, 0x10041040, 0x00041000, 0x10041000, 0x00041040, 0x00001000, 0x00000040,
    0x10040000, 0x10000040, 0x10001000, 0x00001040, 0x00041000, 0x00040040, 0x10040040, 0x10041000,
    0x00001040, 0x00000000, 0x00000000, 0x10040040, 0x10000040, 0x10001000, 0x00041040, 0x00040000,
    0x00041040, 0x00040000, 0x10041000, 0x00001000, 0x00000040, 0x10040040, 0x00001000, 0x00041040,
    0x10001000, 0x00000040, 0x10000040, 0x10040000, 0x10040040, 0x10000000, 0x00040000, 0x10001040,
    0x00000000, 0x10041040, 0x00040040, 0x10000040, 0x10040000, 0x10001000, 0x10001040, 0x00000000,
    0x10041040, 0x00041000, 0x00041000, 0x00001040, 0x00001040, 0x00040040, 0x10000000, 0x10041000,
];

/// VNC DES引擎 - 完全基于TightVNC参考实现
#[derive(Debug, Clone)]
pub struct VncDesEngine {
    /// 子密钥数组
    kn_l: [u32; 32],
}

impl Default for VncDesEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl VncDesEngine {
    /// 创建新的DES引擎实例
    pub fn new() -> Self {
        Self { kn_l: [0; 32] }
    }

    /// 清空密钥
    pub fn clear_key(&mut self) {
        self.kn_l.fill(0);
    }

    /// 设置DES密钥（参考实现兼容）
    pub fn deskey(&mut self, hex_key: &[u8; 8], encrypt: bool) {
        let mut pc1m = [0u8; 56];
        let mut pcr = [0u8; 56];
        let mut kn = [0u32; 32];

        // PC1 permutation
        for j in 0..56 {
            let l = PC1[j] as usize;
            let m = l & 0o7;
            pc1m[j] = if (hex_key[l >> 3] & BYTEBIT[m] as u8) != 0 {
                1
            } else {
                0
            };
        }

        for i in 0..16 {
            let m = if encrypt { i << 1 } else { (15 - i) << 1 };
            let n = m + 1;
            kn[m] = 0;
            kn[n] = 0;

            for j in 0..28 {
                let l = j + TOTROT[i] as usize;
                pcr[j] = if l < 28 { pc1m[l] } else { pc1m[l - 28] };
            }

            for j in 28..56 {
                let l = j + TOTROT[i] as usize;
                pcr[j] = if l < 56 { pc1m[l] } else { pc1m[l - 28] };
            }

            for j in 0..24 {
                if pcr[PC2[j] as usize] != 0 {
                    kn[m] |= BIGBYTE[j];
                }
                if pcr[PC2[j + 24] as usize] != 0 {
                    kn[n] |= BIGBYTE[j];
                }
            }
        }

        self.cookey(&kn);
    }

    /// 处理密钥（参考实现）
    fn cookey(&mut self, raw1: &[u32; 32]) {
        let mut dough = [0u32; 32];
        let mut raw_iter = raw1.chunks_exact(2);

        for i in 0..16 {
            let raw_pair = raw_iter.next().unwrap();
            let raw0 = raw_pair[0];
            let raw1 = raw_pair[1];

            dough[i * 2] = ((raw0 & 0x00fc0000) << 6)
                | ((raw0 & 0x00000fc0) << 10)
                | ((raw1 & 0x00fc0000) >> 10)
                | ((raw1 & 0x00000fc0) >> 6);

            dough[i * 2 + 1] = ((raw0 & 0x0003f000) << 12)
                | ((raw0 & 0x0000003f) << 16)
                | ((raw1 & 0x0003f000) >> 4)
                | (raw1 & 0x0000003f);
        }

        self.kn_l.copy_from_slice(&dough);
    }

    /// 将8字节数组转换为2个u32（参考实现）
    fn scrunch(outof: &[u8; 8]) -> [u32; 2] {
        [
            ((outof[0] as u32) << 24)
                | ((outof[1] as u32) << 16)
                | ((outof[2] as u32) << 8)
                | (outof[3] as u32),
            ((outof[4] as u32) << 24)
                | ((outof[5] as u32) << 16)
                | ((outof[6] as u32) << 8)
                | (outof[7] as u32),
        ]
    }

    /// 将2个u32转换为8字节数组（参考实现）
    fn unscrun(outof: &[u32; 2]) -> [u8; 8] {
        [
            ((outof[0] >> 24) & 0xff) as u8,
            ((outof[0] >> 16) & 0xff) as u8,
            ((outof[0] >> 8) & 0xff) as u8,
            (outof[0] & 0xff) as u8,
            ((outof[1] >> 24) & 0xff) as u8,
            ((outof[1] >> 16) & 0xff) as u8,
            ((outof[1] >> 8) & 0xff) as u8,
            (outof[1] & 0xff) as u8,
        ]
    }

    /// DES轮函数（参考实现 - 包含完整的初始和最终置换）
    fn desfunc(&self, block: &mut [u32; 2]) {
        let mut leftt = block[0];
        let mut right = block[1];

        // Initial permutation
        let mut work = ((leftt >> 4) ^ right) & 0x0f0f0f0f;
        right ^= work;
        leftt ^= work << 4;
        work = ((leftt >> 16) ^ right) & 0x0000ffff;
        right ^= work;
        leftt ^= work << 16;
        work = ((right >> 2) ^ leftt) & 0x33333333;
        leftt ^= work;
        right ^= work << 2;
        work = ((right >> 8) ^ leftt) & 0x00ff00ff;
        leftt ^= work;
        right ^= work << 8;
        right = ((right << 1) | ((right >> 31) & 1)) & 0xffffffff;
        work = (leftt ^ right) & 0xaaaaaaaa;
        leftt ^= work;
        right ^= work;
        leftt = ((leftt << 1) | ((leftt >> 31) & 1)) & 0xffffffff;

        // 16 rounds
        for round in 0..8 {
            let key_idx = round * 4;

            work = ((right << 28) | (right >> 4)) ^ self.kn_l[key_idx];
            let mut fval = SP7[(work & 0x3f) as usize]
                | SP5[((work >> 8) & 0x3f) as usize]
                | SP3[((work >> 16) & 0x3f) as usize]
                | SP1[((work >> 24) & 0x3f) as usize];

            work = right ^ self.kn_l[key_idx + 1];
            fval |= SP8[(work & 0x3f) as usize]
                | SP6[((work >> 8) & 0x3f) as usize]
                | SP4[((work >> 16) & 0x3f) as usize]
                | SP2[((work >> 24) & 0x3f) as usize];

            leftt ^= fval;

            work = ((leftt << 28) | (leftt >> 4)) ^ self.kn_l[key_idx + 2];
            fval = SP7[(work & 0x3f) as usize]
                | SP5[((work >> 8) & 0x3f) as usize]
                | SP3[((work >> 16) & 0x3f) as usize]
                | SP1[((work >> 24) & 0x3f) as usize];

            work = leftt ^ self.kn_l[key_idx + 3];
            fval |= SP8[(work & 0x3f) as usize]
                | SP6[((work >> 8) & 0x3f) as usize]
                | SP4[((work >> 16) & 0x3f) as usize]
                | SP2[((work >> 24) & 0x3f) as usize];

            right ^= fval;
        }

        // Final permutation
        right = (right << 31) | (right >> 1);
        work = (leftt ^ right) & 0xaaaaaaaa;
        leftt ^= work;
        right ^= work;
        leftt = (leftt << 31) | (leftt >> 1);
        work = ((leftt >> 8) ^ right) & 0x00ff00ff;
        right ^= work;
        leftt ^= work << 8;
        work = ((leftt >> 2) ^ right) & 0x33333333;
        right ^= work;
        leftt ^= work << 2;
        work = ((right >> 16) ^ leftt) & 0x0000ffff;
        leftt ^= work;
        right ^= work << 16;
        work = ((right >> 4) ^ leftt) & 0x0f0f0f0f;
        leftt ^= work;
        right ^= work << 4;

        block[0] = right;
        block[1] = leftt;
    }

    /// 执行DES加密/解密（参考实现）
    pub fn des(&mut self, from: &[u8; 8], to: &mut [u8; 8]) {
        let mut work = Self::scrunch(from);
        self.desfunc(&mut work);
        *to = Self::unscrun(&work);
    }

    /// 加密8字节块（参考实现兼容）
    pub fn encrypt(&mut self, dst: &mut [u8; 8], src: &[u8; 8], key: &[u8; 8]) -> Result<()> {
        self.deskey(key, true);
        self.des(src, dst);
        self.clear_key();
        Ok(())
    }

    /// 解密8字节块（参考实现兼容）
    pub fn decrypt(&mut self, dst: &mut [u8; 8], src: &[u8; 8], key: &[u8; 8]) -> Result<()> {
        self.deskey(key, false);
        self.des(src, dst);
        self.clear_key();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_des_engine_creation() {
        let engine = VncDesEngine::new();
        assert_eq!(engine.kn_l, [0; 32]);
    }

    #[test]
    fn test_scrunch_unscrun() {
        let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let packed = VncDesEngine::scrunch(&data);
        let unpacked = VncDesEngine::unscrun(&packed);
        assert_eq!(data, unpacked);
    }

    #[test]
    fn test_encryption_compatibility() {
        let mut engine = VncDesEngine::new();
        let key = [23, 82, 107, 6, 35, 78, 88, 7];
        let password = "test";

        // 准备8字节的密码缓冲区
        let mut password_bytes = [0u8; 8];
        let pwd_bytes = password.as_bytes();
        let copy_len = std::cmp::min(pwd_bytes.len(), 8);
        password_bytes[..copy_len].copy_from_slice(&pwd_bytes[..copy_len]);

        let mut encrypted = [0u8; 8];
        engine
            .encrypt(&mut encrypted, &password_bytes, &key)
            .unwrap();

        // 检查结果应该与参考实现匹配: 2f981dc548e09ec2
        let expected = [0x2f, 0x98, 0x1d, 0xc5, 0x48, 0xe0, 0x9e, 0xc2];
        assert_eq!(encrypted, expected);
    }
}
