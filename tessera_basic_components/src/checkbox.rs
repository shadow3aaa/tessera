use derive_builder::Builder;
use std::sync::Arc;
use tessera::{DimensionValue, Dp};
use tessera_macros::tessera;

use crate::{
    alignment::Alignment,
    boxed::{BoxedArgs, boxed_ui},
    surface::{SurfaceArgsBuilder, surface},
    text::{TextArgsBuilder, text},
};

/// Arguments for the `checkbox` component.
#[derive(Builder, Clone)]
#[builder(pattern = "owned")]
pub struct CheckboxArgs {
    #[builder(default)]
    pub checked: bool,

    #[builder(default = "Arc::new(|_| {})")]
    pub on_toggle: Arc<dyn Fn(bool) + Send + Sync>,

    #[builder(default = "Dp(24.0)")]
    pub size: Dp,

    #[builder(default = "[0.8, 0.8, 0.8, 1.0]")]
    pub color: [f32; 4],

    #[builder(default = "[0.6, 0.7, 0.9, 1.0]")]
    pub checked_color: [f32; 4],

    #[builder(default = "[119, 72, 146]")]
    pub checkmark_color: [u8; 3],

    #[builder(default = "4.0")]
    pub corner_radius: f32,

    #[builder(default)]
    pub hover_color: Option<[f32; 4]>,
}

impl std::fmt::Debug for CheckboxArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckboxArgs")
            .field("checked", &self.checked)
            .field("on_toggle", &"<callback>")
            .field("size", &self.size)
            .field("color", &self.color)
            .field("checked_color", &self.checked_color)
            .field("checkmark_color", &self.checkmark_color)
            .field("corner_radius", &self.corner_radius)
            .field("hover_color", &self.hover_color)
            .finish()
    }
}

impl Default for CheckboxArgs {
    fn default() -> Self {
        CheckboxArgsBuilder::default().build().unwrap()
    }
}

#[tessera]
pub fn checkbox(args: impl Into<CheckboxArgs>) {
    let args: CheckboxArgs = args.into();
    let on_click = {
        let on_toggle = args.on_toggle.clone();
        let checked = args.checked;
        Arc::new(move || {
            on_toggle(!checked);
        })
    };

    surface(
        SurfaceArgsBuilder::default()
            .width(DimensionValue::Fixed(args.size.to_px()))
            .height(DimensionValue::Fixed(args.size.to_px()))
            .color(if args.checked {
                args.checked_color
            } else {
                args.color
            })
            .hover_color(args.hover_color)
            .corner_radius(args.corner_radius)
            .on_click(Some(on_click))
            .build()
            .unwrap(),
        None,
        move || {
            if args.checked {
                surface(
                    SurfaceArgsBuilder::default()
                        .padding(Dp(2.0))
                        .color([0.0; 4])
                        .build()
                        .unwrap(),
                    None,
                    move || {
                        boxed_ui!(
                            BoxedArgs {
                                alignment: Alignment::Center,
                            },
                            move || text(
                                TextArgsBuilder::default()
                                    .text("✔".to_string())
                                    .color(args.checkmark_color)
                                    .size(Dp(args.size.0 * 0.7))
                                    .build()
                                    .unwrap()
                            )
                        );
                    },
                )
            }
        },
    );
}
