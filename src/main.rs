fn main() {
    // Initialize data storage based on n_f
    let mut table: [Vec<TableRow>; 4] = [vec![], vec![], vec![], vec![]];

    // Gather data
    for i in 1..=5 {
        for j in (i+1)..=5 {
            if j != i {
                let index: usize = i as usize - 1;
                table[index].push(calc_all(j, i));
            }
        }
    }

    // Display data
    for group in table {
        for row in group {
            println!("{:?}", row);
        }
    }
}

// Store row data
#[derive(Debug)]
struct TableRow {
    n_i: u32,
    n_f: u32,
    wavelength: f32,
    color: &'static str
}

// Run all calculations and return data
fn calc_all(n_i: u32, n_f: u32) -> TableRow {
    let energy = calculate_energy(n_i, n_f);
    let wavelength = calculate_wavelength(energy);
    let color = calculate_color(wavelength);
    return TableRow {
        n_i,
        n_f,
        wavelength,
        color
    };
}

// Get color from wavelength
fn calculate_color(wavelength: f32) -> &'static str {
    return match wavelength {
        0.0..=400.0 => {
            "UV"
        },
        400.0..=420.0 => {
            "Violet"
        },
        420.0..=440.0 => {
            "Indigo"
        },
        440.0..=490.0 => {
            "Blue"
        },
        490.0..=570.0 => {
            "Green"
        },
        570.0..=585.0 => {
            "Yellow"
        },
        585.0..=620.0 => {
            "Orange"
        },
        620.0..=780.0 => {
            "Red"
        },
        _ => {
            "IR"
        }
    }
}

// Returns wavelength in nm
fn calculate_wavelength(energy: f32) -> f32 {
    let c: f32 = 2.99792458e8; // m / s
    let h: f32 = 6.62606893e-34; // J * s
    let wavelength = (h * c) / energy; // m
    let nm = (wavelength * 1e9);
    return nm;
}

// Returns energy in J
fn calculate_energy(n_i: u32, n_f: u32) -> f32 {
    // Convert eV to J
    let ground_state = 13.6 * 1.60217653e-19; // J
    let mut n_2 = 0;
    let mut n_1 = 0;
    if n_i > n_f {
        n_2 = n_i;
        n_1 = n_f;
    } else {
        n_2 = n_f;
        n_1 = n_i;
    }
    let energy_change = ground_state * ( (1.0 / n_1.pow(2) as f32) - (1.0 / n_2.pow(2) as f32) ); // J
    return energy_change;
}

