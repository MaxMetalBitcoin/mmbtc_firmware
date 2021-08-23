use embedded_graphics::{
    fonts::{Font6x8, Text},
    mock_display::MockDisplay,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    text_style, DrawTarget,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use state_mgmt::MMState;

const PX_WIDTH: u32 = 296;
const PX_HEIGHT: u32 = 152;

fn main() -> Result<(), core::convert::Infallible> {
    let mut state = MMState::new();

    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(PX_WIDTH, PX_HEIGHT));

    display = state.render(display).unwrap();

    // // let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    // // let line_style = Line::new(Point::new())
    // let text_style = text_style!(
    //     font = Font6x8,
    //     text_color = BinaryColor::On,
    //     background_color = BinaryColor::Off,
    // );
    // // let text_style = MonoTextStyle::new(&Font6x8, BinaryColor::On);

    // // Circle::new(Point::new(72, 8), 48)
    // //     .into_styled(line_style)
    // //     .draw(&mut display)?;

    // // Line::new(Point::new(48, 16), Point::new(8, 16))
    // //     .into_styled(line_style)
    // //     .draw(&mut display)?;

    // // Line::new(Point::new(48, 16), Point::new(64, 32))
    // //     .into_styled(line_style)
    // //     .draw(&mut display)?;

    // Rectangle::new(Point::new(79, 15), Point::new(34, 34))
    //     .into_styled(line_style)
    //     .draw(&mut display)?;

    // Text::new("Hello World!", Point::new(5, 5))
    //     .into_styled(text_style)
    //     .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
