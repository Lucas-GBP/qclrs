type ClMatrix = bool;
type Phase = usize;
type Qubit = usize;

pub enum MeasureReturns {
    RandomOne,
    RandomZero,
    AlwaysOne,
    AlwaysZero,
}
const PHASE_QUANT: Phase = 4;

const PHASE_STATES: [&str; PHASE_QUANT] = [" +", "+i", " -", "-i"];
/*
const PAULI_GATES: [[char; 2]; 2] = [
    ['I', 'Z'],
    ['X', 'Y'],
];
const MEASUREMENT_STATES: [&str; 4] = [
    "0",
    "1",
    "0 (random)",
    "1 (random)",
];
*/

pub struct CliffordSimulator {
    // Número de qubits
    n: Qubit,
    // Matriz G para a representação estabilizadora (2n x 2n matriz booleana)
    g: Vec<Vec<ClMatrix>>,
    // Vetor de fases associado às linhas de G
    f: Vec<Phase>,
}
impl CliffordSimulator {
    pub fn new(num_qubits: Qubit) -> Self {
        let total_size = 2 * num_qubits;
        let mut g = vec![vec![false; total_size]; total_size + 1]; // +1 para o buffer
        let f = vec![0; total_size + 1];

        // Inicializar G e F para o estado |0>^n
        for i in 0..num_qubits {
            g[i][i] = true; // X
            g[i + num_qubits][i + num_qubits] = true; // Z
        }

        Self {
            n: num_qubits,
            g,
            f,
        }
    }

