//! Layout Alignment Showcase

use tessera::{DimensionValue, Dp, Px, Renderer};
use tessera_basic_components::{
    alignment::{CrossAxisAlignment, MainAxisAlignment},
    column::{AsColumnItem, ColumnArgsBuilder, column},
    row::{AsRowItem, RowArgsBuilder, row},
    spacer::{SpacerArgs, spacer},
    surface::{SurfaceArgs, surface},
    text::{TextArgsBuilder, text},
};
use tessera_macros::tessera;

/// Create a small colored box
#[tessera]
fn small_box(text_content: &'static str, color: [f32; 4]) {
    surface(
        SurfaceArgs {
            color,
            corner_radius: 25.0,
            padding: Dp(8.0),
            width: Some(DimensionValue::Fixed(Px(40))),
            height: Some(DimensionValue::Fixed(Px(40))),
            ..Default::default()
        },
        None,
        move || {
            text(
                TextArgsBuilder::default()
                    .text(text_content.to_string())
                    .color([255, 255, 255])
                    .size(Dp(12.0))
                    .build()
                    .unwrap(),
            )
        },
    );
}

/// Create a demonstration row
#[tessera]
fn row_demo_line(title: &'static str, alignment: MainAxisAlignment) {
    column(
        ColumnArgsBuilder::default()
            .main_axis_alignment(MainAxisAlignment::Start)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .build()
            .unwrap(),
        [
            // Title
            (move || {
                text(
                    TextArgsBuilder::default()
                        .text(title.to_string())
                        .size(Dp(14.0))
                        .color([80, 80, 80])
                        .build()
                        .unwrap(),
                )
            })
            .into_column_item(),
            // Alignment Demo Container - Fixed Width，Visible Background Border
            (move || {
                surface(
                    SurfaceArgs {
                        color: [0.9, 0.9, 0.9, 1.0], // Gray background to see borders clearly
                        corner_radius: 25.0,
                        padding: Dp(10.0),
                        width: Some(DimensionValue::Fixed(Px(400))), // Sufficient Fixed Width
                        height: Some(DimensionValue::Fixed(Px(70))),
                        ..Default::default()
                    },
                    None,
                    move || {
                        row(
                            RowArgsBuilder::default()
                                .width(DimensionValue::Fill {
                                    min: None,
                                    max: None,
                                }) // row Fill Container Width
                                .height(DimensionValue::Wrap {
                                    min: None,
                                    max: None,
                                }) // row Height Adapts to Content
                                .main_axis_alignment(alignment) // Directly use different main axis alignments
                                .cross_axis_alignment(CrossAxisAlignment::Center)
                                .build()
                                .unwrap(),
                            [
                                (|| small_box("1", [0.2, 0.6, 0.9, 1.0])).into_row_item(),
                                (|| small_box("2", [0.9, 0.2, 0.2, 1.0])).into_row_item(),
                                (|| small_box("3", [0.2, 0.8, 0.3, 1.0])).into_row_item(),
                            ],
                        );
                    },
                );
            })
            .into_column_item(),
        ],
    );
}

/// Main App
#[tessera]
fn app() {
    surface(
        SurfaceArgs {
            color: [1.0, 1.0, 1.0, 1.0], // White Background
            padding: Dp(20.0),
            ..Default::default()
        },
        None,
        || {
            column(
                ColumnArgsBuilder::default()
                    .main_axis_alignment(MainAxisAlignment::Start)
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .build()
                    .unwrap(),
                [
                    // Main Title
                    Box::new(|| {
                        text(
                            TextArgsBuilder::default()
                                .text("Tessera Alignment Demo".to_string())
                                .size(Dp(24.0))
                                .color([40, 40, 40])
                                .build()
                                .unwrap(),
                        )
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    // Spacing
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(30)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    // row Alignment Demo Title
                    Box::new(|| {
                        text(
                            TextArgsBuilder::default()
                                .text("row Main Axis Alignment:".to_string())
                                .size(Dp(18.0))
                                .color([60, 60, 60])
                                .build()
                                .unwrap(),
                        )
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    // Spacing
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(15)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    // RowAlignment Demo
                    Box::new(|| row_demo_line("Start", MainAxisAlignment::Start))
                        as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(20)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| row_demo_line("Center", MainAxisAlignment::Center))
                        as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(20)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| row_demo_line("End", MainAxisAlignment::End))
                        as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(20)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| row_demo_line("SpaceEvenly", MainAxisAlignment::SpaceEvenly))
                        as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(20)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| row_demo_line("SpaceBetween", MainAxisAlignment::SpaceBetween))
                        as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| {
                        spacer(SpacerArgs {
                            width: DimensionValue::Fixed(Px(0)),
                            height: DimensionValue::Fixed(Px(20)),
                        })
                    }) as Box<dyn FnOnce() + Send + Sync>,
                    Box::new(|| row_demo_line("SpaceAround", MainAxisAlignment::SpaceAround))
                        as Box<dyn FnOnce() + Send + Sync>,
                ],
            );
        },
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Renderer::run(app, |app| {
        tessera_basic_components::pipelines::register_pipelines(app);
    })?;
    Ok(())
}
