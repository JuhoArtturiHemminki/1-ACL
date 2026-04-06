use crate::fractal_feedback::{PHI, H_C};

pub struct AetherLink {
    pub charge_level: f64,
    pub reverse_flow_ma: f64,
}

impl AetherLink {
    pub const fn new() -> Self {
        Self {
            charge_level: 85.0,
            reverse_flow_ma: 0.0,
        }
    }

    pub fn process_transduction(&mut self, ambient_thermal: f64, drift: 
&mut f64) -> f64 {
        let energy_delta = ambient_thermal * PHI;
        if energy_delta > 0.1 {
            *drift += energy_delta / H_C;
            self.reverse_flow_ma = energy_delta * 100.0;
            self.charge_level = (self.charge_level + 0.00001).min(100.0);
        } else {
            *drift /= PHI;
            self.reverse_flow_ma = 0.0;
        }
        self.charge_level
    }
}

