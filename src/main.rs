use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{error, info, warn};
use std::time::Duration;
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::mpsc,
    time::sleep,
};

mod account;
mod block;
mod blockchain;
mod mempool;
mod p2p;
mod stake;
mod transaction;
mod util;
mod validator;
mod wallet;

use blockchain::Blockchain;

use crate::wallet::Wallet;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer Id: {}", p2p::PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    let (init_sender, mut init_rcv) = mpsc::unbounded_channel();
    let (pos_mining_sender, mut pos_mining_rcv) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&p2p::KEYS)
        .expect("can create auth keys");

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let mut wallet = Wallet::new();
    // let wallet = Wallet::get_wallet("5ae5066dd048ffb8f8628c44324e63c7b8782a026009a85a96935acb4921abbc5aede624154386ca358af195e13a46981b917ee8279f30a67d7a211a3d3e7243".to_string());
    // let wallet = Wallet::get_wallet("27a23bf39574e86464f4e638241b3ef3dd223d9a30bd97810ff29c992e747e5a230681c76f00b412ccf7757a8449c448a04acd735e497a7612b66d8bfcb8e576".to_string());
    let behaviour = p2p::AppBehaviour::new(
        Blockchain::new(wallet),
        response_sender,
        init_sender.clone(),
    )
    .await;

    let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID)
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
        .build();

    let mut stdin = BufReader::new(stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can be started");

    spawn(async move {
        sleep(Duration::from_secs(1)).await;
        info!("sending init event");
        init_sender.send(true).expect("can send init event");
    });

    let mut planner = periodic::Planner::new();
    planner.start();

    // Run every second
    planner.add(
        move || pos_mining_sender.send(true).expect("can send init event"),
        periodic::Every::new(Duration::from_secs(1)),
    );

    loop {
        let evt = {
            select! {
                line = stdin.next_line() => Some(p2p::EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                _init = init_rcv.recv() => {
                    Some(p2p::EventType::Init)
                }
                _ = pos_mining_rcv.recv() => {
                    Some(p2p::EventType::Mining)
                },
                event = swarm.select_next_some() => {
                    // info!("Unhandled Swarm Event: {:?}", event);
                    None
                },
            }
        };

        if let Some(event) = evt {
            match event {
                p2p::EventType::Init => {
                    let peers = p2p::get_list_peers(&swarm);

                    info!("connected nodes: {}", peers.len());
                    if !peers.is_empty() {
                        let req = p2p::ChainRequest {
                            from_peer_id: peers
                                .iter()
                                .last()
                                .expect("at least one peer")
                                .to_string(),
                        };

                        let json = serde_json::to_string(&req).expect("can jsonify request");
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(p2p::CHAIN_TOPIC.clone(), json.as_bytes());
                    }
                }
                p2p::EventType::Mining => {
                    if let Some(block) = swarm.behaviour_mut().blockchain.mine_block_by_stake() {
                        swarm
                            .behaviour_mut()
                            .blockchain
                            .add_new_block(block.clone());
                        info!("broadcasting new block");
                        let json = serde_json::to_string(&block).expect("can jsonify request");
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(p2p::BLOCK_TOPIC.clone(), json.as_bytes());
                    };
                }
                p2p::EventType::Input(line) => match line.as_str() {
                    "ls p" => p2p::handle_print_peers(&swarm),
                    "create wallet" => Wallet::generate_wallet(),
                    "ls wallet" => p2p::handle_print_wallet(&mut swarm),
                    "ls c" => p2p::handle_print_chain(&swarm),
                    "ls bal" => p2p::handle_print_balance(&swarm),
                    "ls validator" => p2p::handle_print_validator(&swarm),
                    "ls stakes" => p2p::handle_print_stake(&swarm),
                    "ls mempool" => p2p::handle_print_mempool(&swarm),
                    cmd if cmd.starts_with("set wallet") => p2p::handle_set_wallet(cmd, &mut swarm),
                    cmd if cmd.starts_with("create txn") => p2p::handle_create_txn(cmd, &mut swarm),
                    _ => error!("unknown command"),
                },
            }
        }
    }
}
