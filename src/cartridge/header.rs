use std::io::{Cursor, Read, Seek};

use strum::{AsRefStr, FromRepr};

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u16)]
pub enum NewLicensee {
    None = 0x00,
    Nintendo = 0x01,
    Capcom = 0x08,
    ElectronicArts = 0x13,
}

#[derive(Debug, Clone, Copy, FromRepr, AsRefStr)]
#[repr(u8)]
pub enum CartridgeType {
    NoMBC = 0x00,
    MBC1 = 0x01,
    MBC1RAM = 0x02,
    MBC1RAMBattery = 0x03,
    MBC2 = 0x05,
    MBC2Battery = 0x06,
    NoMBCRAM = 0x08,
    NoMBCRAMBattern = 0x09,
    MMM01 = 0x0B,
    MMM01RAM = 0x0C,
    MMM01RAMBattery = 0x0D,
    MBC3TimerBattery = 0x0F,
    MBC3TimerRAMBattery = 0x10,
    MBC3 = 0x11,
    MBC3RAM = 0x12,
    MBC3RAMBattery = 0x13,
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum ROMSize {
    K32 = 0x00,
    K64 = 0x01,
    K128 = 0x02,
    K256 = 0x03,
    K512 = 0x04,
    M1 = 0x05,
    M2 = 0x06,
    M4 = 0x07,
    M8 = 0x08,
    M1Point1 = 0x52,
    M1Point2 = 0x53,
    M1Point5 = 0x54,
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum SaveRAMSize {
    K0 = 0x00,
    K2 = 0x01,
    K8 = 0x02,
    K32 = 0x03,
    K128 = 0x04,
    K64 = 0x05,
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum CountryCode {
    Japan = 0x00,
    NonJapan = 0x01,
}

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum Licensee {
    None = 0x00,
    Nintendo = 0x01,
    Capcom = 0x08,
}

#[derive(Debug, Clone)]
pub struct CartridgeHeader {
    pub title: String,
    pub new_licensee: NewLicensee,
    pub sgb_features: bool,
    pub cartridge_type: CartridgeType,
    pub rom_size: ROMSize,
    pub save_ram_size: SaveRAMSize,
    pub country_code: CountryCode,
    pub licensee: Licensee,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl CartridgeHeader {
    pub fn from_rom(rom: &[u8]) -> std::io::Result<CartridgeHeader> {
        let header_data = &rom[0x0100..=0x014F];
        let mut cursor = Cursor::new(header_data);

        cursor.seek_relative(4)?; // skip entry point data
        cursor.seek_relative(48)?; // skip nintendo logo

        let mut title_buf: [u8; 16] = [0; _];
        cursor.read_exact(&mut title_buf)?;

        let mut new_licensee_code: [u8; 2] = [0; _];
        cursor.read_exact(&mut new_licensee_code)?;

        let mut sgb_flag: [u8; 1] = [0; _];
        cursor.read_exact(&mut sgb_flag)?;

        let mut cartridge_type_code: [u8; 1] = [0; _];
        cursor.read_exact(&mut cartridge_type_code)?;

        let mut rom_size_code: [u8; 1] = [0; _];
        cursor.read_exact(&mut rom_size_code)?;

        let mut save_ram_size_code: [u8; 1] = [0; _];
        cursor.read_exact(&mut save_ram_size_code)?;

        let mut country_code: [u8; 1] = [0; _];
        cursor.read_exact(&mut country_code)?;

        let mut licensee_code: [u8; 1] = [0; _];
        cursor.read_exact(&mut licensee_code)?;

        let mut mask_rom_version_number: [u8; 1] = [0; _];
        cursor.read_exact(&mut mask_rom_version_number)?;

        let mut header_checksum: [u8; 1] = [0; _];
        cursor.read_exact(&mut header_checksum)?;

        let mut global_checksum_be: [u8; 2] = [0; _];
        cursor.read_exact(&mut global_checksum_be)?;

        Ok(CartridgeHeader {
            title: std::str::from_utf8(&title_buf)
                .expect("Invalid UTF-8 in title")
                .to_string(),
            new_licensee: NewLicensee::None, // TODO: fix reading this as an ASCII string
            sgb_features: sgb_flag[0] == 0x03,
            cartridge_type: CartridgeType::from_repr(cartridge_type_code[0])
                .expect("Unknown cartridge type"),
            rom_size: ROMSize::from_repr(rom_size_code[0]).expect("Unknown ROM size code"),
            save_ram_size: SaveRAMSize::from_repr(save_ram_size_code[0])
                .expect("Unknown save RAM size code"),
            country_code: CountryCode::from_repr(rom_size_code[0]).expect("Unknown country code"),
            licensee: Licensee::from_repr(licensee_code[0]).expect("Unknown licensee code"),
            mask_rom_version_number: mask_rom_version_number[0],
            header_checksum: header_checksum[0],
            global_checksum: ((global_checksum_be[0] as u16) << 8) + (global_checksum_be[1] as u16),
        })
    }
}
