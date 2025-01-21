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

// impl FmtAdvanced for ABC {
//     type Target = str;
//     fn fmt_advanced(&self) -> &Self::Target {
//         match self {
//             ABC::A => "A",
//             ABC::B => "B",
//             ABC::C => "C",
//         }
//     }
// }

fmt2::enum_alias! { enum AB: ABC = { A | B }; }
impl AB {
    pub const fn rev(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}
fmt2::enum_alias! { enum AC: ABC = { A | C }; }
impl AC {
    pub const fn rev(self) -> Self {
        match self {
            Self::A => Self::C,
            Self::C => Self::A,
        }
    }
}
fmt2::enum_alias! { enum BC: ABC = { B | C }; }
impl BC {
    pub const fn rev(self) -> Self {
        match self {
            Self::B => Self::C,
            Self::C => Self::B,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    A(BC),
    B(AC),
    C(AB),
}
impl Route {
    #[inline]
    const fn start_middle_end(self) -> (ABC, ABC, ABC) {
        match self {
            Self::A(bc) => (ABC::A, bc.rev().into_parent(), bc.into_parent()),
            Self::B(ac) => (ABC::B, ac.rev().into_parent(), ac.into_parent()),
            Self::C(ab) => (ABC::C, ab.rev().into_parent(), ab.into_parent()),
        }
    }
    #[inline]
    const fn start(self) -> ABC {
        self.start_middle_end().0
    }
    #[inline]
    const fn middle(self) -> ABC {
        self.start_middle_end().1
    }
    #[inline]
    const fn end(self) -> ABC {
        self.start_middle_end().2
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    const fn from_start_to_middle(self) -> Self {
        match self {
            Self::A(bc) => Self::A(bc.rev()),
            Self::B(ac) => Self::B(ac.rev()),
            Self::C(ab) => Self::C(ab.rev()),
        }
    }
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    const fn from_end_to_middle(self) -> Self {
        match self {
            Self::A(BC::B) => Self::B(AC::C),
            Self::A(BC::C) => Self::C(AB::B),
            Self::B(AC::A) => Self::A(BC::C),
            Self::B(AC::C) => Self::C(AB::A),
            Self::C(AB::A) => Self::A(BC::B),
            Self::C(AB::B) => Self::B(AC::A),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    #[serde(alias = "from")]
    start: ABC,
    #[serde(alias = "to")]
    end: ABC,
}

impl From<Route> for Move {
    fn from(route: Route) -> Self {
        Self {
            start: route.start(),
            end: route.end(),
        }
    }
}

impl TryFrom<Move> for Route {
    type Error = ();

    fn try_from(value: Move) -> core::result::Result<Self, Self::Error> {
        Ok(match value {
            Move { start: ABC::A, end } => Route::A(end.try_into()?),
            Move { start: ABC::B, end } => Route::B(end.try_into()?),
            Move { start: ABC::C, end } => Route::C(end.try_into()?),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Game {
    #[serde(alias = "pegA")]
    a: Vec<TowerUint>,
    #[serde(alias = "pegB")]
    b: Vec<TowerUint>,
    #[serde(alias = "pegC")]
    c: Vec<TowerUint>,
    #[serde(skip)]
    count: TowerUint,
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

    pub fn hint(&self) -> Option<Route> {
        let range = (0..self.count).rev();
        let mut end = ABC::C;
        for unit in range {
            let (start, start_is_blocked) = self.find_unit(unit);
            let Ok(route) = Route::try_from(Move { start, end }) else {
                continue;
            };
            let end_is_blocked = self.get_ref(route.end()).iter().any(|&unit_| unit_ < unit);
            if start_is_blocked || end_is_blocked {
                end = route.middle();
                continue;
            }
            return Some(route);
        }
        None
    }

    pub fn hint_with_move(&self) -> Move {
        self.hint().map(From::from).unwrap_or(Move {
            start: ABC::C,
            end: ABC::C,
        })
    }

    pub fn play(&mut self, route: Route) -> Result {
        if self.is_valid_route(route) {
            let start = self.get_mut(route.start()).pop().expect("unreachable");
            self.get_mut(route.end()).push(start);
            Ok(())
        } else {
            Err(Error)
        }
    }

    pub fn play_with_move(&mut self, player_move: Move) -> Result<&mut Self> {
        if let Ok(route) = player_move.try_into() {
            self.play(route)?;
        }
        Ok(self)
    }

    fn is_valid_route(&self, route: Route) -> bool {
        self.is_valid_placement(
            self.get_ref(route.start()).last().copied(),
            self.get_ref(route.end()).last().copied(),
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

    fn find_unit(&self, count: TowerUint) -> (ABC, bool) {
        if let Some(u) = index_of(&self.a, count) {
            return (ABC::A, self.a.get(u + 1).is_some());
        }
        if let Some(u) = index_of(&self.b, count) {
            return (ABC::B, self.b.get(u + 1).is_some());
        }
        if let Some(u) = index_of(&self.c, count) {
            return (ABC::C, self.c.get(u + 1).is_some());
        }
        panic!("unreachable");
    }

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
