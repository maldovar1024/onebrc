use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write as _},
};

struct Item {
    min: f64,
    max: f64,
    sum: f64,
    total: u64,
}

pub fn run_baseline(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let buf = BufReader::new(File::open(input_file)?);

    let mut map = HashMap::<String, Item>::new();

    for line in buf.lines() {
        let line = line?;
        let mut row = line.split(';');
        let city = row.next().unwrap();
        let temperature: f64 = row.next().unwrap().parse().unwrap();

        match map.get_mut(city) {
            Some(item) => {
                item.min = item.min.min(temperature);
                item.max = item.max.max(temperature);
                item.sum += temperature;
                item.total += 1;
            }
            None => {
                map.insert(
                    city.to_string(),
                    Item {
                        min: temperature,
                        max: temperature,
                        sum: temperature,
                        total: 1,
                    },
                );
            }
        }
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
            "{}{city}={min:.1}/{:.1}/{max:.1}",
            if i > 0 { ", " } else { "" },
            sum / total as f64
        )?;
    }

    write!(output, "}}")?;

    Ok(())
}
