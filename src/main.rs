mod qclrs;

fn main() {
    println!("Simulador de Circuitos Quânticos Clifford");
    let mut simulator = qclrs::CliffordSimulator::new(4);
    test(&mut simulator);
}

fn test(c: &mut qclrs::CliffordSimulator) {
    c.h(0);
    for i in 1..c.get_size() {
        c.cnot(0, i);        
    }
}