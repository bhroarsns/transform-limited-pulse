use std::fs::File;
use std::io::{BufReader, Read};

mod fft;
mod complex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = BufReader::new(File::open("spectrum.dat")?);
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    
    let (_wavelength, power): (Vec<f64>, Vec<f64>) = data.split('\n').flat_map(|line| {
        line.split(',').map(|strip| strip.trim().parse::<f64>()).take(2).collect::<Result<Vec<f64>, _>>().map_or(
            None,
            |vec| {
                if vec.len() < 2 {
                    None
                } else {
                    Some((vec[0], vec[1]))
                }
            }
        )
    }).unzip();

    let fpower = fft::fft(&power);

    println!("{:?}", fpower);

    Ok(())
}
