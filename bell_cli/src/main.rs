use num_complex::Complex;
use serde::Serialize;
use std::f64::consts::FRAC_1_SQRT_2;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Resultado {
    amplitudes: Vec<(String, f64, f64)>, // (basis, re, im)
    probabilidades: Vec<(String, f64)>,  // (basis, prob)
}

fn main() {
    // Estado inicial |00⟩
    let mut state = [
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];

    // H ⊗ I
    let h_i = [
        [FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2, 0.0],
        [0.0, FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2],
        [FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2, 0.0],
        [0.0, FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2],
    ];

    let mut new_state = [Complex::new(0.0, 0.0); 4];
    for i in 0..4 {
        for j in 0..4 {
            new_state[i] += Complex::new(h_i[i][j], 0.0) * state[j];
        }
    }
    state = new_state;

    // CNOT
    let cnot = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    let mut new_state = [Complex::new(0.0, 0.0); 4];
    for i in 0..4 {
        for j in 0..4 {
            new_state[i] += Complex::new(cnot[i][j], 0.0) * state[j];
        }
    }
    state = new_state;

    // Preparar resultados
    let mut amplitudes = Vec::new();
    let mut probabilidades = Vec::new();

    for (i, amp) in state.iter().enumerate() {
        let basis = format!("{:02b}", i);
        amplitudes.push((basis.clone(), amp.re, amp.im));
        let prob = amp.norm_sqr();
        if prob > 1e-10 {
            probabilidades.push((basis, prob));
        }
    }

    let resultado = Resultado {
        amplitudes,
        probabilidades,
    };

    // Guardar en JSON
    let json = serde_json::to_string_pretty(&resultado).unwrap();
    let mut file = File::create("bell_result.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("✅ Resultados guardados en bell_result.json");
}
