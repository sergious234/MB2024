use super::*;

pub struct Ils<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> Ils<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Trucks::default(),
        }
    }

    pub fn run(&self, search_kind: SearchType) -> Trucks {
        let mut best_sol = gen_sol(self.palets);
        let mut best_cost = cost(self.cost_mat, &best_sol);

        let mut mutation_sol = best_sol;

        for _ in 0..10 {
            let next_sol = search_kind.search(self.cost_mat, self.palets, mutation_sol);
            let next_cost = cost(self.cost_mat, &next_sol);

            if next_cost < best_cost {
                best_sol = next_sol;
                best_cost = next_cost;
                mutation_sol = mutation(&next_sol);
            } else {
                mutation_sol = mutation(&best_sol);
            }
        }

        println!("{:?} Cost: {}", search_kind, best_cost);
        best_sol
    }
}

fn mutation(t: &Trucks) -> Trucks {
    let mut trucks = t.clone();
    let truck_a = RNG::next() % N_TRUCKS;
    let mut truck_b = RNG::next() % N_TRUCKS;

    while truck_b == truck_a || (truck_b).abs_diff(truck_a) > 4 {
        truck_b = RNG::next() % N_TRUCKS;
    }

    let palet_a = RNG::next() % TRUCK_CAP;
    let mut palet_b = RNG::next() % TRUCK_CAP;

    while palet_b == palet_a || palet_b.abs_diff(palet_a) > 4 {
        palet_b = RNG::next() % TRUCK_CAP;
    }

    let truck_a_change = RNG::next() % N_TRUCKS;
    let truck_b_change = RNG::next() % N_TRUCKS;

    let truck_a_pal = RNG::next() % TRUCK_CAP;
    let truck_b_pal = RNG::next() % TRUCK_CAP;

    let aux = trucks[truck_a][palet_a];
    trucks[truck_a][palet_a] = trucks[truck_a_change][truck_a_pal];
    trucks[truck_a_change][truck_a_pal] = aux;

    let aux = trucks[truck_b][palet_b];
    trucks[truck_b][palet_b] = trucks[truck_b_change][truck_b_pal];
    trucks[truck_b_change][truck_b_pal] = aux;

    trucks
}
