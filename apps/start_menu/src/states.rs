use bevy::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum StartMenu {
    #[default]
    Main,
    Settings,
    Out
}

impl StartMenu {
    pub fn is_main(&self) -> bool {
        match self {
            Self::Main => true,
            _ => false
        }
    }

    pub fn is_settings(&self) -> bool {
        match self {
            Self::Settings => true,
            _ => false
        }
    }

    pub fn is_out(&self) -> bool {
        match self {
            Self::Out => true,
            _ => false
        }
    }
}

impl StartMenu {
    pub fn set_main(&mut self) {
        *self = Self::Main; 
    }

    pub fn set_settings(&mut self) {
        *self = Self::Settings; 
    }

    pub fn set_out(&mut self) {
        *self = Self::Out; 
    }
}