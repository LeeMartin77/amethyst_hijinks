use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
    prelude::*,
    renderer::{sprite::Sprites, SpriteRender, SpriteSheet},
};

pub struct PlayerCharacter {
    pub idle_spritesheet: Handle<SpriteSheet>,
}

impl PlayerCharacter {
    pub fn new(resources: &mut Resources) -> PlayerCharacter {
        let idle_spritesheet = load_sprite_sheets(resources);
        PlayerCharacter { 
            idle_spritesheet: idle_spritesheet.clone()
        }
    }
}


fn load_sprite_sheets(resources: &mut Resources) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = resources.get::<DefaultLoader>().unwrap();
        loader.load("texture/metroidvania_sprites/herochar/herochar_idle_anim_strip_4.png")
    };

    let loader = resources.get::<DefaultLoader>().unwrap();
    let sprites: Handle<Sprites> = loader.load(
        "texture/metroidvania_sprites/herochar/herochar_idle_anim_strip_4.ron",
    );
    let sheet = SpriteSheet {
        texture: texture_handle,
        sprites,
    };
    let q = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
    loader.load_from_data(sheet, (), &q)
}