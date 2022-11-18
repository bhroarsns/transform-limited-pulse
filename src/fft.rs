use super::complex::Complex;

pub trait IntoComplex {
    fn into_complex(self) -> Complex;
}

impl IntoComplex for f64 {
    fn into_complex(self) -> Complex {
        Complex::new_real(self)
    }
}

impl IntoComplex for Complex {
    fn into_complex(self) -> Complex {
        self
    }
}

#[allow(dead_code)]
pub fn fft<A, T>(input: &T) -> Vec<Complex>
where
    A: IntoComplex + Copy,
    T: IntoIterator<Item = A> + Clone
{

    let mut data: Vec<Complex> = input.clone().into_iter().map(|v| v.into_complex()).collect();
    let mut blocks = 1;
    let ex = ((data.len() as f64).log2() / 2.0).ceil() as u32;
    let length = 4_u64.pow(ex) as usize;

    data.resize(length, Complex::ZERO);
    let mut large_p = length / 4;

    while large_p > 0 {

        for j in 0..blocks {

            let shift = j * 4 * large_p;

            for p in 0..large_p {

                let w1 = data[shift + p] + data[shift + p + large_p] + data[shift + p + 2 * large_p] + data[shift + p + 3 * large_p];
                let w2 = data[shift + p] + data[shift + p + large_p].toggle_sign().prod_i() + data[shift + p + 2 * large_p].toggle_sign() + data[shift + p + 3 * large_p].prod_i();
                let w3 = data[shift + p] + data[shift + p + large_p].toggle_sign() + data[shift + p + 2 * large_p] + data[shift + p + 3 * large_p].toggle_sign();
                let w4 = data[shift + p] + data[shift + p + large_p].prod_i() + data[shift + p + 2 * large_p].toggle_sign() + data[shift + p + 3 * large_p].toggle_sign().prod_i();

                data[shift + p] = w1;
                data[shift + p + large_p] = w2 * Complex::exp_i_pi_theta(-2.0 * p as f64 / (large_p * 4) as f64);
                data[shift + p + 2 * large_p] = w3 * Complex::exp_i_pi_theta(-4.0 * p as f64 / (large_p * 4) as f64);
                data[shift + p + 3 * large_p] = w4 * Complex::exp_i_pi_theta(-6.0 * p as f64 / (large_p * 4) as f64);

            }

        }

        blocks *= 4;
        large_p /= 4;

    }

    let mut result: Vec<Complex> = vec![Complex::ZERO; length];
    let multp = 4_u64.pow(ex - 1);

    for i in 0..multp/4 {
        let mut org = i;
        let mut rev = 0;

        for k in 0..ex-2 {
            rev += 4_u64.pow(ex - 3 - k) * (org % 4);
            org /= 4;
        }

        for end in 0..4 {
            for start in 0..4 {
                result[(rev * 4 + start + end * multp) as usize] = data[(i * 4 + start * multp + end) as usize]
            }
        }
    }

    result

}