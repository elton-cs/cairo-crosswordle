use async_channel::{unbounded, Receiver};
#[cfg(target_arch = "wasm32")]
use bevy::tasks::IoTaskPool;
use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use starknet_crypto::Felt;
use torii_client::client::Client;
use torii_grpc::{
    client::EntityUpdateStreaming,
    types::{schema::Entity, EntityKeysClause, KeysClause},
};

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_torii_client);
        app.add_systems(Update, update_entities);
    }
}

#[derive(Resource)]
pub struct ToriiClient {
    pub entity_rx: Receiver<Entity>,
    #[cfg(not(target_arch = "wasm32"))]
    pub runtime: tokio::runtime::Runtime,
}

#[derive(Component)]
pub struct BevyEntity {
    pub dojo_entity: Entity,
}

fn setup_torii_client(mut commands: Commands) {
    let torii_url = "http://0.0.0.0:8080".to_string();
    let rpc_url = "http://0.0.0.0:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world = Felt::from_hex_unchecked(
        "0x5d97c46d046f442f125b6cc83057e97ee6e848c4921126acd8ae9d17b55b369",
    );
    let (tx, rx) = unbounded();

    #[cfg(target_arch = "wasm32")]
    let pool = IoTaskPool::get();

    #[cfg(target_arch = "wasm32")]
    pool.spawn(async move {
        info!("Setting up Torii client");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();
        let mut rcv = client
            .on_entity_updated(vec![EntityKeysClause::Keys(KeysClause {
                keys: vec![],
                pattern_matching: torii_grpc::types::PatternMatching::VariableLen,
                models: vec![],
            })])
            .await
            .unwrap();

        info!("Torii client setup");
        while let Some(Ok((_, entity))) = rcv.next().await {
            info!("Received entity: {:?}", entity);
            tx.send(entity).await.unwrap();
        }
    })
    .detach();

    #[cfg(not(target_arch = "wasm32"))]
    let runtime = tokio::runtime::Runtime::new().unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    runtime.spawn(async move {
        info!("Setting up Torii client");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();
        let mut rcv: EntityUpdateStreaming = client
            .on_entity_updated(vec![EntityKeysClause::Keys(KeysClause {
                keys: vec![],
                pattern_matching: torii_grpc::types::PatternMatching::VariableLen,
                models: vec![],
            })])
            .await
            .unwrap();

        info!("Torii client setup");
        while let Some(Ok((_, entity))) = rcv.next().await {
            info!("Received entity: {:?}", entity);
            tx.send(entity).await.unwrap();
        }
    });

    info!("Torii client setup task spawned");

    commands.insert_resource(ToriiClient {
        entity_rx: rx,
        #[cfg(not(target_arch = "wasm32"))]
        runtime: runtime,
    });
}

fn update_entities(
    mut commands: Commands,
    client: Res<ToriiClient>,
    mut query: Query<&mut BevyEntity>,
) {
    match client.entity_rx.try_recv() {
        Ok(entity) => {
            if let Some(mut bevy_entity) = query
                .iter_mut()
                .find(|e| e.dojo_entity.hashed_keys == entity.hashed_keys)
            {
                info!("Updating entity: {:?}", entity);

                bevy_entity.dojo_entity = entity;
            } else {
                info!("Spawning entity: {:?}", entity);

                commands.spawn(BevyEntity {
                    dojo_entity: entity,
                });
            }
        }
        Err(err) => {
            if err != async_channel::TryRecvError::Empty {
                error!("Error receiving entity: {:?}", err);
            }
        }
    }
}
