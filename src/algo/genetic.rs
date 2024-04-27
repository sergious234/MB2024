use crate::logger;

use super::*;

pub const K: usize = 2;
pub const N_ELITISM: usize = 5;
pub const N_CROMOSOMA: usize = 50;
pub const N_GEN: usize = 50_000;

pub struct Genetic<'a> {
    poblacion: Vec<Cromosoma>,
    palets: &'a Palets,
    cost_mat: &'a Costs,
}

impl<'a> Genetic<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        let greedy = GAGreedyP::new(cost_mat, palets);
        let mut poblacion: Vec<Cromosoma> = (0..N_CROMOSOMA - 1)
            .map(|_| Cromosoma::get_random())
            .collect();

        poblacion.push(greedy.run());

        for p in poblacion.iter_mut() {
            p.cost(palets, cost_mat);
        }

        Self {
            poblacion,
            palets,
            cost_mat,
        }
    }

    pub fn run(&mut self) {
        let mut gen = 0;
        const CROSS: CrossType = CrossType::AEX;
        const MUT_TYPE: MutType = MutType::Simple;

        /*
        for p in self.poblacion.iter_mut() {
            p.cost(self.palets, self.cost_mat);
        }
        */

        let mut next_gen = vec![];
        const PAIRS: usize = N_CROMOSOMA / 2 - 2;
        while gen < N_GEN {
            // println!("{gen}");
            gen += 1;
            self.poblacion.sort_by(|a, b| a.cost.cmp(&b.cost));

            if gen % 100 == 0 {
                let mean_cost = self.poblacion.iter().map(|e| e.cost).min().expect("E");
                logger::log(gen, mean_cost);
            }

            let mut rec_list = vec![];
            for _cross in 0..PAIRS {
                rec_list.clear();
                let mut index_list = vec![];
                let mut i = 0;
                while i < K {
                    let index = RNG::next() % N_CROMOSOMA;
                    let next = &self.poblacion[index];
                    if !index_list.contains(&index) {
                        rec_list.push(next);
                        i += 1;
                    }
                }

                let parent1 = *rec_list
                    .iter()
                    .min_by(|a, b| a.cost.cmp(&b.cost))
                    .expect("Missing parent 1");

                i = 0;
                rec_list.clear();
                index_list = vec![];
                while i < K {
                    let index = RNG::next() % N_CROMOSOMA;
                    let next = &self.poblacion[RNG::next() % N_CROMOSOMA];
                    if !index_list.contains(&index) {
                        rec_list.push(next);
                        i += 1;
                    }
                }

                let parent2 = *rec_list
                    .iter()
                    .min_by(|a, b| a.cost.cmp(&b.cost))
                    .expect("Missing parent 2");

                let cross_prob = RNG::next_f64();

                let mut offpring = if cross_prob <= 0.85 {
                    CROSS.cross(&parent1, &parent2)
                } else {
                    (parent1.clone(), parent2.clone())
                };

                let mut_prob = RNG::next_f64();

                if mut_prob < 0.05 {
                    MUT_TYPE.apply(&mut offpring.0);
                    MUT_TYPE.apply(&mut offpring.1);
                }

                offpring.0.cost(self.palets, self.cost_mat);
                offpring.1.cost(self.palets, self.cost_mat);

                next_gen.push(offpring.0);
                next_gen.push(offpring.1);
            }

            for _ in 0..N_ELITISM {
                next_gen.push(self.poblacion.remove(0));
            }

            /*
            for i in N_ELITISM..self.poblacion.len() {
                let mut best = usize::MAX;
                let mut index = 0;
                for j in 0..next_gen.len() {
                    let diff = next_gen[j].diff(&self.poblacion[i], &self.palets);
                    let better = next_gen[j].cost < self.poblacion[i].cost;
                    if better && diff < best {
                        index = j;
                        best = diff;
                    }
                }

                // 2_000
                if best == 0 {
                    continue;
                }
                let prob = RNG::next_f64();
                if best < N_PALETS && (best < 10 || prob > 0.9) {
                    println!(
                        "Change: {}-{} | {} [{}]",
                        self.poblacion[i].cost, next_gen[index].cost, best, self.poblacion[0].cost
                    );
                    self.poblacion[i] = next_gen.remove(index);
                }
            }

            next_gen.clear();

            if gen % N_GEN / 25 == 0 {
                const X: usize = N_CROMOSOMA / 5;
                println!("tick");
                for _ in 0..X {
                    let index = (RNG::next() % N_CROMOSOMA - X) + X;
                    self.poblacion[index] = Cromosoma::get_random();
                    self.poblacion[index].cost(self.palets, self.cost_mat);
                }
            }
            */
            self.poblacion.clear();

            self.poblacion.append(&mut next_gen);
            self.poblacion.remove(0);
        }

        for p in self.poblacion.iter_mut() {
            p.cost(self.palets, self.cost_mat);
        }

        let best = self
            .poblacion
            .iter()
            .min_by(|a, b| a.cost.cmp(&b.cost))
            .unwrap();
        println!("GA: {}", best.cost);
        let mut sol = Trucks::default();

        for (truck, chunk) in sol.iter_mut().zip(best.palets.chunks(TRUCK_CAP)) {
            for i in 0..TRUCK_CAP {
                truck[i] = self.palets[chunk[i] as usize];
            }
        }

        println!(
            "BLGA: {}",
            cost(
                self.cost_mat,
                &LocalSearchBF::new(self.cost_mat, self.palets).run_with_start(sol)
            )
        );
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct Cromosoma {
    pub palets: Palets,
    pub cost: usize,
}

impl PartialEq for Cromosoma {
    fn eq(&self, other: &Self) -> bool {
        if self.cost != other.cost {
            return false;
        }
        for i in 0..N_PALETS {
            if self.palets[i] != other.palets[i] {
                return false;
            }
        }
        true
    }
}

impl Cromosoma {
    pub fn diff(&self, other: &Cromosoma, palets: &Palets) -> usize {
        self.palets
            .iter()
            .zip(other.palets.iter())
            .map(|(&t, &o)| (t as i32 - o as i32) & 1)
            .map(|e| e as usize)
            //.map(|(&t, &o)| (palets[t as usize] as i32, palets[o as usize] as i32))
            //.map(|(t, o)| (t-o).pow(2))
            //.map(|e| (e as f64).sqrt() as usize)
            .sum()
    }

    pub fn cost(&mut self, palets: &Palets, cost_mat: &Costs) -> usize {
        if self.cost != 0 {
            return self.cost;
        }
        let mut cost = 0;
        let mut visited;
        let mut actual_city;

        for truck in self.palets.chunks(14) {
            visited = [false; N];
            actual_city = 0;
            visited[0] = true;
            cost += truck
                .iter()
                .map(|e| (palets[*e as usize] - 1) as usize)
                .filter_map(|x| {
                    (!visited[x]).then(|| {
                        let c = cost_mat[actual_city][x];
                        visited[x] = true;
                        actual_city = x;
                        c
                    })
                })
                .sum::<usize>()
                + cost_mat[actual_city][0];
        }

        self.cost = cost;
        cost
    }

    pub fn get_random() -> Cromosoma {
        let mut cromosome_palets = vec![];
        let mut palets: Vec<usize> = (0..N_PALETS).collect();

        while !palets.is_empty() {
            let next = RNG::next() % palets.len();
            let p = palets.remove(next);
            cromosome_palets.push(p as Palet);
        }

        Self {
            palets: cromosome_palets,
            cost: 0,
        }
    }

    pub fn new_with(palets: Palets) -> Cromosoma {
        Self { palets, cost: 0 }
    }
}

impl Default for Cromosoma {
    fn default() -> Self {
        Self {
            palets: vec![0; N_PALETS],
            cost: 0,
        }
    }
}

pub enum MutType {
    Simple,
    #[allow(dead_code)]
    Reversed,
}

impl MutType {
    pub fn apply(&self, child: &mut Cromosoma) {
        match self {
            MutType::Simple => {
                let palet_a = RNG::next() % N_PALETS;
                let mut palet_b = RNG::next() % N_PALETS;
                while palet_b == palet_a {
                    palet_b = RNG::next() % N_PALETS;
                }
                (*child).palets.swap(palet_a, palet_b);
            }
            MutType::Reversed => {
                let mut start = RNG::next() % N_PALETS;
                let mut end = RNG::next() % N_PALETS;

                if start > end {
                    std::mem::swap(&mut start, &mut end);
                }

                while end > start {
                    (*child).palets.swap(start, end);
                    start += 1;
                    end -= 1;
                }
            }
        }
    }
}

#[allow(dead_code)]
pub enum CrossType {
    OX,
    AEX,
}

impl CrossType {
    fn cross_ox(start: usize, end: usize, parent1: &Cromosoma, parent2: &Cromosoma) -> Cromosoma {
        let mut child1 = Cromosoma::default();

        for i in start..end {
            child1.palets[i] = parent1.palets[i];
        }

        let cross_sec = &parent1.palets[start..end];
        let mut mask = [true; N_PALETS];
        for p in cross_sec {
            mask[*p as usize] = false;
        }
        let mut rest = parent2
            .palets
            .iter()
            .filter(|p| mask[**p as usize])
            .cloned();

        for i in 0..start {
            // println!("{}, {}", i, rest.len());
            child1.palets[i] = rest.next().expect("No queda iterador");
        }

        for (next, child) in rest.zip(child1.palets.iter_mut().skip(end)) {
            *child = next;
        }

        child1
    }

    pub fn cross_aex(parent1: &Cromosoma, parent2: &Cromosoma) -> Cromosoma {
        let mut cromosoma: Vec<u8> = Vec::with_capacity(N_PALETS); //vec![];
        let mut mask_p1 = [true; N_PALETS];
        let mut mask_p2 = [true; N_PALETS];

        let mut where_p1 = [0; N_PALETS];
        let mut where_p2 = [0; N_PALETS];
        for i in 0..N_PALETS {
            where_p1[parent1.palets[i] as usize] = i;
            where_p2[parent2.palets[i] as usize] = i;
        }

        cromosoma.push(parent1.palets[0]);
        cromosoma.push(parent1.palets[1]);

        const P1: bool = false;
        const P2: bool = true;
        let mut which = P2;

        let pos = |which: bool, pal: Palet| {
            if which == P1 {
                where_p1[pal as usize]
            } else {
                where_p2[pal as usize]
            }
        };

        mask_p1[pos(P1, parent1.palets[0])] = false;
        mask_p1[pos(P1, parent1.palets[1])] = false;
        mask_p2[pos(P2, parent1.palets[0])] = false;

        let mut avaliables: Vec<usize> = vec![];
        let mut mask;
        let mut parent;
        let mut range_iter = (0..N_PALETS).cycle();

        // let mut branchless = [
        //     (&mut mask_p1, &parent1),
        //     (&mut mask_p2, &parent2),
        // ];
        while cromosoma.len() < N_PALETS {
            let last = cromosoma.last().cloned().unwrap();
            // mask = &mut branchless[which].0;
            // parent = branchless[which].1;
            match which {
                P1 => {
                    mask = &mut mask_p1;
                    parent = &parent1;
                }
                P2 => {
                    mask = &mut mask_p2;
                    parent = &parent2;
                }
            }

            let pos = pos(which, last);
            mask[pos] = false;
            let next = pos + 1;
            if next < N_PALETS && mask[next] {
                mask[next] = false;
                // println!("Easy {}", parent1.palets[next_in_p1]);
                cromosoma.push(parent.palets[next])
            } else {
                /*
                let (next, _) = mask
                    .iter()
                    .enumerate()
                    .filter(|e| *e.1 == true)
                    .cycle()
                    .find(|_| RNG::next_f64() > 0.8)
                    .unwrap();

                for n in (0..N_PALETS).filter(|&e| mask[e] == true) {
                   avaliables.push(n);
                }

                let next = avaliables[RNG::next() % avaliables.len()];
                */
                let next = range_iter.find(|&e| mask[e] == true).unwrap();
                mask[next] = false;
                cromosoma.push(parent.palets[next])
            }
            // which = (which+1)%2;
            which = !which;
            avaliables.clear();
        }

        //assert_eq!(mask_p1.iter().filter(|e| **e == true).count(),0);
        //assert_eq!(mask_p2.iter().filter(|e| **e == true).count(),0);
        // println!("{:?}", cromosoma);

        Cromosoma::new_with(cromosoma)
    }

    pub fn cross(&self, parent1: &Cromosoma, parent2: &Cromosoma) -> (Cromosoma, Cromosoma) {
        match self {
            CrossType::OX => {
                let mut start = RNG::next() % N_PALETS;
                let mut end = RNG::next() % N_PALETS;

                if start > end {
                    std::mem::swap(&mut start, &mut end);
                }

                let child1 = CrossType::cross_ox(start, end, parent1, parent2);
                let child2 = CrossType::cross_ox(start, end, parent2, parent1);
                (child1, child2)
            }
            CrossType::AEX => {
                let child1 = CrossType::cross_aex(parent1, parent2);
                let child2 = CrossType::cross_aex(parent2, parent1);
                (child1, child2)
            }
        }
    }
}

pub struct GAGreedyP<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
    #[allow(unused)]
    trucks: Trucks,
}

