pub mod qclrs;

fn main() {
    println!("Simulador de Circuitos Quânticos Clifford");
    let mut simulator = qclrs::CliffordSimulator::new(32);
    test(&mut simulator);

    let r = simulator.str_ket();
    println!("{r}");
}

fn test(c: &mut qclrs::CliffordSimulator) {
    c.h(0);
    for i in 1..c.get_size() {
        c.cnot(0, i);
    }
}
