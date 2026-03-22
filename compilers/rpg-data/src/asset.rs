use rpg_types::{Result, RpgError};
use std::io::{Read, Write};

/// RPG Maker 加密文件头部
const RPGM_HEADER: &[u8] = &[
    0x52, 0x50, 0x47, 0x4d, 0x56, 0x00, 0x00, 0x00, 0x00, 0x03, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

/// PNG 文件标准头部
const PNG_HEADER: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d,
    0x49, 0x48, 0x44, 0x52,
];

/// OGG 文件标准头部模板
static mut OGG_HEADER: [u8; 16] = [79, 103, 103, 83, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/// M4A 文件标准头部模板
static mut M4A_HEADER: [u8; 16] = [0, 0, 0, 28, 102, 116, 121, 112, 77, 52, 65, 32, 0, 0, 2, 0];

/// M4A 文件中可能出现的 box 类型，用于确定头部大小
const M4A_POST_HEADER_BOXES: &[&[u8]] = &[b"moov", b"mdat", b"free", b"skip", b"wide", b"pnot"];

/// 十六进制字符集
const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// RPG Maker 默认加密密钥
const DEFAULT_KEY: &str = "d41d8cd98f00b204e9800998ecf8427e";

/// 资源文件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    /// PNG 图片
    Png,
    /// OGG 音频
    Ogg,
    /// M4A 音频
    M4a,
}

/// 加密文件扩展名枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedExtension {
    /// RPG Maker MV PNG 格式
    Rpgmvp,
    /// RPG Maker MZ PNG 格式
    Png_,
    /// RPG Maker MV OGG 格式
    Rpgmvo,
    /// RPG Maker MZ OGG 格式
    Ogg_,
    /// RPG Maker MV M4A 格式
    Rpgmvm,
    /// RPG Maker MZ M4A 格式
    M4a_,
}

impl EncryptedExtension {
    /// 从扩展名字符串创建枚举
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rpgmvp" => Some(Self::Rpgmvp),
            "png_" => Some(Self::Png_),
            "rpgmvo" => Some(Self::Rpgmvo),
            "ogg_" => Some(Self::Ogg_),
            "rpgmvm" => Some(Self::Rpgmvm),
            "m4a_" => Some(Self::M4a_),
            _ => None,
        }
    }

    /// 获取对应的资源类型
    pub fn asset_type(&self) -> AssetType {
        match self {
            Self::Rpgmvp | Self::Png_ => AssetType::Png,
            Self::Rpgmvo | Self::Ogg_ => AssetType::Ogg,
            Self::Rpgmvm | Self::M4a_ => AssetType::M4a,
        }
    }

    /// 获取解密后的文件扩展名
    pub fn output_extension(&self) -> &'static str {
        match self {
            Self::Rpgmvp | Self::Png_ => "png",
            Self::Rpgmvo | Self::Ogg_ => "ogg",
            Self::Rpgmvm | Self::M4a_ => "m4a",
        }
    }
}

/// RPG Maker 加密资源解密器
pub struct Decrypter {
    key: [u8; 16],
    has_key: bool,
}

impl Decrypter {
    /// 创建新的解密器实例
    pub fn new() -> Self {
        Self {
            key: [0; 16],
            has_key: false,
        }
    }

    /// 使用默认密钥创建解密器
    pub fn with_default_key() -> Self {
        let mut decrypter = Self::new();
        decrypter.set_key_from_str(DEFAULT_KEY).unwrap();
        decrypter
    }

    /// 从十六进制字符串设置解密密钥
    pub fn set_key_from_str(&mut self, key_hex: &str) -> Result<()> {
        if key_hex.len() != 32 {
            return Err(RpgError::decode("asset", "Key must be 32 hex characters"));
        }

        for (j, i) in (0..32).step_by(2).enumerate() {
            let hex_byte = &key_hex[i..i + 2];
            self.key[j] = u8::from_str_radix(hex_byte, 16)
                .map_err(|_| RpgError::decode("asset", "Invalid hex character in key"))?;
        }

        self.has_key = true;
        Ok(())
    }

    /// 从加密文件数据中自动提取解密密钥
    pub fn set_key_from_file(&mut self, data: &[u8], asset_type: AssetType) -> Result<()> {
        if !data.starts_with(RPGM_HEADER) {
            return Err(RpgError::decode("asset", "Invalid RPG Maker header"));
        }

        if data.len() < 32 {
            return Err(RpgError::decode("asset", "File too short"));
        }

        let post_header = &data[16..32];
        let mut key_hex = [0u8; 32];

        match asset_type {
            AssetType::Png => {
                for i in 0..16 {
                    let value = PNG_HEADER[i] ^ post_header[i];
                    key_hex[i * 2] = HEX_CHARS[(value >> 4) as usize];
                    key_hex[i * 2 + 1] = HEX_CHARS[(value & 0x0F) as usize];
                }
            }
            AssetType::Ogg => {
                self.prepare_ogg_header(data)?;
                unsafe {
                    for i in 0..16 {
                        let value = OGG_HEADER[i] ^ post_header[i];
                        key_hex[i * 2] = HEX_CHARS[(value >> 4) as usize];
                        key_hex[i * 2 + 1] = HEX_CHARS[(value & 0x0F) as usize];
                    }
                }
            }
            AssetType::M4a => {
                self.prepare_m4a_header(data)?;
                unsafe {
                    for i in 0..16 {
                        let value = M4A_HEADER[i] ^ post_header[i];
                        key_hex[i * 2] = HEX_CHARS[(value >> 4) as usize];
                        key_hex[i * 2 + 1] = HEX_CHARS[(value & 0x0F) as usize];
                    }
                }
            }
        }

        let key_str = std::str::from_utf8(&key_hex)
            .map_err(|_| RpgError::decode("asset", "Invalid key encoding"))?;
        self.set_key_from_str(key_str)
    }

