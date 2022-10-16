use color_eyre::eyre::Result;
use dynamic_arena::DynamicArena;
use maplit::hashmap;
use r3gl_app::{state::State, screen::{egui::EGuiScreen, taiko::TaikoScreen}, identifier::Identifier};
use wcore::{app::App, graphics::context::Context, screen::Action};
use winit::event::{VirtualKeyCode, ModifiersState};
use str_macro::str;

fn main() -> Result<()> {
	color_eyre::install()?;
    env_logger::init();

    let app = App {
        title: str!("r3gl"),
        width: 1200,
        height: 800,
        screens: Default::default(),
        bindings: Default::default(),
    };

    let arena = DynamicArena::new();
    app.run(State::new(), |app: &mut App<State, Identifier>, graphics: &mut Context| {
        (|| -> Result<()> {
            app.screens.push(arena.alloc(TaikoScreen::new(graphics)?));
            app.screens.push(arena.alloc(EGuiScreen::new(graphics)?));
            app.bindings.insert(Identifier::Editor, hashmap! {
                (VirtualKeyCode::Space, ModifiersState::empty()) => Action::new(
                    str!("Play/Pause"),
                    str!("Starts or stops the current song"),
                    |state: &mut State| {
                        state.editor.toggle_paused();
                    }
                ),
            });
            return Ok(());
        })().unwrap();
    });

	return Ok(());
}
