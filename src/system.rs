use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use serde::Deserialize;

use crate::component::NetworkNode;
use crate::component::TypedDecoder;
use crate::prelude::NetworkMessage;
use crate::prelude::StopMarker;

pub trait AppNetworkMessage {
    fn register_decoder<T: NetworkMessage>(&mut self) -> &mut Self;
}

impl AppNetworkMessage for App {
    fn register_decoder<T: NetworkMessage>(&mut self) -> &mut Self {
        debug!("Registering decoder for {}", T::NAME);
        self.add_systems(PostUpdate, decode_system::<T>);
        self
    }
}

fn decode_system<T: for<'a> Deserialize<'a> + Send + Sync + 'static>(query: Query<(Entity, &NetworkNode), With<TypedDecoder<T>>>) {
    for (entity, node) in query.iter() {
        // debug!("Decoding entity {:?}", entity);
    }
}
