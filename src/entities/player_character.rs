use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
    prelude::*,
    renderer::{sprite::Sprites, SpriteRender, SpriteSheet},
};

pub struct PlayerCharacter {
    pub spritesheet: Handle<SpriteSheet>,
}

impl PlayerCharacter {
    pub fn new(resources: &mut Resources) -> PlayerCharacter {
        let spritesheet = load_sprite_sheets(resources);
        PlayerCharacter { 
            spritesheet: spritesheet.clone()
        }
    }
}


fn load_sprite_sheets(resources: &mut Resources) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = resources.get::<DefaultLoader>().unwrap();
        loader.load("texture/metroidvania_sprites/herochar/herochar_spritesheet.png")
    };

    let loader = resources.get::<DefaultLoader>().unwrap();
    let sprites: Handle<Sprites> = loader.load(
        "texture/metroidvania_sprites/herochar/herochar_spritesheet.ron",
    );
    let sheet = SpriteSheet {
        texture: texture_handle,
        sprites,
    };
    let q = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
    loader.load_from_data(sheet, (), &q)
}