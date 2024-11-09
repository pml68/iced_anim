use super::Animate;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Arc;
use syntect::highlighting;

static THEMES: Lazy<highlighting::ThemeSet> = Lazy::new(highlighting::ThemeSet::load_defaults);

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    SolarizedDark,
    SolarizedLight,
    Base16Mocha,
    Base16OceanDark,
    Base16OceanLight,
    Base16Eighties,
    InspiredGitHub,
    Custom(Arc<highlighting::Theme>),
}

impl Theme {
    pub fn highlighter_theme(&self) -> &'static highlighting::Theme {
        match self {
            Theme::SolarizedDark => &THEMES.themes["Solarized (dark)"],
            Theme::SolarizedLight => &THEMES.themes["Solarized (light)"],
            Theme::Base16Mocha => &THEMES.themes["base16-mocha.dark"],
            Theme::Base16OceanDark => &THEMES.themes["base16-ocean.dark"],
            Theme::Base16OceanLight => &THEMES.themes["base16-ocean.light"],
            Theme::Base16Eighties => &THEMES.themes["base16-eighties.dark"],
            Theme::InspiredGitHub => &THEMES.themes["InspiredGitHub"],
            Theme::Custom(custom) => {
                static HIGHLIGHTER_THEME: OnceCell<highlighting::Theme> = OnceCell::new();
                HIGHLIGHTER_THEME.get_or_init(|| highlighting::Theme::clone(&custom))
            }
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::SolarizedDark => write!(f, "Solarized Dark"),
            Theme::SolarizedLight => write!(f, "Solarized Light"),
            Theme::Base16Mocha => write!(f, "Mocha"),
            Theme::Base16OceanDark => write!(f, "Ocean Dark"),
            Theme::Base16OceanLight => write!(f, "Ocean Light"),
            Theme::Base16Eighties => write!(f, "Eighties"),
            Theme::InspiredGitHub => write!(f, "Inspired GitHub"),
            Theme::Custom(custom) => write!(f, "{}", custom.name.clone().unwrap_or("".to_owned())),
        }
    }
}

impl Animate for Theme {
    fn components() -> usize {
        highlighting::Theme::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut theme = self.highlighter_theme().clone();
        theme.update(components);

        *self = Theme::Custom(Arc::new(theme));
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [self
            .highlighter_theme()
            .distance_to(&end.highlighter_theme())]
        .concat()
    }
}
