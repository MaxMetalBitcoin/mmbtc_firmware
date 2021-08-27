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

use state_mgmt::{mm_state_action::MMStateAction, MMState};

const PX_WIDTH: u32 = 296;
const PX_HEIGHT: u32 = 152;

fn main() -> Result<(), core::convert::Infallible> {
    let mut state = MMState::new();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .build();
    let mut window = Window::new("Hello World", &output_settings);
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(PX_WIDTH, PX_HEIGHT));

    window.update(&display);
    display = state.render(display).unwrap();

    'running: loop {
        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        sdl2::keyboard::Keycode::Up => {
                            println!("{:?}", state);

                            let did_update = state.updateState(
                                state_mgmt::prompt_screen_state::mm_state_action::MMStateAction::Up,
                            );
                            println!("{:?}", state);

                            if did_update {
                                display.clear(BinaryColor::Off);
                                display = state.render(display).unwrap();
                            }
                        }
                        sdl2::keyboard::Keycode::Down => {
                            println!("{:?}", state);

                            let did_update = state.updateState(state_mgmt::prompt_screen_state::mm_state_action::MMStateAction::Down);
                            println!("{:?}", state);

                            if did_update {
                                display.clear(BinaryColor::Off);
                                display = state.render(display).unwrap();
                            }
                        }
                        sdl2::keyboard::Keycode::Left => {
                            println!("{:?}", state);

                            let did_update = state.updateState(state_mgmt::prompt_screen_state::mm_state_action::MMStateAction::Left);
                            println!("{:?}", state);

                            if did_update {
                                display.clear(BinaryColor::Off);
                                display = state.render(display).unwrap();
                            }
                        }
                        sdl2::keyboard::Keycode::Right => {
                            println!("{:?}", state);

                            let did_update = state.updateState(state_mgmt::prompt_screen_state::mm_state_action::MMStateAction::Right);
                            println!("{:?}", state);

                            if did_update {
                                display.clear(BinaryColor::Off);
                                display = state.render(display).unwrap();
                            }
                        }
                        sdl2::keyboard::Keycode::Return => {
                            println!("{:?}", state);

                            let did_update = state.updateState(state_mgmt::prompt_screen_state::mm_state_action::MMStateAction::Enter);
                            println!("{:?}", state);

                            if did_update {
                                display.clear(BinaryColor::Off);
                                display = state.render(display).unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

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

    Ok(())
}
