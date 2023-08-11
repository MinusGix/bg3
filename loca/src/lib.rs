use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek},
};

use binrw::{io::TakeSeekExt, meta::ReadEndian, BinRead, BinWrite, NullString, VecArgs};

pub fn parse_loca(data: &[u8]) -> Result<Loca, binrw::Error> {
    let mut data = Cursor::new(data);
    Loca::read(&mut data)
}

// TODO: Write implementation.

// TODO: Can we make this faster? I expect part of the issue is that we're doing this sequentially.
// It depends on how long the various parts take. If parsing the uninit loca entries takes any notable amount of time (there are a bunch) then we could potentially speed this up by parsing the uninit entries in parallel since they're constant size.
// However the string allocations that are done for nullstring, I expect are the actual root cause.
// We could specialize for lifetimes when we have a borrowed data, which might make it nicer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loca {
    // We don't store the header because we can infer num entries from 'entries'
    // and also infer the texts offsets from the various values
    pub entries: Vec<LocaEntry>,
}
impl Loca {
    // TODO: should we require the version?
    pub fn get(&self, key: Key) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.key == key)
            .map(|e| e.value.as_str())
    }

    /// Takes a hex guid in the form of
    /// `58eaf4f1-fe6c-fdb0-e3cc-13fdeebdcb3e` and returns the string associated with it.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        let key = key.as_bytes();

        self.entries
            .iter()
            .find(|e| e.key.nz_slice() == key)
            .map(|e| e.value.as_str())
        // let key = key.strip_prefix('h').unwrap_or(key);

        // println!("Key: {key}");
        // // TODO: don't allocate a new string for it!
        // let key = key.replace("-", "");
        // println!("KeyRepl: {key}");
        // let mut key_out = [0; 18];
        // if let Err(e) = hex::decode_to_slice(key, &mut key_out) {
        //     eprintln!("Failed to decode: {e:?}");
        //     return None;
        // }
        // println!("KeyOut: {key_out:?}");
        // // We have to extend key_out to 64 bytes
        // let mut key = Key([0; 64]);
        // key.0[..key_out.len()].copy_from_slice(&key_out);
        // println!("Key2: {key:?}");

        // self.get_key(key)
    }
}
impl ReadEndian for Loca {
    const ENDIAN: binrw::meta::EndianKind = binrw::meta::EndianKind::Endian(binrw::Endian::Little);
}
impl BinRead for Loca {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: binrw::Endian,
        _: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let header = Header::read(reader)?;
        let entries: Vec<UninitLocaEntry> = BinRead::read_args(
            reader,
            VecArgs {
                count: header.num_entries as usize,
                inner: (),
            },
        )?;

        reader.seek(std::io::SeekFrom::Start(header.texts_offset.into()))?;
        let entries = entries
            .into_iter()
            .map(|e| {
                let mut data = reader.take_seek(e.length.into());

                let value = NullString::read(&mut data)?;
                let value = String::try_from(value).expect("Failed to convert string to utf8");

                Ok(LocaEntry {
                    key: e.key,
                    version: e.version,
                    value,
                })
            })
            .collect::<Result<Vec<_>, binrw::Error>>()?;

        Ok(Loca { entries })
    }
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little, magic = b"LOCA")]
struct Header {
    pub num_entries: u32,
    pub texts_offset: u32,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(little)]
struct UninitLocaEntry {
    pub key: Key,
    pub version: u16,
    pub length: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaEntry {
    pub key: Key,
    pub version: u16,
    // No length field because we assume we can infer that from the value
    pub value: String,
}

#[derive(Clone, PartialEq, Eq, BinRead, BinWrite)]
pub struct Key(pub [u8; 64]);
impl Key {
    /// Get the slice of the key that doesn't include null bytes
    pub fn nz_slice(&self) -> &[u8] {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(self.0.len());
        &self.0[..len]
    }

    /// NOTE: The key can include null bytes, which this ignores. I expect those aren't in the
    /// actual key and so don't matter?
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        // The key can include null bytes so we don't want to just include them
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(self.0.len());
        std::str::from_utf8(&self.0[..len])
    }
}
impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_str() {
            Ok(s) => write!(f, "\"{}\"", s),
            Err(_) => write!(f, "{:?}", self.0),
        }
    }
}
