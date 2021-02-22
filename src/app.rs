use crate::human::Human;
use crate::{behaviour, config, physics};
use crowdstress_common::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct App {
    started: bool,
    humans: Vec<Human>,
    rooms: Vec<Room>,
    walls: Vec<Section>,
    exits: Vec<Exit>,
}

impl App {
    pub fn tick(&mut self) -> JsValue {
        let mut new_humans: Vec<Human> = Vec::with_capacity(self.humans.len());

        for human in &self.humans {
            let mut human_vectors: Vec<Vector> = Vec::new();
            let mut passed_exits = human.passed_exits.clone();
            let mut target = human.target;

            match target {
                Some(point) => {
                    human_vectors.push(physics::f1(&human, &point));
                    for exit in &self.exits {
                        let is_passed = is_exit_passed(&exit, &human.coords, &point);
                        if is_passed && !passed_exits.contains(&exit.id) {
                            web_sys::console::log_3(
                                &"Exit".into(),
                                &String::from(&exit.id).into(),
                                &"passed".into(),
                            );
                            passed_exits.push(String::from(&exit.id));
                            target = None;
                            break;
                        }
                    }
                }
                None => {
                    let room = get_room(&self.rooms, &human.coords);
                    match room {
                        Some(room) => {
                            let possible_exits =
                                get_possible_exits(&room, &self.exits, &passed_exits);
                            let exit_target = behaviour::get_target(&human, &possible_exits);

                            match exit_target {
                                Some(point) => target = Option::from(point),
                                None => {
                                    web_sys::console::log_1(&"No target - destroy".into());
                                    continue;
                                }
                            }
                        }
                        None => {
                            web_sys::console::log_1(&"No room - destroy".into());
                            continue;
                        }
                    }
                }
            }

            for wall in &self.walls {
                let vector_to_line = geometry::get_vector_to_line(
                    &Section {
                        start: wall.start,
                        end: wall.end,
                    },
                    &human.coords,
                );
                if geometry::is_lines_intersects(
                    &vector_to_line
                        .normalize()
                        .product(9999.0)
                        .to_line(human.coords),
                    wall,
                ) {
                    human_vectors.push(physics::f2w(vector_to_line));
                }
            }

            for other_human in &self.humans {
                let is_self = other_human as *const Human == human as *const Human;
                if !is_self {
                    let vector_to_human = Vector::from_points(&human.coords, &other_human.coords);
                    human_vectors.push(physics::f2(vector_to_human));
                }
            }

            let mut result_vector: Vector = Vector::origin();

            for vector in human_vectors {
                result_vector = result_vector.add(&vector);
            }

            let a = result_vector.divide(config::HUMAN_MASS);
            let dr = a.product(config::DELTA_T.powf(2.0) * config::SCALING_FACTOR);

            new_humans.push(Human {
                coords: Point {
                    x: human.coords.x + dr.x,
                    y: human.coords.y + dr.y,
                },
                target,
                panic: if result_vector.get_length() > 800.0 {
                    100
                } else {
                    human.panic
                },
                passed_exits,
            });
        }

        self.humans = new_humans;
        JsValue::from_serde(&self).unwrap()
    }
}

fn get_room_exits(room: &Room, exits: &Vec<Exit>) -> Vec<Exit> {
    exits
        .iter()
        .filter(|exit| room.exits.contains(&exit.id))
        .map(|exit| Exit {
            id: String::from(&exit.id),
            section: exit.section,
        })
        .collect()
}

fn get_possible_exits(room: &Room, exits: &Vec<Exit>, passed_exits: &Vec<String>) -> Vec<Exit> {
    get_room_exits(&room, &exits)
        .iter()
        .filter(|exit| !passed_exits.contains(&exit.id))
        .map(|exit| Exit {
            id: String::from(&exit.id),
            section: exit.section,
        })
        .collect()
}

fn get_room(rooms: &Vec<Room>, coords: &Point) -> Option<Room> {
    let mut current_room: Option<Room> = None;

    for room in rooms {
        let is_human_in = geometry::is_point_in_polygon(&room.points, &coords);
        if is_human_in {
            current_room = Option::from(Room {
                id: String::from(&room.id),
                points: room.points.clone(),
                exits: room.exits.clone(),
            });
            break;
        }
    }

    current_room
}

fn is_exit_passed(exit: &Exit, coords: &Point, target: &Point) -> bool {
    let human_to_target_vector = Vector::from_points(&coords, &target);
    let vector = Vector::from_points(&exit.section.start, &exit.section.end)
        .normalize()
        .perpendicular()
        .product(config::TARGET_FROM_EXIT_DISTANCE);
    let factor = if vector.projection_to(&human_to_target_vector) < 0.0 {
        -1.0
    } else {
        1.0
    };
    let vector = vector.product(factor);
    let section = exit.section.move_to(&vector);
    geometry::is_point_belongs_to_line(&section, &coords)
}
