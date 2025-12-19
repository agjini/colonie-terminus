use bevy::asset::AssetPath;
use bevy::prelude::*;
use ron_asset_manager::RonAssetPlugin;
use ron_asset_manager::prelude::RonAsset;
use std::collections::VecDeque;
use std::fmt::Debug;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>()
        .add_systems(PreUpdate, load_resource_assets);
}

pub trait LoadResource {
    /// This will load the [`Resource`] as an [`Asset`]. When all of its asset dependencies
    /// have been loaded, it will be inserted as a resource. This ensures that the resource only
    /// exists when the assets are ready.
    fn load_resource<'a, T: Resource + Asset + RonAsset + Debug + Clone>(
        &mut self,
        ron_file: impl Into<AssetPath<'a>>,
    ) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<'a, T: Resource + Asset + RonAsset + Debug + Clone>(
        &mut self,
        ron_file: impl Into<AssetPath<'a>>,
    ) -> &mut Self {
        self.add_plugins(RonAssetPlugin::<T>::default());
        let world = self.world_mut();
        let assets = world.resource::<AssetServer>();

        let value = assets.load::<T>(ron_file);

        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((value.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}

/// A function that inserts a loaded resource.
type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
pub struct ResourceHandles {
    // Use a queue for waiting assets so they can be cycled through and moved to
    // `finished` one at a time.
    waiting: VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    /// Returns true if all requested [`Asset`]s have finished loading and are available as [`Resource`]s.
    pub fn is_all_done(&self) -> bool {
        self.waiting.is_empty()
    }
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                if assets.is_loaded(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                }
            }
        });
    });
}
