use std::collections::BTreeMap;

use shalrath::repr::{
    Brush, BrushPlane, Brushes, Entity, Extension, Properties, TextureOffset, Triangle,
};

use crate::{brush::BrushId, entity::EntityId, face::FaceId, texture::TextureId, Vector2};

/// Struct-of-arrays representation of a [`shalrath::repr::Map`]
#[derive(Debug, Default, Clone)]
pub struct GeoMap {
    pub entities: Vec<EntityId>,
    pub brushes: Vec<BrushId>,
    pub faces: Vec<FaceId>,

    pub textures: BTreeMap<TextureId, String>,

    pub entity_properties: BTreeMap<EntityId, Properties>,
    pub entity_brushes: BTreeMap<EntityId, Vec<BrushId>>,
    pub point_entities: Vec<EntityId>,

    pub brush_faces: BTreeMap<BrushId, Vec<FaceId>>,

    pub face_planes: BTreeMap<FaceId, Triangle>,
    pub face_textures: BTreeMap<FaceId, TextureId>,
    pub face_offsets: BTreeMap<FaceId, TextureOffset>,
    pub face_angles: BTreeMap<FaceId, f32>,
    pub face_scales: BTreeMap<FaceId, Vector2>,
    pub face_extensions: BTreeMap<FaceId, Extension>,
}

impl GeoMap {
    pub fn new(shalrath::repr::Map(map): shalrath::repr::Map) -> Self {
        let mut entity_head = 0;
        let mut brush_head = 0;
        let mut plane_head = 0;
        let mut texture_head = 0;

        let mut entities = Vec::<EntityId>::default();
        let mut brushes = Vec::<BrushId>::default();
        let mut faces = Vec::<FaceId>::default();

        let mut entity_properties = BTreeMap::<EntityId, Properties>::default();
        let mut entity_brushes = BTreeMap::<EntityId, Vec<BrushId>>::default();

        let mut brush_planes = BTreeMap::<BrushId, Vec<FaceId>>::default();

        let mut face_planes = BTreeMap::<FaceId, Triangle>::default();
        let mut face_textures = BTreeMap::<FaceId, TextureId>::default();
        let mut face_offsets = BTreeMap::<FaceId, TextureOffset>::default();
        let mut face_angles = BTreeMap::<FaceId, f32>::default();
        let mut face_scales = BTreeMap::<FaceId, Vector2>::default();
        let mut face_extensions = BTreeMap::<FaceId, Extension>::default();

        let mut textures = BTreeMap::<String, TextureId>::new();

        for Entity {
            properties,
            brushes: Brushes(bs),
        } in map.into_iter()
        {
            let entity_id = EntityId(entity_head);
            entity_head += 1;

            entities.push(entity_id);
            entity_properties.insert(entity_id, properties);

            for Brush(ps) in bs {
                let brush_id = BrushId(brush_head);
                brush_head += 1;

                brushes.push(brush_id);
                entity_brushes.entry(entity_id).or_default().push(brush_id);

                for BrushPlane {
                    plane,
                    texture,
                    texture_offset,
                    angle,
                    scale_x,
                    scale_y,
                    extension,
                } in ps
                {
                    let plane_id = FaceId(plane_head);
                    plane_head += 1;

                    faces.push(plane_id);
                    face_planes.insert(plane_id, plane);

                    let texture_id = if let Some(texture_id) = textures.get(&texture) {
                        *texture_id
                    } else {
                        let texture_id = TextureId(texture_head);
                        textures.insert(texture, texture_id);
                        texture_head += 1;
                        texture_id
                    };

                    face_textures.insert(plane_id, texture_id);

                    face_offsets.insert(plane_id, texture_offset);
                    face_angles.insert(plane_id, angle);
                    face_scales.insert(plane_id, nalgebra::vector![scale_x, scale_y]);
                    face_extensions.insert(plane_id, extension);
                    brush_planes.entry(brush_id).or_default().push(plane_id);
                }
            }
        }

        let point_entities: Vec<EntityId> = entities
            .iter()
            .filter(|entity_id| !entity_brushes.contains_key(entity_id))
            .copied()
            .collect();

        let textures = textures
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<BTreeMap<_, _>>();

        println!(
            "Point Entities: {:#?}",
            point_entities
                .iter()
                .map(|entity_id| (*entity_id, &entity_properties[entity_id]))
                .collect::<Vec<_>>()
        );

        GeoMap {
            entities,
            brushes,
            faces,
            textures,
            entity_properties,
            entity_brushes,
            point_entities,
            brush_faces: brush_planes,
            face_planes,
            face_textures,
            face_offsets,
            face_angles,
            face_scales,
            face_extensions,
        }
    }
}

impl From<shalrath::repr::Map> for GeoMap {
    fn from(map: shalrath::repr::Map) -> Self {
        GeoMap::new(map)
    }
}