#[allow(dead_code)]
impl<'a> GAGreedyP<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Trucks::default(),
        }
    }

    pub fn run(&self) -> Cromosoma {
        let mut sol = Cromosoma::default();
        let mut pals = [0; N];
        for pal in self.palets.iter().cloned() {
            pals[pal as usize - 1] += 1;
        }
        let mut lens = [0; N_TRUCKS];

        let mut lefts: Vec<usize> = (0..N_PALETS).collect();
        let mut cromo: Vec<_> = vec![[0; TRUCK_CAP]; N_TRUCKS];

        let mut pal_costs = vec![];
        for (i, truck) in cromo.iter_mut().enumerate() {
            let last = if truck.len() == 0 {
                0
            } else {
                truck[truck.len() - 1] as usize
            };

            while lens[i] < TRUCK_CAP {
                pal_costs.clear();
                lefts
                    .iter()
                    // Ciudad, Id
                    .map(|&p| (self.palets[p as usize], p))
                    .filter(|&p| pals[p.0 as usize - 1] > 0)
                    .map(|pal| (self.cost_mat[last][pal.0 as usize - 1], pal.0, pal.1))
                    .for_each(|data| pal_costs.push(data));

                pal_costs.sort_by(|a, b| a.0.cmp(&b.0));

                // Coste, Destino, Id
                let best_palets: Vec<_> = pal_costs
                    .iter()
                    .map(|e| e)
                    .take((pals.iter().sum::<usize>() as f64 * 0.1).ceil() as usize)
                    .collect();
                let chosen_one = *best_palets.get(RNG::next() % best_palets.len()).unwrap();

                let last = lens[i];

                (*truck)[last] = chosen_one.2 as u8;
                lefts.remove(
                    lefts
                        .iter()
                        .position(|&e| e == chosen_one.2)
                        .expect("Not found"),
                );
                lens[i] += 1;
                pals[chosen_one.1 as usize - 1] -= 1;
            }
        }

        /*
            let mut pal_costs = vec![];
            for (i, truck) in sol.iter_mut().enumerate() {
                while lens[i] < TRUCK_CAP {
                    pal_costs.clear();

                    for pal in self.palets.iter().cloned() {
                        if pals[pal as usize - 1] > 0 {
                            let t_len = lens[i];
                            let last = if t_len == 0 {
                                0
                            } else {
                                truck[t_len - 1] as usize - 1
                            };
                            pal_costs.push((self.cost_mat[last][pal as usize - 1], pal));
                        }
                    }

                    pal_costs.sort_by(|a, b| a.0.cmp(&b.0));
                    let best_palets: Vec<u8> = pal_costs
                        .iter()
                        .map(|e| e.1)
                        .take((pals.iter().sum::<usize>() as f64 * 0.1).ceil() as usize)
                        .collect();
                    let last = lens[i];
                    let chosen_one = *best_palets.get(RNG::next() % best_palets.len()).unwrap();

                    truck[last] = chosen_one;
                    lens[i] += 1;
                    pals[chosen_one as usize - 1] -= 1;
                }
            }
        */

        sol.palets = cromo.into_iter().flatten().collect();
        sol.cost(self.palets, self.cost_mat);
        sol
    }
}
