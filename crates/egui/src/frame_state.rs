use crate::{id::IdSet, *};
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug)]
pub struct TooltipFrameState {
    pub common_id: Id,
    pub rect: Rect,
    pub count: usize,
}

#[cfg(feature = "accesskit")]
#[derive(Clone)]
pub struct AccessKitFrameState {
    pub node_builders: IdMap<accesskit::NodeBuilder>,
    pub parent_stack: Vec<Id>,
}

#[derive(Clone)]
pub struct FrameState {
    pub used_ids: IdMap<Rect>,
    pub available_rect: Rect,
    pub unused_rect: Rect,
    pub used_by_panels: Rect,
    pub tooltip_state: Option<TooltipFrameState>,
    pub scroll_delta: Vec2,
    pub scroll_target: [Option<(RangeInclusive<f32>, Option<Align>)>; 2],
    #[cfg(feature = "accesskit")]
    pub accesskit_state: Option<AccessKitFrameState>,
    pub highlight_this_frame: IdSet,
    pub highlight_next_frame: IdSet,
}

impl Default for FrameState {
    fn default() -> Self {
        Self {
            used_ids: Default::default(),
            available_rect: Rect::NAN,
            unused_rect: Rect::NAN,
            used_by_panels: Rect::NAN,
            tooltip_state: None,
            scroll_delta: Vec2::ZERO,
            scroll_target: [None, None],
            #[cfg(feature = "accesskit")]
            accesskit_state: None,
            highlight_this_frame: Default::default(),
            highlight_next_frame: Default::default(),
        }
    }
}

impl FrameState {
    pub fn begin_frame(&mut self, input: &InputState) {
        self.used_ids.clear();
        self.available_rect = input.screen_rect();
        self.unused_rect = input.screen_rect();
        self.used_by_panels = Rect::NOTHING;
        self.tooltip_state = None;
        self.scroll_delta = input.scroll_delta;
        self.scroll_target = [None, None];
        #[cfg(feature = "accesskit")]
        {
            self.accesskit_state = None;
        }
        self.highlight_this_frame = self.highlight_next_frame.clone();
    }

    pub fn available_rect(&self) -> Rect {
        crate::egui_assert!(
            self.available_rect.is_finite(),
            "Called `available_rect()` before `Context::run()`"
        );
        self.available_rect
    }

    pub fn allocate_left_panel(&mut self, panel_rect: Rect) {
        crate::egui_assert!(
            panel_rect.min.distance(self.available_rect.min) < 0.1,
            "Mismatching left panel. You must not create a panel from within another panel."
        );
        self.available_rect.min.x = panel_rect.max.x;
        self.unused_rect.min.x = panel_rect.max.x;
        self.used_by_panels = self.used_by_panels.union(panel_rect);
    }

    pub fn allocate_right_panel(&mut self, panel_rect: Rect) {
        crate::egui_assert!(
            panel_rect.max.distance(self.available_rect.max) < 0.1,
            "Mismatching right panel. You must not create a panel from within another panel."
        );
        self.available_rect.max.x = panel_rect.min.x;
        self.unused_rect.max.x = panel_rect.min.x;
        self.used_by_panels = self.used_by_panels.union(panel_rect);
    }

    pub fn allocate_top_panel(&mut self, panel_rect: Rect) {
        crate::egui_assert!(
            panel_rect.min.distance(self.available_rect.min) < 0.1,
            "Mismatching top panel. You must not create a panel from within another panel."
        );
        self.available_rect.min.y = panel_rect.max.y;
        self.unused_rect.min.y = panel_rect.max.y;
        self.used_by_panels = self.used_by_panels.union(panel_rect);
    }

    pub fn allocate_bottom_panel(&mut self, panel_rect: Rect) {
        crate::egui_assert!(
            panel_rect.max.distance(self.available_rect.max) < 0.1,
            "Mismatching bottom panel. You must not create a panel from within another panel."
        );
        self.available_rect.max.y = panel_rect.min.y;
        self.unused_rect.max.y = panel_rect.min.y;
        self.used_by_panels = self.used_by_panels.union(panel_rect);
    }

    pub fn allocate_central_panel(&mut self, panel_rect: Rect) {
        self.unused_rect = Rect::NOTHING;
        self.used_by_panels = self.used_by_panels.union(panel_rect);
    }
}

        
            
       
