use super::*;

#[allow(dead_code)]
pub struct TabuSearch<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
    tabu_mat: Vec<Vec<usize>>
}

#[allow(dead_code)]
impl<'a> TabuSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        let mut mat = vec![];

        for i in 0..N {
            mat.push(vec![]);
            for _ in 0..N {
                mat[i].push(0);
            }
        }

        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
            tabu_mat:mat
        }
    }

    pub fn run(&self){
        todo!()
    }
}
