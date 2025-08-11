use crate::secret_jesus::{EVENTS_PER_GAME, Prodigy};
use crate::{Role, get_roles};
use rustsat::encodings::card::{BoundBoth, DefIncBothBounding};
use rustsat::instances::Cnf;
use rustsat::solvers::Solve;
use rustsat::types::constraints::CardConstraint;
use rustsat::{instances::SatInstance, types::Lit};
use rustsat_minisat::core::Minisat;
use strum::IntoEnumIterator;

pub struct JesusFinder {
    instance: SatInstance,
    // One variable per role assignment.
    x_identity_vars: Vec<[Lit; Role::NUMBER]>,
    // For each player, assign a slice of 5 prodigies (one per event) for each prodigy type.
    x_events_vars: Vec<[[Lit; EVENTS_PER_GAME]; Prodigy::NUMBER]>,
}

impl JesusFinder {
    pub fn new(n: usize) -> Self {
        let mut instance = SatInstance::new();

        let x_identity_vars = (0..n)
            .map(|_| {
                (0..Role::NUMBER)
                    .map(|_| instance.new_lit())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Role count mismatch")
            })
            .collect::<Vec<_>>();

        let x_events_vars: Vec<[[Lit; EVENTS_PER_GAME]; Prodigy::NUMBER]> = (0..n)
            .map(|_| {
                (0..Prodigy::NUMBER)
                    .map(|_| {
                        (0..EVENTS_PER_GAME)
                            .map(|_| instance.new_lit())
                            .collect::<Vec<_>>()
                            .try_into()
                            .expect("Event count mismatch")
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Prodigy count mismatch")
            })
            .collect::<Vec<_>>();

        // Per-role constraints
        let roles = get_roles(n);
        for role in Role::iter() {
            let role_mul = roles.iter().filter(|&r| *r == role).count();
            let x_role_vars = x_identity_vars
                .iter()
                .map(|v: &[Lit; Role::NUMBER]| v[role as usize])
                .collect::<Vec<_>>();

            // Exactly role_mul identities are true
            instance.add_card_constr(CardConstraint::new_eq(x_role_vars, role_mul));
        }

        // Per-player constraints
        for i in 0..n {
            let player_identity_vars = x_identity_vars[i];

            // Player has exactly one role => exactly one identity is true
            instance.add_card_constr(CardConstraint::new_eq(player_identity_vars, 1));

            // Boon constraints
            let player_event_vars: [[Lit; EVENTS_PER_GAME]; Prodigy::NUMBER] = x_events_vars[i];
            for role in Role::iter() {
                let role_identity_var = player_identity_vars[role as usize];

                // Build the cardinality constraint for each prodigy type
                for prodigy in Prodigy::iter() {
                    let player_event_prodigy_vars = player_event_vars[prodigy as usize];
                    let prodigy_mul = role
                        .initial_prodigies()
                        .iter()
                        .filter(|&&p| p == prodigy)
                        .count();

                    let mut encoding = Cnf::new();
                    let prodigy_contraint =
                        CardConstraint::new_eq(player_event_prodigy_vars, prodigy_mul);
                    DefIncBothBounding::encode_constr(
                        prodigy_contraint,
                        &mut encoding,
                        instance.var_manager_mut(),
                    )
                    .unwrap();

                    for clause in encoding.iter_mut() {
                        clause.add(!role_identity_var);
                        instance.add_clause(clause.clone());
                    }
                }
            }
        }

        Self {
            instance,
            x_identity_vars,
            x_events_vars,
        }
    }

    pub fn process_event(
        &mut self,
        event_index: usize,
        selected_player_ids: &[usize],
        selected_prodigies: &[Prodigy],
    ) {
        // Add cardinality contraints for played prodigies.
        for prodigy in Prodigy::iter() {
            let prodigy_count = selected_prodigies.iter().filter(|&&p| p == prodigy).count();

            let participating_event_vars: Vec<Lit> = selected_player_ids
                .iter()
                .map(|&id| self.x_events_vars[id][prodigy as usize][event_index])
                .collect();

            self.instance.add_card_constr(CardConstraint::new_eq(
                participating_event_vars,
                prodigy_count,
            ));
        }
    }

    pub fn find_jesus(&mut self) -> Vec<Vec<bool>> {
        let mut solver = Minisat::default();
        solver.add_cnf(self.instance.clone().into_cnf().0).unwrap();

        let mut results = Vec::new();
        loop {
            if let Ok(solved) = solver.solve() {
                if solved != rustsat::solvers::SolverResult::Sat {
                    break;
                }
            } else {
                break;
            }

            match solver.full_solution() {
                Ok(sol) => {
                    let model: Vec<bool> = self
                        .x_identity_vars
                        .iter()
                        .map(|&lits| {
                            let var = lits[Role::Jesus as usize].var();
                            sol[var] == rustsat::types::TernaryVal::True
                        })
                        .collect();
                    results.push(model.clone());

                    let block = self
                        .x_identity_vars
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &lits)| {
                            if model[i] {
                                Some(!lits[Role::Jesus as usize])
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    for &lit in &block {
                        solver.add_unit(lit).expect("Failed to add clause");
                    }
                }

                Err(e) => {
                    eprintln!("Error getting full solution: {}", e);
                    break;
                }
            }
        }

        results
    }
}
