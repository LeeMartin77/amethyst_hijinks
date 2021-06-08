mod orbital;
mod systems;
mod bundle;

use amethyst::{
    assets::LoaderBundle,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::orbital::Orbital;

use crate::bundle::OrbitalBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::new().with_bindings_from_file(binding_path)?;


    let mut game_data = DispatcherBuilder::default();
        game_data.add_bundle(TransformBundle)
        .add_bundle(LoaderBundle)
        .add_bundle(OrbitalBundle)
        .add_bundle(input_bundle)
        .add_bundle(UiBundle::<u32>::default())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                        .with_clear(ClearColor {float32: [0.0, 0.0, 0.0, 1.0]}))
                
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
    );

    let assets_dir = app_root.join("assets");
    let game = Application::new(assets_dir, Orbital::default(), game_data)?;
    game.run();
    Ok(())
}
