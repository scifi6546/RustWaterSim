use super::{Mission, MissionScenario, WinState};
use crate::prelude::BrushBudget;
use bevy::prelude::*;
use nalgebra::Vector2;
use std::sync::{Arc, Mutex};
use water_sim::{
    BoundaryConditions, Grid, PreferredSolver, Solver, SolverBoundaryConditions, Source,
};
pub fn get_missions() -> Vec<Mission> {
    vec![
        Mission {
            scenario: Arc::new(Mutex::new(TsunamiScenario {})),
        },
        Mission {
            scenario: Arc::new(Mutex::new(Canal {})),
        },
    ]
}
pub fn insert_missions(mut commands: Commands) {
    commands.insert_resource(get_missions())
}
struct TsunamiScenario {}
impl MissionScenario for TsunamiScenario {
    fn get_solver(&self) -> (PreferredSolver, BrushBudget) {
        fn ground(x: usize, _y: usize) -> f32 {
            if x < 180 {
                0.0
            } else if x <= 200 {
                10.0 * (x as f32 - 180.0) / 20.0
            } else {
                10.0
            }
        }
        let dimensions = Vector2::new(300, 100);
        let ocean_level = 8.0;
        let ocean = BoundaryConditions::Ocean { level: ocean_level };
        let reflect = BoundaryConditions::Reflect;
        (
            PreferredSolver::new(
                Grid::from_fn(
                    |x, y| {
                        if x < 100 {
                            20.0
                        } else {
                            (ocean_level - ground(x, y)).max(0.0)
                        }
                    },
                    dimensions,
                ),
                Grid::from_fn(|x, y| ground(x, y), dimensions),
                Vec::new(),
                SolverBoundaryConditions {
                    x_plus: ocean,
                    x_minus: ocean,
                    y_plus: reflect,
                    y_minus: reflect,
                },
            ),
            BrushBudget {
                used_ground: 0.0,
                max_ground: Some(10_000.0),
            },
        )
    }
    fn name(&self) -> String {
        "Tsunami".to_string()
    }
    fn get_win_state(&self, solver: &PreferredSolver, budget: &BrushBudget) -> WinState {
        let loose_vol = 100.0f32;
        let water_height = solver.water_h();
        let vol = (200..300)
            .flat_map(|x| (0..50).map(move |y| water_height.get(x, y)))
            .fold(0.0, |acc, x| acc + x);
        if loose_vol < vol {
            WinState::Lost
        } else {
            let ground_percent = budget.used_ground * 100.0 / budget.max_ground.unwrap();
            WinState::InProgress(format!(
                "vol: {}%, used ground: {}%",
                vol / loose_vol * 100.0,
                ground_percent
            ))
        }
    }
}
pub struct Canal {}
impl MissionScenario for Canal {
    fn get_solver(&self) -> (PreferredSolver, BrushBudget) {
        fn g(x: usize, y: usize) -> f32 {
            let x = x as f32;
            let y = y as f32;
            let w = y - 100.0;
            let w = (w + 5.0 * (x * 0.1).sin()).abs();
            let d = (w / (1.0 + 0.01 * x)).min(20.0);
            d * 1.0 - 0.1 * x
        }
        let ocean_level = -20.0;
        let dimensions = Vector2::new(300, 200);
        (
            PreferredSolver::new(
                Grid::from_fn(|x, y| (ocean_level - g(x, y)).max(0.0), dimensions),
                Grid::from_fn(g, dimensions),
                vec![Source {
                    center: Vector2::new(50.0, 100.0),
                    radius: 5.0,
                    height: 1.0,
                    period: 1.0,
                }],
                SolverBoundaryConditions {
                    x_plus: BoundaryConditions::Ocean { level: ocean_level },
                    x_minus: BoundaryConditions::Ocean { level: ocean_level },
                    y_plus: BoundaryConditions::Ocean { level: ocean_level },
                    y_minus: BoundaryConditions::Ocean { level: ocean_level },
                },
            ),
            BrushBudget {
                used_ground: 0.0,
                max_ground: None,
            },
        )
    }

    fn name(&self) -> String {
        "Canal".to_string()
    }

    fn get_win_state(&self, _solver: &PreferredSolver, _budget: &BrushBudget) -> WinState {
        WinState::InProgress("todo".to_string())
    }
}
