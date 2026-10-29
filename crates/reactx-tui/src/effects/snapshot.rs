//! Captured cells used to replay a node while it animates out.

use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::Rect;

/// A copy of the painted cells of one node rectangle.
#[derive(Clone)]
pub(crate) struct Snapshot {
    area: Rect,
    cells: Vec<Cell>,
}

impl Snapshot {
    /// Copy the cells of `rect` from `buf`, clipped to the buffer.
    pub(crate) fn capture(buf: &Buffer, rect: Rect) -> Option<Self> {
        let area = rect.intersection(buf.area);
        if area.is_empty() {
            return None;
        }
        let mut cells = Vec::with_capacity(area.width as usize * area.height as usize);
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                cells.push(buf[(x, y)].clone());
            }
        }
        Some(Self { area, cells })
    }

    /// Copy the captured cells back into `buf` and return the area written.
    /// The area shrinks when the terminal got smaller since the capture.
    pub(crate) fn restore(&self, buf: &mut Buffer) -> Option<Rect> {
        let area = self.area.intersection(buf.area);
        if area.is_empty() {
            return None;
        }
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let index = (y - self.area.y) as usize * self.area.width as usize
                    + (x - self.area.x) as usize;
                let Some(cell) = self.cells.get(index) else {
                    continue;
                };
                if let Some(target) = buf.cell_mut((x, y)) {
                    *target = cell.clone();
                }
            }
        }
        Some(area)
    }
}
