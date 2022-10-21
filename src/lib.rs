use byteorder::{BigEndian, LittleEndian, NativeEndian, WriteBytesExt};
use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal, NormalError};
use std::io::Write;

pub enum Endianess {
    Big,
    Little,
    Native,
}

impl Default for Endianess {
    fn default() -> Self {
        Endianess::Native
    }
}

pub fn get_random_f64(
    mean: f64,
    std_dev: f64,
    size: usize,
    seed: Option<u64>,
    endianess: Option<Endianess>,
) -> Result<Vec<u8>, NormalError> {
    let vec = get_normal_distribution_f64(mean, std_dev, size, seed)?;
    Ok(to_u8_f64(&vec, endianess.unwrap_or_default()))
}

pub fn get_normal_distribution_f64(
    mean: f64,
    std_dev: f64,
    size: usize,
    seed: Option<u64>,
) -> Result<Vec<f64>, NormalError> {
    let normal = Normal::new(mean, std_dev)?;
    let values: Vec<f64> = match seed {
        Some(s) => {
            let mut rng = ChaCha8Rng::seed_from_u64(s);
            std::iter::repeat(0f64)
                .take(size)
                .map(|_| normal.sample(&mut rng))
                .collect()
        }
        None => {
            let mut rng = thread_rng();
            std::iter::repeat(0f64)
                .take(size)
                .map(|_| normal.sample(&mut rng))
                .collect()
        }
    };

    Ok(values)
}

pub fn to_u8_f64(data: &[f64], endianess: Endianess) -> Vec<u8> {
    let mut wtr = Vec::new();
    match endianess {
        Endianess::Big => {
            for val in data.iter() {
                wtr.write_f64::<BigEndian>(*val).unwrap();
            }
            wtr
        }
        Endianess::Little => {
            for val in data.iter() {
                wtr.write_f64::<LittleEndian>(*val).unwrap();
            }
            wtr
        }
        Endianess::Native => {
            for val in data.iter() {
                wtr.write_f64::<NativeEndian>(*val).unwrap();
            }
            wtr
        }
    }
}

pub fn write_to_disk_f64<W: Write>(
    values: &[f64],
    path: &mut W,
    endianess: Endianess,
) -> std::io::Result<()> {
    let data = to_u8_f64(values, endianess);
    path.write_all(&data)?;
    Ok(())
}

pub fn get_normal_distribution_f32(
    mean: f32,
    std_dev: f32,
    size: usize,
    seed: Option<u64>,
) -> Result<Vec<f32>, NormalError> {
    let normal = Normal::new(mean, std_dev)?;
    let values: Vec<f32> = match seed {
        Some(s) => {
            let mut rng = ChaCha8Rng::seed_from_u64(s);
            std::iter::repeat(0f32)
                .take(size)
                .map(|_| normal.sample(&mut rng))
                .collect()
        }
        None => {
            let mut rng = thread_rng();
            std::iter::repeat(0f32)
                .take(size)
                .map(|_| normal.sample(&mut rng))
                .collect()
        }
    };

    Ok(values)
}

pub fn to_u8_f32(data: &[f32], endianess: Endianess) -> Vec<u8> {
    let mut wtr = Vec::new();
    match endianess {
        Endianess::Big => {
            for val in data.iter() {
                wtr.write_f32::<BigEndian>(*val).unwrap();
            }
            wtr
        }
        Endianess::Little => {
            for val in data.iter() {
                wtr.write_f32::<LittleEndian>(*val).unwrap();
            }
            wtr
        }
        Endianess::Native => {
            for val in data.iter() {
                wtr.write_f32::<NativeEndian>(*val).unwrap();
            }
            wtr
        }
    }
}

pub fn write_to_disk_f32<W: Write>(
    values: &[f32],
    path: &mut W,
    endianess: Endianess,
) -> std::io::Result<()> {
    let data = to_u8_f32(values, endianess);
    path.write_all(&data)?;
    Ok(())
}
