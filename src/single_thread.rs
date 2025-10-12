use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write as _},
};
use ahash::RandomState;

struct Item {
    min: i64,
    max: i64,
    sum: i64,
    total: u64,
}

fn fold_num(a: i64, &b: &u8) -> i64 {
    if b == b'.' {
        a
    } else {
        a * 10 + (b - b'0') as i64
    }
}

pub fn run_single_thread(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let mut reader = BufReader::new(File::open(input_file)?);
    let mut buf = Vec::with_capacity(110);

    let mut map = HashMap::<Box<[u8]>, Item, RandomState>::default();

    loop {
        reader.read_until(b'\n', &mut buf)?;
        if buf.last().is_some_and(|&b| b == b'\n') {
            buf.pop();
        }
        if buf.is_empty() {
            break;
        }

        let splitter= buf.iter().position(|&x| x == b';').unwrap();

        let city = &buf[0..splitter];

        let t = &buf[splitter + 1..];
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

        buf.clear();
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
