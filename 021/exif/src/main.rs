// cargo run -- name.jpg
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    if let Err(err) = run() {
        eprintln!("エラー: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("使い方: exif <画像ファイルパス>")?;

    let mut buf = Vec::new();
    BufReader::new(File::open(&path)?).read_to_end(&mut buf)?;

    let summary = extract_summary(&buf).map_err(|e| format!("EXIF解析エラー: {e}"))?;

    println!("=== EXIF情報 ===");
    println!("ファイル: {path}");
    println!("---------------------------");
    println!(
        "カメラメーカー: {}",
        summary.make.unwrap_or_else(|| "情報なし".into())
    );
    println!(
        "カメラモデル  : {}",
        summary.model.unwrap_or_else(|| "情報なし".into())
    );
    println!(
        "撮影日時      : {}",
        summary.datetime.unwrap_or_else(|| "情報なし".into())
    );
    println!(
        "露光時間      : {}",
        summary.exposure.unwrap_or_else(|| "情報なし".into())
    );
    println!(
        "F値           : {}",
        summary.fnumber.unwrap_or_else(|| "情報なし".into())
    );
    println!(
        "ISO感度       : {}",
        summary.iso.unwrap_or_else(|| "情報なし".into())
    );

    Ok(())
}

struct ExifSummary {
    make: Option<String>,
    model: Option<String>,
    datetime: Option<String>,
    exposure: Option<String>,
    fnumber: Option<String>,
    iso: Option<String>,
}

fn extract_summary(data: &[u8]) -> Result<ExifSummary, String> {
    let tiff = extract_tiff_data(data)?;
    let order = ByteOrder::parse(tiff).ok_or("TIFFヘッダーのバイトオーダーが不正です")?;
    if read_u16(tiff, 2, order)? != 0x002A {
        return Err("TIFF識別子が不正です".into());
    }
    let ifd0_offset = read_u32(tiff, 4, order)? as usize;
    let (ifd0, _) = parse_ifd(tiff, ifd0_offset, order)?;

    let exif_ifd = find_entry(&ifd0, TAG_EXIF_IFD)
        .and_then(|entry| entry.as_u32(order))
        .and_then(|offset| {
            parse_ifd(tiff, offset as usize, order)
                .ok()
                .map(|(entries, _)| entries)
        });

    let make = find_entry(&ifd0, TAG_MAKE).and_then(|entry| entry.as_ascii());
    let model = find_entry(&ifd0, TAG_MODEL).and_then(|entry| entry.as_ascii());

    let datetime = exif_ifd
        .as_ref()
        .and_then(|entries| find_entry(entries, TAG_DATETIME_ORIGINAL))
        .and_then(|entry| entry.as_ascii())
        .or_else(|| find_entry(&ifd0, TAG_DATETIME).and_then(|entry| entry.as_ascii()));

    let exposure = exif_ifd
        .as_ref()
        .and_then(|entries| find_entry(entries, TAG_EXPOSURE_TIME))
        .and_then(|entry| {
            entry
                .as_rational(order)
                .and_then(|vals| vals.first().cloned())
        })
        .map(|(num, denom)| format_rational(num, denom, false));

    let fnumber = exif_ifd
        .as_ref()
        .and_then(|entries| find_entry(entries, TAG_FNUMBER))
        .and_then(|entry| {
            entry
                .as_rational(order)
                .and_then(|vals| vals.first().cloned())
        })
        .map(|(num, denom)| format_rational(num, denom, true));

    let iso = exif_ifd
        .as_ref()
        .and_then(|entries| find_entry(entries, TAG_ISO_SPEED))
        .and_then(|entry| entry.as_short(order))
        .and_then(|vals| vals.first().cloned())
        .map(|value| value.to_string());

    Ok(ExifSummary {
        make,
        model,
        datetime,
        exposure,
        fnumber,
        iso,
    })
}

fn format_rational(num: u32, denom: u32, show_decimal: bool) -> String {
    if denom == 0 {
        return format!("{num}/0");
    }
    if show_decimal {
        format!("{num}/{denom} ({:.2})", num as f64 / denom as f64)
    } else if num % denom == 0 {
        format!("{}", num / denom)
    } else {
        format!("{num}/{denom}")
    }
}

fn extract_tiff_data(data: &[u8]) -> Result<&[u8], String> {
    if data.len() < 4 || &data[0..2] != b"\xFF\xD8" {
        return Err("JPEG (SOI) ではありません".into());
    }

    let mut pos = 2;
    while pos + 1 < data.len() {
        if data[pos] != 0xFF {
            return Err("マーカー構造が不正です".into());
        }
        let marker = data[pos + 1];
        pos += 2;

        if marker == 0xD9 || marker == 0xDA {
            break;
        }
        if marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            continue;
        }

        if pos + 2 > data.len() {
            return Err("セグメント長が不足しています".into());
        }
        let len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
        if len < 2 || pos + len > data.len() {
            return Err("セグメント範囲が不正です".into());
        }

        let data_start = pos + 2;
        let data_end = pos + len;
        if marker == 0xE1 {
            let segment = &data[data_start..data_end];
            if segment.len() < 6 || &segment[0..6] != b"Exif\0\0" {
                return Err("APP1セグメントにExifヘッダーがありません".into());
            }
            return Ok(&segment[6..]);
        }

        pos += len;
    }

    Err("EXIF(APP1) セグメントが見つかりません".into())
}

