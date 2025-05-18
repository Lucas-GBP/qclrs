type ClMatrix = bool;
type Phase = usize;
type Qubit = usize;

/// Retorna o índice da coluna X do qubit q
#[inline]
fn x_index(s: &CliffordSimulator, q: Qubit) -> Qubit {
    let _ = s;
    q
}
/// Retorna o índice da coluna Z do qubit q
#[inline]
fn z_index(s: &CliffordSimulator, q: Qubit) -> Qubit {
    q+s.n
}
/// Retorna o índice da linha R dos desestabilizadores
#[inline]
fn d_index(s: &CliffordSimulator, r: Qubit) -> Qubit {
    let _ = s;
    r
}
/// Retorna o índice da linha R dos estabilizadores
#[inline]
fn s_index(s: &CliffordSimulator, r: Qubit) -> Qubit {
    r+s.n
}
/// Verifica se G[i][q] representa um operador X
#[inline]
fn is_x(s: &CliffordSimulator, i: Qubit, q: Qubit) -> bool {
    s.g[i][x_index(s, q)] && !s.g[i][z_index(s, q)]
}
/// Verifica se G[i][q] representa um operador Y
#[inline]
fn is_y(s: &CliffordSimulator, i: Qubit, q: Qubit) -> bool {
    s.g[i][x_index(s, q)] && s.g[i][z_index(s, q)]
}
/// Verifica se G[i][q] representa um operador Z
#[inline]
fn is_z(s: &CliffordSimulator, i: Qubit, q: Qubit) -> bool {
    !s.g[i][x_index(s, q)] && s.g[i][z_index(s, q)]
}

enum MeasureReturns {
    RandomOne,
    RandomZero,
    AlwaysOne,
    AlwaysZero,
}

