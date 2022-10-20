# Minecraft GroupChat Bot

A Minecraft bot that acts as a private group chat. Any player with their name in the group can simply send a message to the bot via `/msg`, 
and the bot then sends that message through to everybody else in the group with the senders name attached. 

Admins of the bot can also add/list/remove people from the group, to keep it dynamic.

## Running

To run the bot, simply clone the repository and use the `--release` flag for the most optimized build:

```Shell
$ git clone https://github.com/JorianWoltjer/groupchat-bot.git && cd groupchat-bot
$ cargo run --release
```

## Settings

To change any settings like who is admin at the start, or the server address to join, you must edit the source code. 

### Server Address

In [`src/main.rs`](src/main.rs) there is a `const ADDRESS` defined, which you can change to any server address and port for the bot to join. 

### Account

The bot can join offline servers (Cracked/Open to LAN) with any username, but for online server that authenticate with Mojang, you will need to log in a Microsoft account. By default the bot does offline mode, but you can change it in [`src/main.rs`](src/main.rs) again by setting `ONLINE_MODE = true`. 

If you set this to true, you will be asked to sign in with a Microsoft account when the program starts. The bot will then join the server using that account. 

### Group/Admins

There should be one or more admins on the server that can add/remove people from the group. Only admins can do this, normal people in the group can only read and write in chat. 

You can edit the list of players in the group by default, and players that are admin. In the [`src/main.rs`](src/main.rs) again, change the `group` and `admin` values in the `state` variable. 
