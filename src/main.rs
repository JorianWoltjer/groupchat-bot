use std::{sync::Arc, path::PathBuf};

use azalea::{Account, Client, Event};
use azalea_protocol::packets::game::{ClientboundGamePacket::PlayerChat, clientbound_player_chat_packet::ChatType};
use parking_lot::Mutex;

const ADDRESS: &str = "localhost:25565";
const ONLINE_MODE: bool = false;

#[tokio::main]
async fn main() {
    // Create account
    let account;
    if ONLINE_MODE {
        let cache_file = PathBuf::from("cache.json");
    
        let auth_result = azalea_auth::auth(
            "example@example.com",
            azalea_auth::AuthOpts {
                cache_file: Some(cache_file),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    
        account = Account::microsoft(&auth_result.access_token).await.unwrap();
    } else {
        account = Account::offline("GroupChat");
    }
    
    // Create initial settings
    let state = State {
        group: vec![  // People who have access to read/write to the group chat
            String::from("Notch"),
            String::from("Herobrine"),
        ],
        admins: vec![  // People who have access to commands to add/remove people from the group
            String::from("Notch"),
        ],
    };

    // Start client
    println!("Joining {ADDRESS} as {} in {} mode", 
        account.username,
        if ONLINE_MODE { "online" } else { "offline" }
    );

    azalea::start(azalea::Options {
        account,
        address: ADDRESS,
        state: Arc::new(Mutex::new(state)),
        plugins: vec![],
        handle,
    })
    .await
    .unwrap();
}

#[derive(Default)]
pub struct State {
    pub group: Vec<String>,
    pub admins: Vec<String>,
}

async fn handle(bot: Client, event: Arc<Event>, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
    match &*event {
        Event::Packet(p) => {
            if let PlayerChat(packet) = &**p {
                match packet.chat_type.chat_type {
                    ChatType::MsgCommandIncoming => {
                        let sender = packet.chat_type.name.to_string();
                        let content = &packet.message.signed_body.content.plain;
                        println!("<{sender}> {content}");

                        // If command by admin
                        if content.starts_with("/") && state.lock().admins.contains(&sender) {
                            let mut args = content.split(" ");
                            let response = match args.next() {
                                Some("/add") => {  // Add a player to the group
                                    if let Some(player) = args.next() {
                                        state.lock().group.push(player.to_string());

                                        bot.chat(&format!("/msg {player} [+] You have been added to the group!")).await?;

                                        format!("[+] Added {player} to group")
                                    } else {
                                        String::from("[!] Usage: /add <name>")
                                    }
                                }
                                Some("/list") => {  // List players in the group
                                    let group = &state.lock().group;
                                    format!("{group:?}")
                                }
                                Some(command @ "/remove") | Some(command @ "/kick") => {  // Remove a player from the group
                                    if let Some(player) = args.next() {
                                        if state.lock().group.contains(&player.to_string()) {
                                            state.lock().group.retain(|p| p != player);
                                            
                                            bot.chat(&format!("/msg {player} [-] You have been removed from the group!")).await?;

                                            format!("[-] Removed {player} from group")
                                        } else {
                                            format!("[!] '{player}' is not in the group")
                                        }
                                    } else {
                                        format!("[!] Usage: {command} <name>")
                                    }
                                }
                                Some(command) => format!("[!] Unknown command '{command}'"),
                                _ => String::from("[!] No command found"),
                            };

                            println!("{response}");
                            let command = format!("/msg {sender} {response}");
                            bot.chat(&command).await?;
                        } else if state.lock().group.contains(&sender) {
                            // Send message to everyone in group
                            let group = state.lock().group.clone();
                            
                            for player in group {
                                if player == sender {  // Skip sender
                                    continue;
                                }

                                let command = format!("/msg {player} <{sender}> {content}");
                                if command.len() <= 200 {
                                    bot.chat(&command).await?;
                                } else {
                                    bot.chat(&format!("/msg {sender} [!] Message too long")).await?;
                                    println!("Error: message too long");
                                }
                            }
                        } else {
                            bot.chat(&format!("/msg {sender} [-] Sorry, you are not in the group")).await?;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    Ok(())
}