const PHASE_QUANT:Phase = 4;
const Q_FALSE:Qubit = 0;
const Q_TRUE:Qubit = 1;

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

        Self { n: num_qubits, g, f }
    }

    pub fn get_size(&self) -> Qubit {
        self.n
    }
    /// Retorna o tamanho total da matriz G e do vetor (2 * n)
    fn total_size(&self) -> Qubit {
        self.n*2
    }
    /// Retorna o índice do buffer (2 * n)
    fn buffer_index(&self) -> Qubit {
        self.n*2
    }

    fn add_phase(&mut self, qubit: Qubit) {
        self.f[qubit] = ((self.f[qubit]) + PHASE_QUANT / 2) % PHASE_QUANT;
    }
    fn copy_rows(&mut self, target: Qubit, control: Qubit) {
        for qubit in 0..self.n {
            let x_idx = x_index(&self, qubit);
            let z_idx = z_index(&self, qubit);

            self.g[target][x_idx] = self.g[control][x_idx];
            self.g[target][z_idx] = self.g[control][z_idx];
        }
        self.f[target] = self.f[control];
    }
    fn swap_rows(&mut self, row1: Qubit, row2: Qubit) {
        let buffer_index = self.buffer_index();
        self.copy_rows(buffer_index, row2);
        self.copy_rows(row2, row1);
        self.copy_rows(row1, buffer_index);
    }
    fn mult_row(&mut self, target_row: Qubit, control_row: Qubit) {
        // Ajusta a Fase
        let mut e: Qubit = 0;// expoente que i esta elevado
        for q in 0..self.n {
            if is_x(&self, control_row, q){
                if is_y(&self, target_row, q) {
                    e += 1;
                } else 
                if is_z(&self, target_row, q) {
                    e -= 1;
                }
            } else 
            if is_y(&self, control_row, q) {
                if is_z(&self, target_row, q) {
                    e += 1;
                } else 
                if is_x(&self, target_row, q) {
                    e -= 1;
                }
            } else 
            if is_z(&self, control_row, q) {
                if is_x(&self, target_row, q) {
                    e += 1;
                } else 
                if is_y(&self, target_row, q) {
                    e -= 1;
                }
            }
        }
    
        e = (e+self.f[target_row]+self.f[control_row])%PHASE_QUANT;
        if !(e >= 0) {
            e += PHASE_QUANT;
        }
        self.f[target_row] = e;
    
        // Realiza a multiplicação
        for qubit in 0..self.n {
            let x_i = x_index(&self, qubit);
            let z_i = z_index(&self, qubit);
            self.g[target_row][x_i] ^= self.g[control_row][x_i];
            self.g[target_row][z_i] ^= self.g[control_row][z_i];
        }
    }
    fn set_row(&mut self, row: Qubit, obs: Qubit) {
        for i in 0..self.n {
            let x_idx = x_index(&self,i);
            let z_idx = z_index(&self,i);
            self.g[row][x_idx] = false;
            self.g[row][z_idx] = false;
        }
        self.f[row] = 0;
    
        self.g[row][obs] = true;
    }

    pub fn h(&mut self, qubit: Qubit) {
        let total_size = Self::total_size(&self);
        for i in 0..total_size {
            let tmp = self.g[i][x_index(&self, qubit)];
            let x_idx = x_index(&self, qubit);
            let z_idx = z_index(&self, qubit);

            self.g[i][x_idx] = self.g[i][z_idx];
            self.g[i][z_idx] = tmp;

            if is_y(&self, qubit, i) {
                self.add_phase(i);
            }
        }
    }
    pub fn s(&mut self, qubit: Qubit) {
        let total_size = Self::total_size(&self);
        for i in 0..total_size {
            if is_y(&self, qubit, i) {
                self.add_phase(i);
            }

            let z_index = z_index(&self, qubit);
            self.g[i][z_index] ^=
                self.g[i][x_index(&self, qubit)];
        }
    }
    pub fn cnot(&mut self, control: Qubit, target: Qubit) {
        for i in 0..self.total_size() {
            let x_target: usize = x_index(&self, target);
            let z_target = z_index(&self, target);
            let x_control = x_index(&self, control);
            let z_control = z_index(&self, control);

            self.g[i][x_target] ^= self.g[i][x_control];
            self.g[i][z_control] ^= self.g[i][z_target];

            if (
                self.g[i][x_control] && self.g[i][z_target] &&
                self.g[i][x_target] && self.g[i][z_control]
                ) || (
                self.g[i][x_control] && self.g[i][z_target] &&
                !self.g[i][x_target] && !self.g[i][z_control]
            ) {
                self.add_phase(i); // Adiciona a fase se X e Z estiverem presentes (caso Y)
            }

        }
    }
    pub fn measure(&mut self, qubit: Qubit, suppress: bool) -> MeasureReturns {
        // TODO: Otimizar e criar opção para medir passivamente
        let mut s_pivot: Option<Qubit> = None;
    
        // Verifica se o qubit é indeterminado
        for i in 0..self.n {
            if self.g[s_index(&self, i)][x_index(&self, qubit)] {
                s_pivot = Some(i);
                break;
            }
        }
    
        if let Some(s_pivot) = s_pivot {
            self.copy_rows(d_index(&self, s_pivot), s_index(&self, s_pivot));
            self.set_row(s_index(&self, s_pivot), s_index(&self, qubit));
    
            let zi = z_index(&self, s_pivot);
            self.f[zi] = (2 * (rand::random::<u64>() % 2)) as Phase;
    
            for i in 0..(2 * self.n) {
                if i != s_pivot && self.g[i][x_index(&self, qubit)] {
                    self.mult_row(i, s_pivot);
                }
            }
    
            if self.f[z_index(&self, s_pivot)] != 0 {
                return MeasureReturns::RandomOne;
            } else {
                return MeasureReturns::RandomZero;
            }
        }
    
        // Se não for indeterminado e a medição não for suprimida
        if !suppress {
            let mut d_pivot: Option<Qubit> = None;
            for i in 0..self.n {
                if self.g[i][x_index(&self, qubit)] {
                    d_pivot = Some(i);
                    break;
                }
            }
    
            if let Some(d_pivot) = d_pivot {
                self.copy_rows(self.buffer_index(), d_pivot + self.n);
                for i in (d_pivot + 1)..self.n {
                    if self.g[i][x_index(&self, qubit)] {
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
    pub fn y(&mut self, qubit: Qubit){
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



}
