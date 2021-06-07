use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, Bindings}
};
use amethyst::ui::{RenderUi, UiBundle};

mod orbital;
mod systems;


use crate::orbital::Orbital;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    // We'll put the rest of the code here.
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::new().with_bindings_from_file(binding_path)?;

    //let ui_bundle = UiBundle::new();

    let game_data = DispatcherBuilder::default()
    .add_bundle(TransformBundle)
    .add_system(systems::VelocitySystem)//, "velocity_system", &[]
    .add_system(systems::GravitySystem)//, "gravity_system", &["velocity_system"]
    .add_bundle(input_bundle)
    //.add_bundle(ui_bundle)
    .add_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                    //.with_clear(ClearColor::new([0.0, 0.0, 0.0, 1.0])))
            
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
    );

    //let assets_dir = app_root.join("assets");
    //let mut game = Application::build(assets_dir, Orbital::default())?.build(())?;
    //game.run();
    Ok(())
}
