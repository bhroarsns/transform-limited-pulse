use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::env;

mod complex;
use complex::Complex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let timestep = (&args[1]).trim().parse::<f64>()?;
    let n_start = (&args[2]).trim().parse::<i32>()?;
    let n_end = (&args[3]).trim().parse::<i32>()?;
    let input_filename = &args[4];
    let output_filename = &args[5];
    
    let mut f = BufReader::new(File::open(input_filename)?);
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    
    const SPEED_OF_LIGHT: f64 = 3.0e8; // in m/s
    let convert_wavelength_to_frequency = |wavelength: &f64| -> f64 {
        SPEED_OF_LIGHT / (wavelength * 1.0e-9) / 1.0e15
    };

    let data = data.split('\n').flat_map(|line| {
        if line.starts_with("#") {
            None
        } else {
            line.split_whitespace().take_while(|strip| !strip.starts_with("#")).map(|strip| strip.trim().parse::<f64>()).collect::<Result<Vec<f64>, _>>().ok().and_then(|vec| {
                if vec.len() < 4 + 1 {
                    None
                } else {
                    Some((convert_wavelength_to_frequency(&vec[1]), vec[4]))
                }
            })
        }
    });

    let background = {
        let (a, b) = data.clone().filter(|(wl, _)| wl < &900_f64).fold((0.0, 0), |(sum, count), (_, power)| (sum + power, count + 1));
        a / b as f64
    };

    let data: Vec<(f64, f64)> = data.map(|(freq, power)| (freq, power - background)).collect();
    // let data: Vec<(f64, f64)> = data.map(|(freq, _)| (freq, (-(freq - 0.3)*(freq - 0.3) / 1.0 / 1.0).exp())).collect();

    let mut out = BufWriter::new(File::create(output_filename)?);

    let max = timestep * n_end.abs().max(n_start.abs()) as f64;
    let max = max.log10().floor() as usize + 1 + timestep.log10().floor().abs() as usize + 2;

    // const PI: f64 = std::f64::consts::PI;
    for i_t in n_start..n_end {
        let time = timestep * i_t as f64;

        let (integral, _) = data.iter().fold(
            (Complex::ZERO, (data[0].0, data[0].1)),
            |(result, (prev_freq, prev_power)), (freq, power)| {
            (
                result + (prev_freq - freq) * (Complex::new_polar(prev_power, prev_freq * time) + Complex::new_polar(*power, freq * time)) / 2.0,
                (
                    *freq,
                    *power
                )
            )
        });
    
        writeln!(out,
            "{:>+0max$.*} {:+.20} {:+.20} {:+.20}",
            timestep.log10().floor().abs() as usize,
            time,
            integral.abs(),
            integral.re(),
            integral.im(),
        )?;
    }

    Ok(())
}
