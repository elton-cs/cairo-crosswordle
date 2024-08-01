use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};
use starknet_crypto::{poseidon_hash_many, Felt};
use torii_client::client::Client;
use torii_grpc::{client::EntityUpdateStreaming, types::EntityKeysClause};

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_net_session).add_systems(
            FixedUpdate,
            (tell_the_net_task_what_to_do, handle_net_updates),
        );
    }
}

async fn get_torii_client() -> Client {
    let torii_url = "http://0.0.0.0:8080".to_string();
    let rpc_url = "http://0.0.0.0:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world = Felt::from_hex_unchecked(
        "0xb4079627ebab1cd3cf9fd075dda1ad2454a7a448bf659591f259efa2519b18",
    );

    let client: Client = Client::new(torii_url, rpc_url, relay_url, world)
        .await
        .unwrap();

    client
}

async fn get_entities_stream(
    client: &Client,
    player_contract_address: Felt,
) -> EntityUpdateStreaming {
    // create hash of all models' keys (in this case, just one: the player's contract addresss)
    let vec_keys = vec![player_contract_address.clone()];
    let hashed_keys = poseidon_hash_many(&vec_keys);

    // subscribe to updates on models with player's contract address as key
    let entity_keys_clause = Some(EntityKeysClause::HashedKeys(vec![hashed_keys])).unwrap();
    let stream = client
        .on_entity_updated(vec![entity_keys_clause])
        .await
        .unwrap();

    stream
}

/// Messages we send to our netcode task
#[derive(Debug)]
enum MyNetControlMsg {
    DoSomething,
    // ...
}

/// Messages we receive from our netcode task
#[derive(Debug)]
enum MyNetUpdateMsg {
    SomethingHappened,
    // ...
}

#[derive(Component, Debug)]
struct Counter(u64);

/// Channels used for communicating with our game's netcode task.
/// (The side used from our Bevy systems)
#[derive(Resource)]
struct MyNetChannels {
    tx_control: Sender<MyNetControlMsg>,
    rx_updates: Receiver<MyNetUpdateMsg>,
}

fn setup_net_session(mut commands: Commands) {
    // create our channels:
    let (tx_control, rx_control) = async_channel::unbounded();
    let (tx_updates, rx_updates) = async_channel::unbounded();

    // spawn our background i/o task for networking
    // and give it its side of the channels:
    IoTaskPool::get()
        .spawn(async move { my_netcode(rx_control, tx_updates).await })
        .detach();

    // NOTE: `.detach()` to let the task run
    // without us storing the `Task` handle.
    // Otherwise, the task will get canceled!

    // (though in a real application, you probably want to
    // store the `Task` handle and have a system to monitor
    // your task and recreate it if necessary)

    // put our side of the channels in a resource for later
    commands.insert_resource(MyNetChannels {
        tx_control,
        rx_updates,
    });

    let counter = Counter(0);
    commands.spawn(counter);
}

fn handle_net_updates(my_channels: Res<MyNetChannels>, mut query: Query<&mut Counter>) {
    // Non-blocking check for any new messages on the channel
    while let Ok(msg) = my_channels.rx_updates.try_recv() {
        // TODO: do something with `msg`
        // println!("{msg:?} with counter.");
        for mut item in query.iter_mut() {
            println!("{:?}", item.0);
            item.0 += 1;
        }
    }
}

fn tell_the_net_task_what_to_do(my_channels: Res<MyNetChannels>) {
    if let Err(e) = my_channels
        .tx_control
        .try_send(MyNetControlMsg::DoSomething)
    {
        // TODO: handle errors. Maybe our task has
        // returned or panicked, and closed the channel?
    }
}

/// This runs in the background I/O task
async fn my_netcode(rx_control: Receiver<MyNetControlMsg>, tx_updates: Sender<MyNetUpdateMsg>) {
    // TODO: Here we can connect and talk to our multiplayer server,
    // handle incoming `MyNetControlMsg`s, send `MyNetUpdateMsg`s, etc.

    while let Ok(msg) = rx_control.recv().await {
        // TODO: do something with `msg`

        // Send data back, to be handled from Bevy systems:
        tx_updates
            .send(MyNetUpdateMsg::SomethingHappened)
            .await
            .expect("Error sending updates over channel");

        // We can also spawn additional parallel tasks
        // IoTaskPool::get()
        //     .spawn(async move {
        //         // ... some other I/O work ...
        //     })
        //     .detach();
    }
}
