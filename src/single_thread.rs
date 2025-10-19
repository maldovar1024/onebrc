use ahash::RandomState;
use memmap2::Mmap;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write as _},
    simd::{cmp::SimdPartialEq, u8x64},
};

struct Item {
    min: i64,
    max: i64,
    sum: i64,
    total: u64,
}

#[inline]
fn fold_num(a: i64, &b: &u8) -> i64 {
    if b == b'.' {
        a
    } else {
        a * 10 + (b - b'0') as i64
    }
}

const WIDTH: usize = 64;

#[inline]
fn find_char(buf: &[u8], start: usize, c: u8) -> Option<usize> {
    let test = u8x64::splat(c);

    let mut i = start;

    while i + WIDTH <= buf.len() {
        if let Some(target_pos) = u8x64::from_slice(&buf[i..]).simd_eq(test).first_set() {
            return Some(i + target_pos);
        }

        i += WIDTH;
    }

    buf[i..].iter().position(|&x| x == c).map(|x| x + i)
}

pub fn run_single_thread(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let mmap = unsafe { Mmap::map(&File::open(input_file)?)? };
    let buf = mmap.as_ref();

    let mut map = HashMap::<Box<[u8]>, Item, RandomState>::default();

    let mut start = 0;

    while start < buf.len() {
        let colon_pos = find_char(buf, start, b';').unwrap();
        let next_end = find_char(buf, colon_pos + 1, b'\n').unwrap_or(buf.len());

        let city = &buf[start..colon_pos];
        let t = &buf[colon_pos + 1..next_end];
        let temperature = if t[0] == b'-' {
            -t[1..].iter().fold(0, fold_num)
        } else {
            t.iter().fold(0, fold_num)
        };

        match map.get_mut(city) {
            Some(item) => {
                item.min = item.min.min(temperature);
                item.max = item.max.max(temperature);
                item.sum += temperature;
                item.total += 1;
            }
            None => {
                map.insert(
                    Box::from(city),
                    Item {
                        min: temperature,
                        max: temperature,
                        sum: temperature,
                        total: 1,
                    },
                );
            }
        }

        start = next_end + 1;
    }

    let mut v = Vec::from_iter(map);
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut output = BufWriter::new(File::create(output_file)?);

    write!(output, "{{")?;

    for (
        i,
        (
            city,
            Item {
                min,
                max,
                sum,
                total,
            },
        ),
    ) in v.into_iter().enumerate()
    {
        write!(
            output,
            "{}{}={:.1}/{:.1}/{:.1}",
            if i > 0 { ", " } else { "" },
            unsafe { String::from_utf8_unchecked(city.to_vec()) },
            min as f64 / 10.,
            sum as f64 / total as f64 / 10.,
            max as f64 / 10.,
        )?;
    }

    write!(output, "}}")?;

    Ok(())
}
