use fmt2::write_to::{FmtAdvanced, WriteTo};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Error;

pub type Result<T = ()> = core::result::Result<T, Error>;

type TowerUint = usize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum ABC {
    A,
    B,
    C,
}

impl FmtAdvanced for ABC {
    type Target = str;
    fn fmt_advanced(&self) -> &Self::Target {
        match self {
            ABC::A => "A",
            ABC::B => "B",
            ABC::C => "C",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    start: ABC,
    end: ABC,
}

impl From<Route> for Move {
    fn from(route: Route) -> Self {
        Self {
            start: route.start,
            end: route.end,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Route {
    start: ABC,
    middle: ABC,
    end: ABC,
}

impl WriteTo for Route {
    fmt2::fmt! { [s] =>
        r#"{"from":"# {s.start} r#","to":"# {s.end} r#"}"#
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    a: Vec<TowerUint>,
    b: Vec<TowerUint>,
    c: Vec<TowerUint>,
    count: TowerUint,
}

impl WriteTo for Game {
    fmt2::fmt! { [s] =>
        "{"
        r#""pegA":"# {s.a;?}
        r#","pegB":"# {s.b;?}
        r#","pegC":"# {s.c;?}
        "}"
    }
}

impl Game {
    pub fn new(count: TowerUint) -> Self {
        Self {
            a: (0..count).rev().collect(),
            b: Vec::with_capacity(count),
            c: Vec::with_capacity(count),
            count,
        }
    }

    fn hint_internal(&self, route: Route, unit: TowerUint, index: usize) -> Move {
        let index_new = index + 1;
        let start_blocker = self.get_ref(route.start).get(index_new);
        if let Some(&unit_new) = start_blocker {
            return self.hint_internal(
                Route {
                    start: route.start,
                    middle: route.end,
                    end: route.middle,
                },
                unit_new,
                index_new,
            );
        }
        let end_blocker = self
            .get_ref(route.end)
            .iter()
            .enumerate()
            .find(|(_, &unit_)| unit_ < unit);
        if let Some((index_new, &unit_new)) = end_blocker {
            return self.hint_internal(
                Route {
                    start: route.end,
                    middle: route.start,
                    end: route.middle,
                },
                unit_new,
                index_new,
            );
        }
        Move::from(route)
    }

    pub fn hint(&self) -> Move {
        let range = (0..self.count).rev();
        for unit in range {
            if self.c.contains(&unit) {
                continue;
            }
            let (route, index) = if let Some(index) = index_of(&self.a, unit) {
                (
                    Route {
                        start: ABC::A,
                        middle: ABC::B,
                        end: ABC::C,
                    },
                    index,
                )
            } else {
                (
                    Route {
                        start: ABC::B,
                        middle: ABC::A,
                        end: ABC::C,
                    },
                    index_of(&self.b, unit).expect("unreachable"),
                )
            };
            return self.hint_internal(route, unit, index);
        }
        Move {
            start: ABC::C,
            end: ABC::C,
        }
    }

    fn is_valid_move(&self, player_move: Move) -> bool {
        self.is_valid_placement(
            self.get_ref(player_move.start).last().copied(),
            self.get_ref(player_move.end).last().copied(),
        )
    }

    fn is_valid_placement(&self, start: Option<TowerUint>, end: Option<TowerUint>) -> bool {
        let Some(start) = start else {
            return false;
        };
        let Some(end) = end else {
            return true;
        };
        end >= start
    }

    pub fn play(&mut self, player_move: Move) -> Result<&mut Self> {
        if self.is_valid_move(player_move) {
            let start = self.get_mut(player_move.start).pop().expect("unreachable");
            self.get_mut(player_move.end).push(start);
            Ok(self)
        } else {
            Err(Error)
        }
    }
    //
    //     fn find_tower_abc(&self, count: TowerUint) -> (ABC, usize) {
    //         if let Some(u) = index_of(&self.a, count) {
    //             return (ABC::A, u);
    //         }
    //         if let Some(u) = index_of(&self.b, count) {
    //             return (ABC::B, u);
    //         }
    //         if let Some(u) = index_of(&self.b, count) {
    //             return (ABC::B, u);
    //         }
    //         panic!("unreachable");
    //     }
    //
    //     fn find_tower_mut(&self, count: TowerUint) -> ABC {
    //         if self.a.contains(&count) {
    //             return ABC::A;
    //         }
    //         if self.b.contains(&count) {
    //             return ABC::B;
    //         }
    //
    //         ABC::C
    //     }

    fn get_ref(&self, tower: ABC) -> &[TowerUint] {
        match tower {
            ABC::A => &self.a,
            ABC::B => &self.b,
            ABC::C => &self.c,
        }
    }
    fn get_mut(&mut self, tower: ABC) -> &mut Vec<TowerUint> {
        match tower {
            ABC::A => &mut self.a,
            ABC::B => &mut self.b,
            ABC::C => &mut self.c,
        }
    }
}

fn index_of<T>(s: &[T], value: T) -> Option<usize>
where
    T: PartialEq + Copy,
{
    s.iter()
        .enumerate()
        .find(|(_, &v)| v == value)
        .map(|(u, _)| u)
}
