use super::Animate;
use once_cell::sync::Lazy;
use std::sync::Arc;
use syntect::highlighting;

static THEMES: Lazy<highlighting::ThemeSet> = Lazy::new(highlighting::ThemeSet::load_defaults);

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    SolarizedDark,
    Base16Mocha,
    Base16Ocean,
    Base16Eighties,
    InspiredGitHub,
    Custom(Arc<highlighting::Theme>),
}

impl Theme {
    pub fn highlighter_theme(&self) -> highlighting::Theme {
        match self {
            Theme::SolarizedDark => THEMES.themes["Solarized (dark)"].clone(),
            Theme::Base16Mocha => THEMES.themes["base16-mocha.dark"].clone(),
            Theme::Base16Ocean => THEMES.themes["base16-ocean.dark"].clone(),
            Theme::Base16Eighties => THEMES.themes["base16-eighties.dark"].clone(),
            Theme::InspiredGitHub => THEMES.themes["InspiredGitHub"].clone(),
            Theme::Custom(custom) => highlighting::Theme::clone(&custom),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::SolarizedDark => write!(f, "Solarized Dark"),
            Theme::Base16Mocha => write!(f, "Mocha"),
            Theme::Base16Ocean => write!(f, "Ocean"),
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
        let mut theme = self.highlighter_theme();
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
