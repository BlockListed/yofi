use std::marker::PhantomData;

use font_kit::loaders::freetype::Font;
use font_kit::source::SystemSource;
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};

use super::{Drawable, Space};
use crate::Entry;

const ENTRY_HEIGHT: f32 = 25.;

pub struct ListView<'a, It> {
    items: It,
    selected_item: usize,
    font: Font,
    font_color: SolidSource,
    selected_font_color: SolidSource,
    _tparam: PhantomData<&'a ()>,
}

impl<It> ListView<'_, It> {
    pub fn new(items: It, selected_item: usize) -> Self {
        Self {
            items,
            selected_item,
            font: FONT.with(Clone::clone),
            font_color: SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0),
            selected_font_color: SolidSource::from_unpremultiplied_argb(0x90, 0x90, 0, 0xcc),
            _tparam: PhantomData,
        }
    }
}

std::thread_local! {
    static FONT: Font = SystemSource::new()
        .select_best_match(
            &[font_kit::family_name::FamilyName::SansSerif],
            &font_kit::properties::Properties::new(),
        )
        .unwrap()
        .load()
        .unwrap();
}

impl<'a, It> Drawable for ListView<'a, It>
where
    It: Iterator<Item = &'a Entry>,
{
    fn draw(self, dt: &mut DrawTarget, space: Space, point: Point) -> Space {
        for (i, item) in self
            .items
            .skip(self.selected_item.saturating_sub(3))
            .enumerate()
        {
            let y = point.y + 28. + (i as f32) * ENTRY_HEIGHT;
            if y + ENTRY_HEIGHT > space.height {
                break;
            }
            let pos = Point::new(point.x + 10., y);
            let color = if i == self.selected_item {
                self.selected_font_color
            } else {
                self.font_color
            };
            dt.draw_text(
                &self.font,
                24.,
                item.name.as_str(),
                pos,
                &Source::Solid(color),
                &DrawOptions::new(),
            );
        }

        space
    }
}