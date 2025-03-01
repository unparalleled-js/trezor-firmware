use crate::ui::{
    component::{Component, Event, EventCtx},
    geometry::{Alignment, Alignment2D, Grid, GridCellSpan, Insets, Offset, Rect, TOP_RIGHT},
};

pub struct GridPlaced<T> {
    inner: T,
    grid: Grid,
    cells: GridCellSpan,
}

impl<T> GridPlaced<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            grid: Grid::new(Rect::zero(), 0, 0),
            cells: GridCellSpan {
                from: (0, 0),
                to: (0, 0),
            },
        }
    }

    pub fn with_grid(mut self, rows: usize, cols: usize) -> Self {
        self.grid.rows = rows;
        self.grid.cols = cols;
        self
    }

    pub fn with_spacing(mut self, spacing: i16) -> Self {
        self.grid.spacing = spacing;
        self
    }

    pub fn with_row_col(mut self, row: usize, col: usize) -> Self {
        self.cells.from = (row, col);
        self.cells.to = (row, col);
        self
    }

    pub fn with_from_to(mut self, from: (usize, usize), to: (usize, usize)) -> Self {
        self.cells.from = from;
        self.cells.to = to;
        self
    }
}

impl<T> Component for GridPlaced<T>
where
    T: Component,
{
    type Msg = T::Msg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.grid.area = bounds;
        self.inner.place(self.grid.cells(self.cells))
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.inner.event(ctx, event)
    }

    fn paint(&mut self) {
        self.inner.paint()
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for GridPlaced<T>
where
    T: Component,
    T: crate::trace::Trace,
{
    fn trace(&self, d: &mut dyn crate::trace::Tracer) {
        d.open("GridPlaced");
        d.field("inner", &self.inner);
        d.close();
    }
}

pub struct FixedHeightBar<T> {
    inner: T,
    height: i16,
}

impl<T> FixedHeightBar<T> {
    pub const fn bottom(inner: T, height: i16) -> Self {
        Self { inner, height }
    }
}

impl<T> Component for FixedHeightBar<T>
where
    T: Component,
{
    type Msg = T::Msg;

    fn place(&mut self, bounds: Rect) -> Rect {
        let (_, bar) = bounds.split_bottom(self.height);
        self.inner.place(bar)
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.inner.event(ctx, event)
    }

    fn paint(&mut self) {
        self.inner.paint()
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for FixedHeightBar<T>
where
    T: Component,
    T: crate::trace::Trace,
{
    fn trace(&self, d: &mut dyn crate::trace::Tracer) {
        d.open("FixedHeightBar");
        d.field("inner", &self.inner);
        d.close();
    }
}

pub struct Floating<T> {
    inner: T,
    size: Offset,
    border: Offset,
    align: Alignment2D,
}

impl<T> Floating<T> {
    pub const fn new(size: Offset, border: Offset, align: Alignment2D, inner: T) -> Self {
        Self {
            inner,
            size,
            border,
            align,
        }
    }

    pub const fn top_right(side: i16, border: i16, inner: T) -> Self {
        let size = Offset::uniform(side);
        let border = Offset::uniform(border);
        Self::new(size, border, TOP_RIGHT, inner)
    }
}

impl<T> Component for Floating<T>
where
    T: Component,
{
    type Msg = T::Msg;

    fn place(&mut self, bounds: Rect) -> Rect {
        let mut border = self.border;
        let area = match self.align.0 {
            Alignment::Start => bounds.split_left(self.size.x).0,
            Alignment::Center => panic!("alignment not supported"),
            Alignment::End => {
                border.x = -border.x;
                bounds.split_right(self.size.x).1
            }
        };
        let area = match self.align.1 {
            Alignment::Start => area.split_top(self.size.y).0,
            Alignment::Center => panic!("alignment not supported"),
            Alignment::End => {
                border.y = -border.y;
                area.split_bottom(self.size.y).1
            }
        };
        self.inner.place(area.translate(border))
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.inner.event(ctx, event)
    }

    fn paint(&mut self) {
        self.inner.paint()
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for Floating<T>
where
    T: Component,
    T: crate::trace::Trace,
{
    fn trace(&self, d: &mut dyn crate::trace::Tracer) {
        d.open("Floating");
        d.field("inner", &self.inner);
        d.close();
    }
}

pub struct VSplit<T, U> {
    first: T,
    second: U,
    width: i16,
    spacing: i16,
}

impl<T, U> VSplit<T, U> {
    pub const fn new(width: i16, spacing: i16, first: T, second: U) -> Self {
        Self {
            first,
            second,
            width,
            spacing,
        }
    }
}

impl<M, T, U> Component for VSplit<T, U>
where
    T: Component<Msg = M>,
    U: Component<Msg = M>,
{
    type Msg = M;

    fn place(&mut self, bounds: Rect) -> Rect {
        let (left, right) = bounds.split_left(self.width);
        let right = right.inset(Insets::left(self.spacing));
        self.first.place(left);
        self.second.place(right);
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.first
            .event(ctx, event)
            .or_else(|| self.second.event(ctx, event))
    }

    fn paint(&mut self) {
        self.first.paint();
        self.second.paint();
    }
}

#[cfg(feature = "ui_debug")]
impl<T, U> crate::trace::Trace for VSplit<T, U>
where
    T: Component + crate::trace::Trace,
    U: Component + crate::trace::Trace,
{
    fn trace(&self, d: &mut dyn crate::trace::Tracer) {
        d.open("VSplit");
        d.field("first", &self.first);
        d.field("second", &self.second);
        d.close();
    }
}
