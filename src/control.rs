use sfml::graphics::RenderWindow;
use sfml::window::{Event, Key};

pub struct ButtonState {
    pub held    : bool,
    pub press   : bool,
    pub release : bool

}

impl ButtonState {
    fn default() -> ButtonState {
        return ButtonState {
            held    : false,
            press   : false,
            release : false,
        }
    }
}

pub struct ControllerState {
    pub quit            : ButtonState,
    pub move_left       : ButtonState,
    pub move_right      : ButtonState,
    pub move_up         : ButtonState,
    pub move_down       : ButtonState,
    pub move_forward    : ButtonState,
    pub move_backward   : ButtonState,
    pub look_left       : ButtonState,
    pub look_right      : ButtonState,
    pub look_up         : ButtonState,
    pub look_down       : ButtonState,
    pub look_cw         : ButtonState,
    pub look_ccw        : ButtonState
}




impl ControllerState {
    pub fn default() -> ControllerState {
        return ControllerState {
            quit            : ButtonState::default(),
            move_left       : ButtonState::default(),
            move_right      : ButtonState::default(),
            move_up         : ButtonState::default(),
            move_down       : ButtonState::default(),
            move_forward    : ButtonState::default(),
            move_backward   : ButtonState::default(),
            look_left       : ButtonState::default(),
            look_right      : ButtonState::default(),
            look_up         : ButtonState::default(),
            look_down       : ButtonState::default(),
            look_cw         : ButtonState::default(),
            look_ccw        : ButtonState::default(),
        }
    }

    pub fn update(&mut self, window : &mut RenderWindow) {
        macro_rules! event_logic {
            ($(($name:ident, $key:pat)),*) => {
                $(
                    self.$name.press = false;
                    self.$name.release = false;
                )*
                while let Some(event) = window.poll_event() {
                    match event {
                         // Resolve key presses and releases
                        Event::Closed => {
                            self.quit.press = true;
                            self.quit.held = true;
                        },
                        Event::KeyReleased {code, ..} => {
                            match code {
                                $(
                                    $key => {
                                        self.$name.held = false;
                                        self.$name.release = true;
                                    },
                                )*
                                _ => {}
                            }
                         },
                         Event::KeyPressed {code, ..} => {
                            match code {
                                $(
                                    $key => {
                                        self.$name.held = true;
                                        self.$name.press = true;
                                    },
                                )*
                                _ => {}
                            }
                        },
                         _ => {}
                    }
                }
            }
        }

        event_logic!(
                (move_forward, Key::W),
                (move_left, Key::A),
                (move_backward, Key::S),
                (move_right, Key::D),
                (move_up, Key::R),
                (move_down, Key::F),
                (quit, Key::Q),
                (look_left, Key::LEFT),
                (look_right, Key::RIGHT),
                (look_up, Key::UP),
                (look_down, Key::DOWN),
                (look_cw, Key::PAGEDOWN),
                (look_ccw, Key::PAGEUP)
        );
    }
}
