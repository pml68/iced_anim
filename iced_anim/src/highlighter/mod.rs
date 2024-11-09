pub mod highlighter;
pub mod theme;

pub use highlighter::{Highlight, Highlighter, Settings};
pub use theme::Theme;

use super::Animate;
use syntect::highlighting;

impl Animate for u8 {
    fn components() -> usize {
        1
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        *self += components.next().unwrap() as u8;
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        vec![*self as f32 - *end as f32]
    }
}

impl Animate for highlighting::Color {
    fn components() -> usize {
        4
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.r =
            ((self.r as f32 / 255.0 + components.next().unwrap()).clamp(0.0, 1.0) * 255.0) as u8;
        self.g =
            ((self.g as f32 / 255.0 + components.next().unwrap()).clamp(0.0, 1.0) * 255.0) as u8;
        self.b =
            ((self.b as f32 / 255.0 + components.next().unwrap()).clamp(0.0, 1.0) * 255.0) as u8;
        self.a =
            ((self.a as f32 / 255.0 + components.next().unwrap()).clamp(0.0, 1.0) * 255.0) as u8;
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.r.distance_to(&end.r),
            self.g.distance_to(&end.g),
            self.b.distance_to(&end.b),
            self.a.distance_to(&end.a),
        ]
        .concat()
    }
}

impl Animate for highlighting::StyleModifier {
    fn components() -> usize {
        highlighting::Color::components() * 2
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.foreground.update(components);
        self.background.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.foreground.distance_to(&end.foreground),
            self.background.distance_to(&end.background),
        ]
        .concat()
    }
}

impl Animate for highlighting::ThemeItem {
    fn components() -> usize {
        highlighting::StyleModifier::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.style.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.style.distance_to(&end.style)
    }
}

impl Animate for highlighting::Theme {
    fn components() -> usize {
        highlighting::ThemeItem::components() * 150
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        for item in self.scopes.iter_mut() {
            item.update(components);
        }
        let extra =
            Self::components() - self.scopes.len() * highlighting::ThemeItem::components() - 1;
        components.nth(extra);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        let mut distance: Vec<f32> = self
            .scopes
            .iter()
            .zip(end.scopes.iter().take(self.scopes.len()))
            .flat_map(|(start, end)| start.distance_to(end))
            .collect();
        distance.resize(Self::components(), 0.0);
        distance
    }
}