    fn prepare_ogg_header(&mut self, data: &[u8]) -> Result<()> {
        if data.len() < 100 {
            return Err(RpgError::decode("asset", "OGG file too short to determine key"));
        }

        let mut pos = 16;
        let serialno = self.read_ogg_page_serialno(data, &mut pos)?;

        unsafe {
            OGG_HEADER[14..16].copy_from_slice(&serialno.to_le_bytes()[0..2]);
        }

        Ok(())
    }

    fn read_ogg_page_serialno(&self, data: &[u8], pos: &mut usize) -> Result<u32> {
        const HEADER_SIZE: usize = 27;
        const SERIALNO_POS: usize = 14;

        if *pos + HEADER_SIZE > data.len() {
            return Err(RpgError::decode("asset", "Invalid OGG page header"));
        }

        let header = &data[*pos..*pos + HEADER_SIZE];
        *pos += HEADER_SIZE;

        let segment_count = header[26] as usize;

        if *pos + 255 > data.len() {
            return Err(RpgError::decode("asset", "Invalid OGG segment table"));
        }

        let segment_table = &data[*pos..*pos + 255];
        *pos += 255;

        let over_count = 255 - segment_count;
        *pos -= over_count;

        let mut body_length = 0;
        for &segment in &segment_table[..segment_count] {
            body_length += segment as usize;
        }

        *pos += body_length;

        let serialno = u32::from_le_bytes([
            header[SERIALNO_POS],
            header[SERIALNO_POS + 1],
            header[SERIALNO_POS + 2],
            header[SERIALNO_POS + 3],
        ]);

        Ok(serialno)
    }

    fn prepare_m4a_header(&mut self, data: &[u8]) -> Result<()> {
        if data.len() < 80 {
            return Err(RpgError::decode("asset", "M4A file too short to determine key"));
        }

        let file_start = &data[16..80];
        let chunks = file_start.chunks_exact(4);

        for (i, chunk) in chunks.enumerate() {
            if M4A_POST_HEADER_BOXES.contains(&chunk) && i > 0 {
                let prev_chunk_i = i - 1;
                let header_type_box_size = (prev_chunk_i * 4) as u32;

                unsafe {
                    M4A_HEADER[0..4].copy_from_slice(&header_type_box_size.to_be_bytes());
                }
                break;
            }
        }

        Ok(())
    }

    /// 解密加密的文件数据，返回解密后的字节数组
    pub fn decrypt(&mut self, data: &[u8], asset_type: AssetType) -> Result<Vec<u8>> {
        if !data.starts_with(RPGM_HEADER) {
            return Err(RpgError::decode("asset", "Invalid RPG Maker header"));
        }

        if !self.has_key {
            self.set_key_from_file(data, asset_type)?;
        }

        let mut result = data[16..].to_vec();
        self.xor_buffer(&mut result);

        self.verify_decrypted(&result, asset_type)?;

        Ok(result)
    }

    /// 就地解密加密的文件数据，返回解密后的字节切片引用
    pub fn decrypt_in_place<'a>(&'a mut self, data: &'a mut [u8], asset_type: AssetType) -> Result<&'a [u8]> {
        if !data.starts_with(RPGM_HEADER) {
            return Err(RpgError::decode("asset", "Invalid RPG Maker header"));
        }

        if !self.has_key {
            self.set_key_from_file(data, asset_type)?;
        }

        let sliced = &mut data[16..];
        self.xor_buffer(sliced);

        self.verify_decrypted(sliced, asset_type)?;

        Ok(sliced)
    }

    fn xor_buffer(&self, buffer: &mut [u8]) {
        for (i, item) in buffer.iter_mut().enumerate().take(16) {
            *item ^= self.key[i];
        }
    }

    fn verify_decrypted(&self, data: &[u8], asset_type: AssetType) -> Result<()> {
        match asset_type {
            AssetType::Png => {
                if data.len() < 8 || &data[0..8] != b"\x89PNG\r\n\x1a\n" {
                    return Err(RpgError::decode("asset", "Decrypted PNG has invalid signature"));
                }
            }
            AssetType::Ogg => {
                if data.len() < 4 || &data[0..4] != b"OggS" {
                    return Err(RpgError::decode("asset", "Decrypted OGG has invalid signature"));
                }
            }
            AssetType::M4a => {
                if data.len() < 12 || &data[4..8] != b"ftyp" {
                    return Err(RpgError::decode("asset", "Decrypted M4A has invalid signature"));
                }
            }
        }
        Ok(())
    }
}

impl Default for Decrypter {
    fn default() -> Self {
        Self::new()
    }
}

/// 读取并解密加密的资源文件，同时保存解密后的文件
pub fn read_asset(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| RpgError::decode("asset", "No file extension found"))?;

    let encrypted_ext = EncryptedExtension::from_extension(ext)
        .ok_or_else(|| RpgError::decode("asset", "Unsupported encrypted extension"))?;

    let mut decrypter = Decrypter::new();
    let decrypted = decrypter.decrypt(&buffer, encrypted_ext.asset_type())?;

    let output_path = std::path::Path::new(file_path)
        .with_extension(encrypted_ext.output_extension());

    let mut output_file = std::fs::File::create(&output_path)?;
    output_file.write_all(&decrypted)?;

    Ok(format!("成功解密 {} -> {}", file_path, output_path.to_string_lossy()))
}

/// 解密加密的资源数据，返回解密后的字节数组
pub fn decrypt_asset_data(data: &[u8], asset_type: AssetType) -> Result<Vec<u8>> {
    let mut decrypter = Decrypter::new();
    decrypter.decrypt(data, asset_type)
}
