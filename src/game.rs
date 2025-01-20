use fmt2::write_to::{FmtAdvanced, WriteTo};

pub struct Error;

pub type Result<T = ()> = core::result::Result<T, Error>;

type TowerUint = usize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
struct StartEnd {
    start: ABC,
    end: ABC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
    A(BC),
    B(AC),
    C(AB),
}

impl WriteTo for Route {
    fmt2::fmt! { [s] =>
        r#"{"from":"# {s.start_middle_end().0} r#","to":"# {s.start_middle_end().2} r#"}"#
    }
}

impl Route {
    fn start_middle_end(self) -> (ABC, ABC, ABC) {
        match self {
            Self::A(bc) => (ABC::A, ABC::from(bc.rev()), ABC::from(bc)),
            Self::B(ac) => (ABC::B, ABC::from(ac.rev()), ABC::from(ac)),
            Self::C(ab) => (ABC::C, ABC::from(ab.rev()), ABC::from(ab)),
        }
    }
    fn start(self) -> ABC {
        self.start_middle_end().0
    }
    fn middle(self) -> ABC {
        self.start_middle_end().1
    }
    fn end(self) -> ABC {
        self.start_middle_end().2
    }
}

#[derive(Clone)]
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

    fn hint_internal(&self, route: Route, unit: TowerUint, index: usize) -> StartEnd {
        let (start, middle, end) = route.start_middle_end();
        if let Some(end_blocker) = index_of_lt(self.get_ref(end), unit) {
            return self.hint_internal(route, unit, index);
        }
        if let Some(start_blocker) = self.get_ref(start).get(index + 1) {
            return StartEnd { start, end: middle };
        }
        StartEnd { start, end }
    }

    pub fn hint(&self) -> StartEnd {
        let range = (0..self.count).rev();
        for unit in range {
            if self.c.contains(&unit) {
                continue;
            }
            let (route, index) = if let Some(index) = index_of(&self.a, unit) {
                (Route::A(BC::C), index)
            } else {
                (
                    Route::B(AC::C),
                    index_of(&self.b, unit).expect("unreachable"),
                )
            };
            return self.hint_internal(route, unit, index);
        }
        StartEnd {
            start: ABC::C,
            end: ABC::C,
        }
    }

    fn is_valid_route(&self, route: Route) -> bool {
        let (start, _, end) = route.start_middle_end();
        self.is_valid_placement(
            self.get_ref(start).last().copied(),
            self.get_ref(end).last().copied(),
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

    pub fn play(&mut self, route: Route) -> Result<&mut Self> {
        let (start, _, end) = route.start_middle_end();
        if self.is_valid_route(route) {
            let start = self.get_mut(start).pop().expect("unreachable");
            self.get_mut(end).push(start);
            Ok(self)
        } else {
            Err(Error)
        }
    }

    fn find_tower_abc(&self, count: TowerUint) -> (ABC, usize) {
        if let Some(u) = index_of(&self.a, count) {
            return (ABC::A, u);
        }
        if let Some(u) = index_of(&self.b, count) {
            return (ABC::B, u);
        }
        if let Some(u) = index_of(&self.b, count) {
            return (ABC::B, u);
        }
        panic!("unreachable");
    }

    fn find_tower_mut(&self, count: TowerUint) -> ABC {
        if self.a.contains(&count) {
            return ABC::A;
        }
        if self.b.contains(&count) {
            return ABC::B;
        }

        ABC::C
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

fn index_of_lt<T>(s: &[T], value: T) -> Option<usize>
where
    T: PartialEq + Copy,
{
    s.iter()
        .enumerate()
        .find(|(_, &v)| v < value)
        .map(|(u, _)| u)
}
