use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings}
};

mod orbital;
mod systems;

use crate::orbital::Orbital;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    // We'll put the rest of the code here.
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with(systems::VelocitySystem, "velocity_system", &[])
    .with(systems::GravitySystem, "gravity_system", &["velocity_system"])
    .with_bundle(input_bundle)?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
    )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Orbital::default(), game_data)?;
    game.run();
    Ok(())
}
