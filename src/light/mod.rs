use std::marker::PhantomData;

pub enum LightState {
    On(Light<On>),
    Off(Light<Off>),
}

impl LightState {
    pub const fn is_on(&self) -> bool {
        matches!(self, Self::On(_))
    }

    pub const fn is_off(&self) -> bool {
        matches!(self, Self::Off(_))
    }

    pub const fn turn_on(self) -> Self {
        match self {
            Self::Off(light) => Self::On(light.turn_on()),
            Self::On(light) => Self::On(light),
        }
    }

    pub const fn turn_off(self) -> Self {
        match self {
            Self::On(light) => Self::Off(light.turn_off()),
            Self::Off(light) => Self::Off(light),
        }
    }
}

pub struct Light<State> {
    state: PhantomData<State>,
}

pub struct On;
pub struct Off;

impl Light<On> {
    pub const fn turn_off(self) -> Light<Off> {
        Light { state: PhantomData }
    }
}

impl Light<Off> {
    pub const fn new() -> Self {
        Self { state: PhantomData }
    }
    pub const fn turn_on(self) -> Light<On> {
        Light { state: PhantomData }
    }
}
