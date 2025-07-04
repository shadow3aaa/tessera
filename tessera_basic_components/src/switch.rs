use derive_builder::Builder;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tessera::{
    ComputedData, Constraint, CursorEventContent, DimensionValue, Dp, PressKeyEventType, PxPosition,
};
use tessera_macros::tessera;

use crate::{
    pipelines::ShapeCommand,
    surface::{SurfaceArgsBuilder, surface},
};

const ANIMATION_DURATION: Duration = Duration::from_millis(150);

/// State for the `switch` component, handling animation.
pub struct SwitchState {
    pub checked: bool,
    progress: Mutex<f32>,
    last_toggle_time: Mutex<Option<Instant>>,
}

impl SwitchState {
    pub fn new(initial_state: bool) -> Self {
        Self {
            checked: initial_state,
            progress: Mutex::new(if initial_state { 1.0 } else { 0.0 }),
            last_toggle_time: Mutex::new(None),
        }
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
        *self.last_toggle_time.lock() = Some(Instant::now());
    }
}

/// Arguments for the `switch` component.
#[derive(Builder, Clone)]
#[builder(pattern = "owned")]
pub struct SwitchArgs {
    #[builder(default)]
    pub state: Option<Arc<Mutex<SwitchState>>>,

    #[builder(default = "false")]
    pub checked: bool,

    #[builder(default = "Arc::new(|_| {})")]
    pub on_toggle: Arc<dyn Fn(bool) + Send + Sync>,

    #[builder(default = "Dp(52.0)")]
    pub width: Dp,

    #[builder(default = "Dp(32.0)")]
    pub height: Dp,

    #[builder(default = "[0.8, 0.8, 0.8, 1.0]")]
    pub track_color: [f32; 4],

    #[builder(default = "[0.6, 0.7, 0.9, 1.0]")]
    pub track_checked_color: [f32; 4],

    #[builder(default = "[1.0, 1.0, 1.0, 1.0]")]
    pub thumb_color: [f32; 4],

    #[builder(default = "Dp(3.0)")]
    pub thumb_padding: Dp,
}

impl std::fmt::Debug for SwitchArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchArgs")
            .field("state", &self.state.is_some())
            .field("checked", &self.checked)
            .field("on_toggle", &"<callback>")
            // ... other fields
            .finish()
    }
}

#[tessera]
pub fn switch(args: impl Into<SwitchArgs>) {
    let args: SwitchArgs = args.into();
    let thumb_size = Dp(args.height.0 - (args.thumb_padding.0 * 2.0));

    surface(
        SurfaceArgsBuilder::default()
            .width(DimensionValue::Fixed(thumb_size.to_px()))
            .height(DimensionValue::Fixed(thumb_size.to_px()))
            .color(args.thumb_color)
            .corner_radius(thumb_size.0 as f32 / 2.0)
            .build()
            .unwrap(),
        None,
        || {},
    );

    let on_toggle = args.on_toggle.clone();
    let state = args.state.clone();
    let checked = args.checked;

    state_handler(Box::new(move |input| {
        if let Some(state) = &state {
            let state = state.lock();
            let mut progress = state.progress.lock();

            if let Some(last_toggle_time) = *state.last_toggle_time.lock() {
                let elapsed = last_toggle_time.elapsed();
                let animation_fraction =
                    (elapsed.as_secs_f32() / ANIMATION_DURATION.as_secs_f32()).min(1.0);

                *progress = if state.checked {
                    animation_fraction
                } else {
                    1.0 - animation_fraction
                };
            }
        }

        let clicks = input
            .cursor_events
            .iter()
            .filter(|e| {
                matches!(
                    &e.content,
                    CursorEventContent::Pressed(PressKeyEventType::Left)
                )
            })
            .count();
        if clicks > 0 {
            // The component's only job is to report the intent to toggle.
            // The parent component is responsible for actually changing the state.
            on_toggle(!checked);
        }
    }));

    measure(Box::new(move |input| {
        let thumb_id = input.children_ids[0];
        let thumb_constraint = Constraint::new(
            DimensionValue::Wrap {
                min: None,
                max: None,
            },
            DimensionValue::Wrap {
                min: None,
                max: None,
            },
        );
        let thumb_size =
            tessera::measure_node(thumb_id, &thumb_constraint, input.tree, input.metadatas)?;

        let self_width_px = args.width.to_px();
        let self_height_px = args.height.to_px();
        let thumb_padding_px = args.thumb_padding.to_px();

        let progress = args
            .state
            .as_ref()
            .map(|s| *s.lock().progress.lock())
            .unwrap_or(if args.checked { 1.0 } else { 0.0 });

        let start_x = thumb_padding_px;
        let end_x = self_width_px - thumb_size.width - thumb_padding_px;
        let thumb_x = start_x.0 as f32 + (end_x.0 - start_x.0) as f32 * progress;

        let thumb_y = (self_height_px - thumb_size.height) / 2;

        tessera::place_node(
            thumb_id,
            PxPosition::new(tessera::Px(thumb_x as i32), thumb_y),
            input.metadatas,
        );

        let track_color = if args.checked {
            args.track_checked_color
        } else {
            args.track_color
        };
        let track_command = ShapeCommand::Rect {
            color: track_color,
            corner_radius: (self_height_px.0 as f32) / 2.0,
            shadow: None,
        };
        if let Some(mut metadata) = input.metadatas.get_mut(&input.current_node_id) {
            metadata.basic_drawable = Some(Box::new(track_command));
        }

        Ok(ComputedData {
            width: self_width_px,
            height: self_height_px,
        })
    }));
}
