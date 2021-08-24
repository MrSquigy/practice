use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    // exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    println!("{} is a struct!", Structure(3));

    let num = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", num);
    println!("Debug: {:?}", num);
}