const TAG_MAKE: u16 = 0x010F;
const TAG_MODEL: u16 = 0x0110;
const TAG_DATETIME: u16 = 0x0132;
const TAG_EXIF_IFD: u16 = 0x8769;
const TAG_DATETIME_ORIGINAL: u16 = 0x9003;
const TAG_EXPOSURE_TIME: u16 = 0x829A;
const TAG_FNUMBER: u16 = 0x829D;
const TAG_ISO_SPEED: u16 = 0x8827;

#[derive(Clone, Copy)]
enum ByteOrder {
    Little,
    Big,
}

impl ByteOrder {
    fn parse(data: &[u8]) -> Option<Self> {
        match data.get(0..2) {
            Some(b"II") => Some(ByteOrder::Little),
            Some(b"MM") => Some(ByteOrder::Big),
            _ => None,
        }
    }
}

fn read_u16(data: &[u8], offset: usize, order: ByteOrder) -> Result<u16, String> {
    if offset + 2 > data.len() {
        return Err("u16読み取り範囲外です".into());
    }
    let bytes = [data[offset], data[offset + 1]];
    Ok(match order {
        ByteOrder::Little => u16::from_le_bytes(bytes),
        ByteOrder::Big => u16::from_be_bytes(bytes),
    })
}

fn read_u32(data: &[u8], offset: usize, order: ByteOrder) -> Result<u32, String> {
    if offset + 4 > data.len() {
        return Err("u32読み取り範囲外です".into());
    }
    let bytes = [
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ];
    Ok(match order {
        ByteOrder::Little => u32::from_le_bytes(bytes),
        ByteOrder::Big => u32::from_be_bytes(bytes),
    })
}

#[derive(Clone)]
struct IfdEntry {
    tag: u16,
    field_type: u16,
    count: u32,
    value: Vec<u8>,
}

fn parse_ifd(
    data: &[u8],
    offset: usize,
    order: ByteOrder,
) -> Result<(Vec<IfdEntry>, Option<usize>), String> {
    if offset + 2 > data.len() {
        return Err("IFDのオフセットが不正です".into());
    }
    let entry_count = read_u16(data, offset, order)? as usize;
    let mut entries = Vec::with_capacity(entry_count);
    let mut cursor = offset + 2;

    for _ in 0..entry_count {
        if cursor + 12 > data.len() {
            return Err("IFDエントリが途中で途切れています".into());
        }
        let tag = read_u16(data, cursor, order)?;
        let field_type = read_u16(data, cursor + 2, order)?;
        let count = read_u32(data, cursor + 4, order)?;
        let value_offset = &data[cursor + 8..cursor + 12];

        let unit_size = type_unit_size(field_type)
            .ok_or_else(|| format!("未対応のフィールドタイプ: {field_type}"))?;
        let total_size = unit_size
            .checked_mul(count as usize)
            .ok_or("フィールドサイズが大きすぎます")?;

        let value = if total_size == 0 {
            Vec::new()
        } else if total_size <= 4 {
            value_offset[..total_size].to_vec()
        } else {
            let actual_offset = read_u32(data, cursor + 8, order)? as usize;
            if actual_offset + total_size > data.len() {
                return Err("IFDエントリのデータ領域が範囲外です".into());
            }
            data[actual_offset..actual_offset + total_size].to_vec()
        };

        entries.push(IfdEntry {
            tag,
            field_type,
            count,
            value,
        });

        cursor += 12;
    }

    let next_ifd = if cursor + 4 <= data.len() {
        Some(read_u32(data, cursor, order)? as usize)
    } else {
        None
    };

    Ok((entries, next_ifd))
}

fn type_unit_size(field_type: u16) -> Option<usize> {
    match field_type {
        1 | 2 | 6 | 7 => Some(1),
        3 | 8 => Some(2),
        4 | 9 => Some(4),
        5 | 10 => Some(8),
        11 => Some(4),
        12 => Some(8),
        _ => None,
    }
}

impl IfdEntry {
    fn as_ascii(&self) -> Option<String> {
        if self.field_type != 2 {
            return None;
        }
        let end = self
            .value
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.value.len());
        std::str::from_utf8(&self.value[..end])
            .ok()
            .map(|s| s.trim().to_string())
    }

    fn as_short(&self, order: ByteOrder) -> Option<Vec<u16>> {
        if self.field_type != 3 {
            return None;
        }
        let mut values = Vec::with_capacity(self.count as usize);
        for chunk in self.value.chunks_exact(2) {
            let bytes = [chunk[0], chunk[1]];
            let value = match order {
                ByteOrder::Little => u16::from_le_bytes(bytes),
                ByteOrder::Big => u16::from_be_bytes(bytes),
            };
            values.push(value);
        }
        Some(values)
    }

    fn as_rational(&self, order: ByteOrder) -> Option<Vec<(u32, u32)>> {
        if self.field_type != 5 {
            return None;
        }
        let mut values = Vec::with_capacity(self.count as usize);
        for chunk in self.value.chunks_exact(8) {
            let num = match order {
                ByteOrder::Little => u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
                ByteOrder::Big => u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
            };
            let denom = match order {
                ByteOrder::Little => u32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
                ByteOrder::Big => u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]),
            };
            values.push((num, denom));
        }
        Some(values)
    }

    fn as_u32(&self, order: ByteOrder) -> Option<u32> {
        if self.value.len() != 4 {
            return None;
        }
        let bytes = [self.value[0], self.value[1], self.value[2], self.value[3]];
        Some(match order {
            ByteOrder::Little => u32::from_le_bytes(bytes),
            ByteOrder::Big => u32::from_be_bytes(bytes),
        })
    }
}

fn find_entry<'a>(entries: &'a [IfdEntry], tag: u16) -> Option<&'a IfdEntry> {
    entries.iter().find(|entry| entry.tag == tag)
}
