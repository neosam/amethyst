#![allow(clippy::default_trait_access)]
//! GPU POD data types.
use amethyst_assets::{AssetStorage, Handle};
use amethyst_core::math::Point3;
use amethyst_rendy::{
    rendy::{
        hal::format::Format,
        mesh::{AsVertex, VertexFormat},
    },
    resources::Tint as TintComponent,
    sprite::SpriteSheet,
    Texture,
};
use glsl_layout::*;

/// `TileMapArgs`
/// ```glsl,ignore
/// uniform TileMapArgs {
///    uniform mat4 proj;
///    uniform mat4 view;
///    uniform mat4 map_coordinate_transform;
///    uniform mat4 map_transform;
/// };
/// ```
#[derive(Clone, Copy, Debug, AsStd140)]
#[repr(C, align(16))]
pub struct TileMapArgs {
    /// Projection matrix
    pub proj: mat4,
    /// View matrix
    pub view: mat4,
    /// Projection matrix
    pub map_coordinate_transform: mat4,
    /// View matrix
    pub map_transform: mat4,
    /// Sprite Dimensions. Because we assume tiles are uniform for a map, we can store these here.
    pub sprite_dimensions: vec2,
}

/// Tile Vertex Data
/// ```glsl,ignore
/// vec2 dir_x;
/// vec2 dir_y;
/// vec2 pos;
/// vec2 u_offset;
/// vec2 v_offset;
/// float depth;
/// vec4 tint;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub struct TileArgs {
    /// Upper-left coordinate of the sprite in the spritesheet
    pub u_offset: vec2,
    /// Bottom-right coordinate of the sprite in the spritesheet
    pub v_offset: vec2,
    /// Tint for this this sprite
    pub tint: vec4,
    /// Tile coordinate
    pub tile_coordinate: vec3,
}

impl AsVertex for TileArgs {
    fn vertex() -> VertexFormat {
        VertexFormat::new((
            (Format::Rg32Sfloat, "u_offset"),
            (Format::Rg32Sfloat, "v_offset"),
            (Format::Rgba32Sfloat, "tint"),
            (Format::Rgb32Sfloat, "tile_coordinate"),
        ))
    }
}

impl TileArgs {
    #[allow(clippy::cast_precision_loss)]
    /// Extracts POD vertex data from the provided storages for a sprite.
    ///
    /// # Arguments
    /// * `tex_storage` - `Texture` Storage
    /// * `sprite_storage` - `SpriteSheet` Storage
    /// * `sprite_render` - `SpriteRender` component reference
    /// * `transform` - 'Transform' component reference
    pub fn from_data<'a>(
        tex_storage: &AssetStorage<Texture>,
        sprite_sheet: &'a SpriteSheet,
        sprite_number: usize,
        tint: Option<&TintComponent>,
        tile_coordinate: &Point3<u32>,
    ) -> Option<(Self, &'a Handle<Texture>)> {
        if !tex_storage.contains(&sprite_sheet.texture) {
            return None;
        }

        let sprite = &sprite_sheet.sprites[sprite_number];

        Some((
            Self {
                u_offset: [sprite.tex_coords.left, sprite.tex_coords.right].into(),
                v_offset: [sprite.tex_coords.top, sprite.tex_coords.bottom].into(),
                tint: tint.map_or([1.0; 4].into(), |t| {
                    let (r, g, b, a) = t.0.into_components();
                    [r, g, b, a].into()
                }),
                tile_coordinate: [
                    tile_coordinate.x as f32,
                    tile_coordinate.y as f32,
                    tile_coordinate.z as f32,
                ]
                .into(),
            },
            &sprite_sheet.texture,
        ))
    }
}
