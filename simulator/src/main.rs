use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, DrawTarget};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use state_mgmt::{mm_state_action, MMState};

const PX_WIDTH: u32 = 296;
const PX_HEIGHT: u32 = 152;

fn main() -> Result<(), core::convert::Infallible> {
    let mut state = MMState::new();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .build();
    let mut window = Window::new("Max Metal Bitcoin HWW", &output_settings);
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(PX_WIDTH, PX_HEIGHT));

    window.update(&display);
    display = state.render(display).unwrap();

    // Uses a system described by: event/interrupt -> updateState -> effects
    // where event/interrupt is user input or software/hardware interrupt
    //   -interrupts should "lock" in such a way that another interrupt can't
    //   run at the same time. Ie: 1 action goes thru the whole system, before
    //   next action can be processed. This could cause an issue for holding down a
    //   down arrow perhaps, but other fixes may be possible for this.
    // updateState is a function of currentState + action => newState
    //   - it should only accept certain actions for what the currentState is
    //   in the case that the no change is made to state, or a no op occurs,
    //   no change is made to the display or other periphs
    // effects is based on the newState and did_update being true. In that case,
    // the screen is updated, and other periphs may need to be called, such as write
    // to sd card, etc.
    'running: loop {
        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    sdl2::keyboard::Keycode::Up => {
                        println!("{:?}", state);

                        let did_update = state.update_state(mm_state_action::MMStateAction::Up);
                        println!("{:?}", state);

                        if did_update {
                            display.clear(BinaryColor::Off).unwrap();
                            display = state.render(display).unwrap();
                        }
                    }
                    sdl2::keyboard::Keycode::Down => {
                        println!("{:?}", state);

                        let did_update = state.update_state(mm_state_action::MMStateAction::Down);
                        println!("{:?}", state);

                        if did_update {
                            display.clear(BinaryColor::Off).unwrap();
                            display = state.render(display).unwrap();
                        }
                    }
                    sdl2::keyboard::Keycode::Left => {
                        println!("{:?}", state);

                        let did_update = state.update_state(mm_state_action::MMStateAction::Left);
                        println!("{:?}", state);

                        if did_update {
                            display.clear(BinaryColor::Off).unwrap();
                            display = state.render(display).unwrap();
                        }
                    }
                    sdl2::keyboard::Keycode::Right => {
                        println!("{:?}", state);

                        let did_update = state.update_state(mm_state_action::MMStateAction::Right);
                        println!("{:?}", state);

                        if did_update {
                            display.clear(BinaryColor::Off).unwrap();
                            display = state.render(display).unwrap();
                        }
                    }
                    sdl2::keyboard::Keycode::Return => {
                        println!("{:?}", state);

                        let did_update = state.update_state(mm_state_action::MMStateAction::Enter);
                        println!("{:?}", state);

                        if did_update {
                            display.clear(BinaryColor::Off).unwrap();
                            display = state.render(display).unwrap();
                        }
                    }
                    _ => {}
                },
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
