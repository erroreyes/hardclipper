use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::*;
use std::sync::Arc;

use crate::HardclipperParams;

#[derive(Lens)]
struct Data {
    params: Arc<HardclipperParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 280))
}

pub(crate) fn create(
    params: Arc<HardclipperParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {

    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {

        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Error loading /src/style.css");

        // Fonts
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        // Params
        Data {
            params: params.clone(),
        }
        .build(cx);

        // Title, then controls
        VStack::new(cx, |cx| {
            
            // Title
            Label::new(cx, "CLIPPER")
                .font_family(vec![ FamilyOwned::Name(String::from(
                    assets::NOTO_SANS,
                ))])
                .font_size(30.0)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .child_left(Pixels(8.0))
                .child_right(Stretch(1.0))
                ;

            // Controls
            HStack::new(cx, |cx| {

                // Left side labels
                VStack::new(cx, |cx| {

                    Label::new(cx, "MODE")
                    .class("col-left-label")
                    ;
                    Label::new(cx, "INPUT")
                    .class("col-left-label")
                    ;
                    Label::new(cx, "CEILING")
                    .class("col-left-label")
                    ;
                    Label::new(cx, "OUTPUT")
                    .class("col-left-label")
                    ;
                    Label::new(cx, "DELTA")
                    .class("col-left-label")
                    ;

                })
                .width(Stretch(1.0))
                ;

                // Right side sliders
                VStack::new(cx, |cx| {
                    
                    ParamSlider::new(cx, Data::params, |params| &params.mode)
                    .background_color(Color::rgb(160, 193, 255))
                    ;
                    ParamSlider::new(cx, Data::params, |params| &params.input_gain)
                    .background_color(Color::rgb(160, 193, 255))
                    ;
                    ParamSlider::new(cx, Data::params, |params| &params.ceiling)
                    .background_color(Color::rgb(160, 193, 255))
                    ;
                    ParamSlider::new(cx, Data::params, |params| &params.output_gain)
                    .background_color(Color::rgb(160, 193, 255))
                    ;
                    ParamSlider::new(cx, Data::params, |params| &params.delta)
                    .background_color(Color::rgb(160, 193, 255))
                    ;

                })
                .width(Stretch(2.0))
                ;

            })
            ;
            
        })
        .space(Pixels(2.0))
        ;

        // Resize handle goes to the bottom of the list of widgets.
        ResizeHandle::new(cx);

    })
}