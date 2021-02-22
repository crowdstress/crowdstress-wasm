use crate::config;
use crate::human::Human;
use crowdstress_common::prelude::*;
use js_sys::Math;

struct ExitExtended {
    probability: f64,
    middle: Point,
    vector_to_target: Vector,
}

pub fn get_target(human: &Human, exit_sections: &Vec<Exit>) -> Option<Point> {
    #[derive(Copy, Clone)]
    struct ExitGeometry {
        middle: Point,
        vector_to_target: Vector,
        vector_from_human: Vector,
    }

    if exit_sections.is_empty() {
        return None;
    }

    let exits: Vec<ExitGeometry> = exit_sections
        .iter()
        .map(|exit| {
            let section_middle = geometry::get_section_middle(&exit.section);
            let vector_from_human = Vector::from_points(&human.coords, &section_middle);
            let vector_to_target = Vector::from_points(&exit.section.start, &exit.section.end)
                .normalize()
                .perpendicular()
                .product(config::TARGET_FROM_EXIT_DISTANCE);
            let factor = if vector_to_target.projection_to(&vector_from_human) < 0.0 {
                -1.0
            } else {
                1.0
            };
            let vector_to_target = vector_to_target.product(factor);
            ExitGeometry {
                middle: section_middle,
                vector_to_target,
                vector_from_human,
            }
        })
        .collect();

    let distances_sum: f64 = exits
        .iter()
        .map(|exit| exit.vector_from_human.get_length())
        .sum();

    let exits_extended: Vec<ExitExtended> = exits
        .iter()
        .map(|exit| ExitExtended {
            probability: if exits.len() > 1 {
                (distances_sum - exit.vector_from_human.get_length()) / distances_sum
            } else {
                1.0
            },
            middle: exit.middle,
            vector_to_target: exit.vector_to_target,
        })
        .collect();

    let selected_exit = select_exit(&exits_extended);

    Option::from(
        selected_exit
            .vector_to_target
            .to_line(selected_exit.middle)
            .end,
    )
}

fn select_exit(exits: &Vec<ExitExtended>) -> &ExitExtended {
    let rnd: f64 = Math::random();

    for exit in exits {
        let result = rnd - exit.probability;
        if result < 0.0 {
            return exit;
        }
    }

    &exits[0]
}