    /// Retorna a quantidade de Qubit no sistema
    pub fn get_size(&self) -> Qubit {
        self.n
    }
    /// Retorna o tamanho total da matriz G e do vetor (2 * n)
    fn total_size(&self) -> Qubit {
        self.n * 2
    }
    /// Retorna o índice do buffer (2 * n)
    fn buffer_index(&self) -> Qubit {
        self.n * 2
    }
    /*
       Matrix Operations
    */
    /// Retorna o índice da coluna X do qubit q
    #[inline]
    fn x_index(&self, q: Qubit) -> Qubit {
        q
    }
    /// Retorna o índice da coluna Z do qubit q
    #[inline]
    fn z_index(&self, q: Qubit) -> Qubit {
        q + self.n
    }
    /// Retorna o índice da linha R dos desestabilizadores
    #[inline]
    fn d_index(&self, r: Qubit) -> Qubit {
        r
    }
    /// Retorna o índice da linha R dos estabilizadores
    #[inline]
    fn s_index(&self, r: Qubit) -> Qubit {
        r + self.n
    }
    /// Verifica se G[i][q] representa um operador X
    #[inline]
    fn is_x(&self, i: Qubit, q: Qubit) -> bool {
        self.g[i][self.x_index(q)] && !self.g[i][self.z_index(q)]
    }
    /// Verifica se G[i][q] representa um operador Y
    #[inline]
    fn is_y(&self, i: Qubit, q: Qubit) -> bool {
        self.g[i][self.x_index(q)] && self.g[i][self.z_index(q)]
    }
    /// Verifica se G[i][q] representa um operador Z
    #[inline]
    fn is_z(&self, i: Qubit, q: Qubit) -> bool {
        !self.g[i][self.x_index(q)] && self.g[i][self.z_index(q)]
    }
    #[inline]
    fn add_phase(&mut self, qubit: Qubit) {
        self.f[qubit] = (self.f[qubit] + PHASE_QUANT / 2) % PHASE_QUANT;
    }
    #[inline]
    fn swap_rows(&mut self, row1: Qubit, row2: Qubit) {
        self.copy_rows(self.buffer_index(), row2);
        self.copy_rows(row2, row1);
        self.copy_rows(row1, self.buffer_index());
    }
    fn copy_rows(&mut self, target: Qubit, control: Qubit) {
        for qubit in 0..self.n {
            let x_idx = self.x_index(qubit);
            let z_idx = self.z_index(qubit);

            self.g[target][x_idx] = self.g[control][x_idx];
            self.g[target][z_idx] = self.g[control][z_idx];
        }
        self.f[target] = self.f[control];
    }
    fn mult_row(&mut self, target_row: Qubit, control_row: Qubit) {
        // Ajusta a Fase
        let mut e: isize = 0; // expoente que i esta elevado
        for q in 0..self.n {
            if self.is_x(control_row, q) {
                if self.is_y(target_row, q) {
                    e += 1;
                } else if self.is_z(target_row, q) {
                    e -= 1;
                }
            } else if self.is_y(control_row, q) {
                if self.is_z(target_row, q) {
                    e += 1;
                } else if self.is_x(target_row, q) {
                    e -= 1;
                }
            } else if self.is_z(control_row, q) {
                if self.is_x(target_row, q) {
                    e += 1;
                } else if self.is_y(target_row, q) {
                    e -= 1;
                }
            }
        }

        let f = (self.f[target_row] + self.f[control_row]) as isize;
        let phase_quant = PHASE_QUANT as isize;
        e = (e + f) % phase_quant;
        if !(e >= 0) {
            e += phase_quant;
        }
        self.f[target_row] = e as Qubit;

        // Realiza a multiplicação
        for qubit in 0..self.n {
            let x_i = self.x_index(qubit);
            let z_i = self.z_index(qubit);
            self.g[target_row][x_i] ^= self.g[control_row][x_i];
            self.g[target_row][z_i] ^= self.g[control_row][z_i];
        }
    }
    fn set_row(&mut self, row: Qubit, obs: Qubit) {
        for i in 0..self.n {
            let x_idx = self.x_index(i);
            let z_idx = self.z_index(i);
            self.g[row][x_idx] = false;
            self.g[row][z_idx] = false;
        }
        self.f[row] = 0;

        self.g[row][obs] = true;
    }
    fn clean_buffer(&mut self) {
        let buffer_index = self.buffer_index();
        self.f[buffer_index] = 0;
        for i in 0..self.n {
            let x_idx = self.x_index(i);
            let z_idx = self.z_index(i);
            self.g[buffer_index][x_idx] = false;
            self.g[buffer_index][z_idx] = false;
        }
    }
    /*
       Primative Quantum Logic Gates
    */
    pub fn h(&mut self, qubit: Qubit) {
        let total_size = Self::total_size(&self);
        for i in 0..total_size {
            let tmp = self.g[i][self.x_index(qubit)];
            let x_idx = self.x_index(qubit);
            let z_idx = self.z_index(qubit);

            self.g[i][x_idx] = self.g[i][z_idx];
            self.g[i][z_idx] = tmp;

            if self.is_y(i, qubit) {
                self.add_phase(i);
            }
        }
    }
    pub fn s(&mut self, qubit: Qubit) {
        let total_size = Self::total_size(&self);
        for i in 0..total_size {
            if self.is_y(qubit, i) {
                self.add_phase(i);
            }

            let z_index = self.z_index(qubit);
            self.g[i][z_index] ^= self.g[i][self.x_index(qubit)];
        }
    }
    pub fn cnot(&mut self, control: Qubit, target: Qubit) {
        for i in 0..self.total_size() {
            let x_target = self.x_index(target);
            let z_target = self.z_index(target);
            let x_control = self.x_index(control);
            let z_control = self.z_index(control);

            self.g[i][x_target] ^= self.g[i][x_control];
            self.g[i][z_control] ^= self.g[i][z_target];

            if (self.g[i][x_control]
                && self.g[i][z_target]
                && self.g[i][x_target]
                && self.g[i][z_control])
                || (self.g[i][x_control]
                    && self.g[i][z_target]
                    && !self.g[i][x_target]
                    && !self.g[i][z_control])
            {
                self.add_phase(i); // Adiciona a fase se X e Z estiverem presentes (caso Y)
            }
        }
    }
    pub fn measure(&mut self, qubit: Qubit, suppress: bool) -> MeasureReturns {
        // TODO: Otimizar e criar opção para medir passivamente
        let mut s_pivot: Option<Qubit> = None;

        // Verifica se o qubit é indeterminado
        for i in 0..self.n {
            if self.g[self.s_index(i)][self.x_index(qubit)] {
                s_pivot = Some(i);
                break;
            }
        }

        if let Some(s_pivot) = s_pivot {
            self.copy_rows(self.d_index(s_pivot), self.s_index(s_pivot));
            self.set_row(self.s_index(s_pivot), self.s_index(qubit));

            let zi = self.z_index(s_pivot);
            self.f[zi] = (2 * (rand::random::<u64>() % 2)) as Phase;

            for i in 0..(2 * self.n) {
                if i != s_pivot && self.g[i][self.x_index(qubit)] {
                    self.mult_row(i, s_pivot);
                }
            }

            if self.f[self.z_index(s_pivot)] != 0 {
                return MeasureReturns::RandomOne;
            } else {
                return MeasureReturns::RandomZero;
            }
        }

        // Se não for indeterminado e a medição não for suprimida
        if !suppress {
            let mut d_pivot: Option<Qubit> = None;
            for i in 0..self.n {
                if self.g[i][self.x_index(qubit)] {
                    d_pivot = Some(i);
                    break;
                }
            }

            if let Some(d_pivot) = d_pivot {
                self.copy_rows(self.buffer_index(), d_pivot + self.n);
                for i in (d_pivot + 1)..self.n {
                    if self.g[i][self.x_index(qubit)] {
                        self.mult_row(self.buffer_index(), i + self.n);
                    }
                }

                if self.f[self.buffer_index()] != 0 {
                    return MeasureReturns::AlwaysOne;
                } else {
                    return MeasureReturns::AlwaysZero;
                }
            }
        }

        MeasureReturns::AlwaysZero
    }
    /*
       Emergent Quantum Logic Gates
    */
    pub fn z(&mut self, qubit: Qubit) {
        self.s(qubit); // Aplica S
        self.s(qubit); // Aplica S novamente
    }
    pub fn st(&mut self, qubit: Qubit) {
        self.z(qubit); // Aplica Z
        self.s(qubit); // Aplica S
    }
    pub fn x(&mut self, qubit: Qubit) {
        self.h(qubit);
        self.z(qubit);
        self.h(qubit);
    }
    pub fn y(&mut self, qubit: Qubit) {
        self.s(qubit);
        self.x(qubit);
        self.st(qubit);
    }
    pub fn minus_x(&mut self, qubit: Qubit) {
        self.z(qubit);
        self.x(qubit);
        self.z(qubit);
    }
    pub fn minus_y(&mut self, qubit: Qubit) {
        self.z(qubit);
        self.y(qubit);
        self.z(qubit);
    }
    /*
       Colapso do Sistema
    */
    fn gaussian(&mut self) -> Qubit {
        let mut i = self.n;
        let result: Qubit;

        for j in 0..self.n {
            for k in i..self.total_size() {
                if self.g[k][self.x_index(j)] {
                    self.swap_rows(i, k);
                    self.swap_rows(i - self.n, k - self.n);

                    for z in (i + 1)..self.total_size() {
                        if self.g[z][self.x_index(j)] {
                            self.mult_row(z, i);
                            self.mult_row(i - self.n, z - self.n);
                        }
                    }

                    i += 1;
                    break;
                }
            }
        }
        result = i - self.n;

        for j in 0..self.n {
            for k in i..self.total_size() {
                if self.g[k][self.z_index(j)] {
                    self.swap_rows(i, k);
                    self.swap_rows(i - self.n, k - self.n);

                    for z in (i + 1)..self.total_size() {
                        if self.g[z][self.z_index(j)] {
                            self.mult_row(z, i);
                            self.mult_row(i - self.n, z - self.n);
                        }
                    }

                    i += 1;
                    break;
                }
            }
        }

        return result;
    }
    fn seed(&mut self, gauss: Qubit) {
        //TODO: otimizar
        let mut min: Qubit = 0;
        let buffer_index = self.buffer_index();
        self.clean_buffer();

        let mut i = buffer_index - 1;
        while i >= self.n + gauss {
            let mut f = self.f[i];

            let mut j = self.n as isize - 1;
            while j >= 0 {
                let j_idx = j as usize;
                if self.g[i][self.z_index(j_idx)] {
                    min = j_idx;
                    if self.g[buffer_index][self.x_index(j_idx)] {
                        f = (f + (PHASE_QUANT - 2)) % PHASE_QUANT;
                    }
                }
                j -= 1;
            }

            if f == 2 {
                let x_idx = self.x_index(min);
                self.g[buffer_index][x_idx] = true;
            }

            i -= 1;
        }
    }
    /*
       String's
    */
    fn str_base_state(&self) -> String {
        let buffer_index = self.buffer_index();
        let mut e = self.f[buffer_index];
        let mut result = String::new();

        for i in 0..self.n {
            if self.is_y(buffer_index, i) {
                e = (e + 1) % 4;
            }
        }

        result.push_str(PHASE_STATES[e as usize]);
        result.push('|');

        for i in 0..self.n {
            let x_idx = self.x_index(i);
            let bit = self.g[buffer_index][x_idx];
            result.push(if bit { '1' } else { '0' });
        }

        result.push('>');
        result
    }
    pub fn str_ket(&mut self) -> String {
        let buffer_index = self.buffer_index();
        let gauss = self.gaussian();

        if gauss >= 64 {
            return "muitos estados para printar.".to_string();
        }

        let states_quant = 1u64 << gauss;
        let mut result = format!("\n{states_quant} possiveis estados\n");

        self.seed(gauss);
        result.push_str(&self.str_base_state());

        for i in 0..(states_quant - 1) {
            let i2 = i ^ (i + 1);
            for j in 0..gauss {
                if i2 & (1 << j) != 0 {
                    self.mult_row(buffer_index, self.n + j as usize);
                }
            }
            result.push_str(&self.str_base_state());
        }

        result
    }
}
