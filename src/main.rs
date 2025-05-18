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

    c.s(0);
    c.z(0);
    c.st(0);
    c.x(0);
    c.y(0);
    c.minus_x(0);
    c.minus_y(0);
    for i in 0..c.get_size() {
        c.measure(i, false);        
    }

}