
use glfw::{
  Action, Key, Modifiers, MouseButton, WindowEvent,
};

/// A type used to pass “inputs” to examples.
#[derive(Clone, Debug)]
pub enum InputAction {
    /// Quit the application.
    Quit,

    /// Primary action. Typically used to fire a weapon, select an object, etc. Typically used along with a position on
    /// screen.
    PrimaryPressed,

    /// Primary action. Typically used to fire a weapon, select an object, etc. Typically used along with a position on
    /// screen.
    PrimaryReleased,

    /// Main action. Typically used to switch an effect on and off or to cycle through it.
    MainToggle,

    /// Auxiliary action. Often used to showcase / toggle smaller parts of a bigger effect.
    AuxiliaryToggle,

    /// Forward direction. Typically used to move forward.
    Forward,

    /// Forward direction. Typically used to move backward.
    Backward,

    /// Left direction. Typically used to move something left, move left, etc.
    Left,

    /// Right direction. Typically used to move something right, move right, etc.
    Right,

    /// Up direction. Typically used to move something up, go up, etc.
    Up,

    /// Down direction. Typically used to move something down, go down, etc.
    Down,

    /// Cursor moved. The cursor is a 2D-coordinate pointer on the screen that can be actioned by moving a stick, a mouse,
    /// etc.
    CursorMoved { x: f32, y: f32 },

    /// Framebuffer size changed.
    Resized { width: u32, height: u32 },

    /// Vertical scrolling.
    VScroll { amount: f32 },
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LoopFeedback<T> {
    Continue(T),
    Exit,
}

pub fn adapt_events(event: WindowEvent) -> Option<InputAction> {
  match event {
      WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
          Some(InputAction::Quit)
      }

      WindowEvent::Key(Key::Space, _, Action::Release, mods) => {
          if mods.is_empty() {
              Some(InputAction::MainToggle)
          } else if mods == Modifiers::Shift {
              Some(InputAction::AuxiliaryToggle)
          } else {
              None
          }
      }

      WindowEvent::Key(key, _, Action::Press, _)
      | WindowEvent::Key(key, _, Action::Repeat, _) => {
          log::debug!("key press: {:?}", key);
          match key {
              Key::A => Some(InputAction::Left),
              Key::D => Some(InputAction::Right),
              Key::W => Some(InputAction::Forward),
              Key::S => Some(InputAction::Backward),
              Key::F => Some(InputAction::Up),
              Key::R => Some(InputAction::Down),
              _ => None,
          }
      }

      WindowEvent::MouseButton(MouseButton::Button1, action, _) => match action {
          Action::Press => Some(InputAction::PrimaryPressed),
          Action::Release => Some(InputAction::PrimaryReleased),
          _ => None,
      },

      WindowEvent::CursorPos(x, y) => Some(InputAction::CursorMoved {
          x: x as _,
          y: y as _,
      }),

      WindowEvent::FramebufferSize(width, height) => Some(InputAction::Resized {
          width: width as _,
          height: height as _,
      }),

      WindowEvent::Scroll(_, amount) => Some(InputAction::VScroll {
          amount: amount as f32,
      }),

      _ => None,
  }
}
