use ndarray::{Array1, Array2};
use num_complex::Complex;

const INV_SQRT_2: f64 = 0.7071067811865476; // 1 / √2

fn main() {
    // Estado inicial |00⟩ → [1, 0, 0, 0]
    let mut state: Array1<Complex<f64>> = Array1::from_vec(vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ]);

    // Puerta Hadamard en el primer qubit: H ⊗ I
    let h_i: Array2<Complex<f64>> = Array2::from_shape_vec(
        (4, 4),
        vec![
            Complex::new(INV_SQRT_2, 0.0), Complex::new(0.0, 0.0), Complex::new(INV_SQRT_2, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(INV_SQRT_2, 0.0), Complex::new(0.0, 0.0), Complex::new(INV_SQRT_2, 0.0),
            Complex::new(INV_SQRT_2, 0.0), Complex::new(0.0, 0.0), Complex::new(-INV_SQRT_2, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(INV_SQRT_2, 0.0), Complex::new(0.0, 0.0), Complex::new(-INV_SQRT_2, 0.0),
        ],
    ).expect("Error al crear H ⊗ I");

    state = h_i.dot(&state);

    // Puerta CNOT (control = qubit 0, target = qubit 1)
    let cnot: Array2<Complex<f64>> = Array2::from_shape_vec(
        (4, 4),
        vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ],
    ).expect("Error al crear CNOT");

    state = cnot.dot(&state);

    // Mostrar el estado cuántico resultante
    println!("✅ Estado de Bell |Φ⁺⟩ = 1/√2 (|00⟩ + |11⟩)");
    println!("\nAmplitudes:");
    for (i, amplitude) in state.iter().enumerate() {
        let basis = format!("{:02b}", i);
        println!("|{}⟩: {:.6} + {:.6}i", basis, amplitude.re, amplitude.im);
    }

    println!("\nProbabilidades:");
    for (i, amplitude) in state.iter().enumerate() {
        let prob = amplitude.norm_sqr(); // |a|² + |b|²
        if prob > 1e-10 {
            let basis = format!("{:02b}", i);
            println!("P(|{}⟩) = {:.2}%", basis, prob * 100.0);
        }
    }
}

//fn main() {
  //  println!("Hello, world!");
//}
