use crate::circuit::config::Config;

use super::Circuit;

pub struct Stats {
    // number of layers in the final circuit
    pub num_layers: usize,
    // number of segments
    pub num_segments: usize,
    // number of used input variables
    pub num_inputs: usize,
    // number of mul/add/cst gates in all circuits (unexpanded)
    pub num_total_mul: usize,
    pub num_total_add: usize,
    pub num_total_cst: usize,
    // number of mul/add/cst gates in expanded form of all layers
    pub num_expanded_mul: usize,
    pub num_expanded_add: usize,
    pub num_expanded_cst: usize,
    // number of total gates in the final circuit (except input gates)
    pub num_total_gates: usize,
    // number of actually used gates used in the final circuit
    pub num_used_gates: usize,
}

struct CircuitStats {
    num_expanded_mul: usize,
    num_expanded_add: usize,
    num_expanded_cst: usize,
}

impl<C: Config> Circuit<C> {
    pub fn get_stats(&self) -> Stats {
        let mut m: Vec<CircuitStats> = Vec::with_capacity(self.segments.len());
        let mut ar = Stats {
            num_layers: 0,
            num_segments: 0,
            num_inputs: 0,
            num_total_mul: 0,
            num_total_add: 0,
            num_total_cst: 0,
            num_expanded_mul: 0,
            num_expanded_add: 0,
            num_expanded_cst: 0,
            num_total_gates: 0,
            num_used_gates: 0,
        };
        for i in 0..self.segments.len() {
            let num_self_mul = self.segments[i].gate_muls.len();
            let num_self_add = self.segments[i].gate_adds.len();
            let num_self_cst = self.segments[i].gate_consts.len();
            let mut r = CircuitStats {
                num_expanded_mul: num_self_mul,
                num_expanded_add: num_self_add,
                num_expanded_cst: num_self_cst,
            };
            for (sub_id, allocs) in self.segments[i].child_segs.iter() {
                r.num_expanded_mul += m[*sub_id].num_expanded_mul * allocs.len();
                r.num_expanded_add += m[*sub_id].num_expanded_add * allocs.len();
                r.num_expanded_cst += m[*sub_id].num_expanded_cst * allocs.len();
            }
            ar.num_total_mul += num_self_mul;
            ar.num_total_add += num_self_add;
            ar.num_total_cst += num_self_cst;
            m.push(r);
        }
        for x in self.layer_ids.iter() {
            ar.num_expanded_mul += m[*x].num_expanded_mul;
            ar.num_expanded_add += m[*x].num_expanded_add;
            ar.num_expanded_cst += m[*x].num_expanded_cst;
        }
        ar.num_segments = self.segments.len();
        ar.num_layers = self.layer_ids.len();
        let (input_mask, output_mask) = self.compute_masks();
        for i in 0..self.layer_ids.len() {
            ar.num_total_gates += self.segments[self.layer_ids[i]].num_outputs;
            for j in 0..self.segments[self.layer_ids[i]].num_outputs {
                if output_mask[self.layer_ids[i]][j] {
                    ar.num_used_gates += 1;
                }
            }
        }
        for i in 0..self.segments[self.layer_ids[0]].num_inputs {
            if input_mask[self.layer_ids[0]][i] {
                ar.num_inputs += 1;
            }
        }
        ar
    }
}
