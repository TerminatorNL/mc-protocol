use crate::protocol::State;
use crate::protocol::Direction;
use steven_protocol::protocol::{LenPrefixedBytes, UUID, LenPrefixed, FixedPoint12};
use steven_protocol::format;
use steven_protocol::item;
use steven_protocol::nbt;
use steven_protocol::types;
use steven_protocol::protocol::packet;
use steven_protocol::protocol::{VarInt, VarLong};
use steven_shared::Position;

crate::define_protocol!(pub Proto_1_17, "1.17", 755 {
    State::Handshaking => {
        Direction::ServerBound => {
            /// Handshake is the first packet sent in the protocol.
            /// Its used for deciding if the request is a client
            /// is requesting status information about the server
            /// (MOTD, players etc) or trying to login to the server.
            ///
            /// The host and port fields are not used by the vanilla
            /// server but are there for virtual server hosting to
            /// be able to redirect a client to a target server with
            /// a single address + port.
            ///
            /// Some modified servers/proxies use the handshake field
            /// differently, packing information into the field other
            /// than the hostname due to the protocol not providing
            /// any system for custom information to be transfered
            /// by the client to the server until after login.
            0x00 => Handshake {
                /// The protocol version of the connecting client
                protocol_version: VarInt,
                /// The hostname the client connected to
                host: String,
                /// The port the client connected to
                port: u16,
                /// The next protocol state the client wants
                next: VarInt,
            }
        }
    },
    State::Status => {
        Direction::ServerBound => {
            /// StatusRequest is sent by the client instantly after
            /// switching to the Status protocol state and is used
            /// to signal the server to send a StatusResponse to the
            /// client
            0x00 => StatusRequest,
            /// StatusPing is sent by the client after recieving a
            /// StatusResponse. The client uses the time from sending
            /// the ping until the time of recieving a pong to measure
            /// the latency between the client and the server.
            0x01 => StatusPing{
                ping: i64,
            },
        },
        Direction::ClientBound => {
            /// StatusResponse is sent as a reply to a StatusRequest.
            /// The Status should contain a json encoded structure with
            /// version information, a player sample, a description/MOTD
            /// and optionally a favicon.
            ///
            /// The structure is as follows
            ///
            /// ```json
            /// {
            ///     "version": {
            ///         "name": "1.8.3",
            ///         "protocol": 47,
            ///     },
            ///     "players": {
            ///         "max": 20,
            ///         "online": 1,
            ///         "sample": [
            ///            packet  {"name": "Thinkofdeath", "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"}
            ///         ]
            ///     },
            ///     "description": "Hello world",
            ///     "favicon": "data:image/png;base64,<data>"
            /// }
            /// ```
            0x00 => StatusResponse{
                status: String,
            },
            /// StatusPong is sent as a reply to a StatusPing.
            /// The Time field should be exactly the same as the
            /// one sent by the client.
            0x01 => StatusPong{
                ping: i64
            },
        }
    },
    State::Login => {
        Direction::ServerBound => {
            /// LoginStart is sent immeditately after switching into the login
            /// state. The passed username is used by the server to authenticate
            /// the player in online mode.
            0x00 => LoginStart{
                username: String
            },
            /// EncryptionResponse is sent as a reply to EncryptionRequest. All
            /// packets following this one must be encrypted with AES/CFB8
            /// encryption.
            0x01 => EncryptionResponse{
                /// The key for the AES/CFB8 cipher encrypted with the
                /// public key
                shared_secret: LenPrefixedBytes<VarInt>,
                /// The verify token from the request encrypted with the
                /// public key
                verify_token: LenPrefixedBytes<VarInt>,
            },
            0x02 => LoginPluginResponse{
                message_id: VarInt,
                successful: bool,
                data: Vec<u8>,
            }
        },
        Direction::ClientBound => {
            /// LoginDisconnect is sent by the server if there was any issues
            /// authenticating the player during login or the general server
            /// issues (e.g. too many players).
            0x00 => LoginDisconnect{
                reason: format::Component,
            },
            /// EncryptionRequest is sent by the server if the server is in
            /// online mode. If it is not sent then its assumed the server is
            /// in offline mode.
            0x01 => EncryptionRequest{
                /// Generally empty, left in from legacy auth
                /// but is still used by the client if provided
                server_id: String,
                /// A RSA Public key serialized in x.509 PRIX format
                public_key: LenPrefixedBytes<VarInt>,
                /// Token used by the server to verify encryption is working
                /// correctly
                verify_token: LenPrefixedBytes<VarInt>,
            },
            0x02 => LoginSuccess{
                uuid: UUID,
                username: String,
            },
            /// SetInitialCompression sets the compression threshold during the
            /// login state.
            0x03 => SetInitialCompression{
                /// Threshold where a packet should be sent compressed
                threshold: VarInt,
            },
            0x04 => LoginPluginRequest{
                message_id: VarInt,
                channel: String,
                data: Vec<u8>,
            },
        }
    },
    State::Play => {
        Direction::ServerBound => {
            /// TeleportConfirm is sent by the client as a reply to a telport from
            /// the server.
            0x00 => TeleportConfirm{
                teleport_id: VarInt,
            },
            0x01 => QueryBlockNBT{
                transaction_id: VarInt,
                location: Position,
            },
            0x02 => SetDifficulty{
                new_difficulty: u8,
            },
            /// ChatMessage is sent by the client when it sends a chat message or
            /// executes a command (prefixed by '/').
            0x03 => ChatMessage {
                message: String,
            },
            /// ClientStatus is sent to update the client's status
            0x04 => ClientStatus{
                action_id: VarInt,
            },
            /// ClientSettings is sent by the client to update its current settings.
            0x05 => ClientSettings {
                locale: String,
                view_distance: u8,
                chat_mode: VarInt,
                chat_colors: bool,
                displayed_skin_parts: u8,
                main_hand: VarInt,
            },
            /// TabComplete is sent by the client when the client presses tab in
            /// the chat box.
            0x06 => TabComplete {
                text: String,
                assume_command: bool,
                has_target: bool,
                target: Option<Position> where |p| {p.has_target},
            },
            /// ClickWindowButton is used for clicking an enchantment, lectern, stonecutter, or loom.
            0x07 => ClickWindowButton {
                id: u8,
                button: u8,
            },
            /// ClickWindow is sent when the client clicks in a window.
            0x08 => ClickWindow {
                id: u8,
                slot: i16,
                button: u8,
                action_number: u16,
                mode: VarInt,
                clicked_item: Option<item::Stack>,
            },
            /// CloseWindow is sent when the client closes a window.
            0x09 => CloseWindow {
                id: u8,
            },
            /// PluginMessageServerbound is used for custom messages between the client
            /// and server. This is mainly for plugins/mods but vanilla has a few channels
            /// registered too.
            0x0a => PluginMessageServerbound {
                channel: String,
                data: Vec<u8>,
            },
            0x0b => EditBook {
                new_book: Option<item::Stack>,
                is_signing: bool,
                hand: VarInt,
            },
            0x0c => QueryEntityNBT {
                transaction_id: VarInt,
                entity_id: VarInt,
            },
            /// UseEntity is sent when the user interacts (right clicks) or attacks
            /// (left clicks) an entity.
            0x0d => UseEntity {
                target_id: VarInt,
                ty: VarInt,
                target_x: f32 where |p| {p.ty.0 == 2},
                target_y: f32 where |p| {p.ty.0 == 2},
                target_z: f32 where |p| {p.ty.0 == 2},
                hand: VarInt where |p| {p.ty.0 == 0 || p.ty.0 == 2},
                sneaking: bool,
            },
            /// Sent when Generate is pressed on the Jigsaw Block interface.
            0x0e => GenerateStructure {
                location: Position,
                levels: VarInt,
                keep_jigsaws: bool,
            },
            /// KeepAliveServerbound is sent by a client as a response to a
            /// KeepAliveClientbound. If the client doesn't reply the server
            /// may disconnect the client.
            0x0f => KeepAliveServerbound {
                id: i64,
            },
            0x10 => LockDifficulty {
                locked: bool,
            },
            /// PlayerPosition is used to update the player's position.
            0x11 => PlayerPosition {
                x: f64,
                y: f64,
                z: f64,
                on_ground: bool,
            },
            /// PlayerPositionLook is a combination of PlayerPosition and
            /// PlayerLook.
            0x12 => PlayerPositionLook {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            },
            /// PlayerLook is used to update the player's rotation.
            0x13 => PlayerLook {
                yaw: f32,
                pitch: f32,
                on_ground: bool,
            },
            /// Player is used to update whether the player is on the ground or not.
            0x14 => Player {
                on_ground: bool,
            },
            /// Sent by the client when in a vehicle instead of the normal move packet.
            0x15 => VehicleMove {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
            },
            /// SteerBoat is used to visually update the boat paddles.
            0x16 => SteerBoat {
                left_paddle_turning: bool,
                right_paddle_turning: bool,
            },
            0x17 => PickItem {
                slot_to_use: VarInt,
            },
            /// CraftRecipeRequest is sent when player clicks a recipe in the crafting book.
            0x18 => CraftRecipeRequest {
                window_id: u8,
                recipe: VarInt,
                make_all: bool,
            },
            0x19 => ClientAbilities {
                flags: u8,
            },
            /// PlayerDigging is sent when the client starts/stops digging a block.
            /// It also can be sent for droppping items and eating/shooting.
            0x1a => PlayerDigging {
                status: VarInt,
                location: Position,
                face: u8,
            },
            /// PlayerAction is sent when a player preforms various actions.
            0x1b => PlayerAction{
                entity_id: VarInt,
                action_id: VarInt,
                jump_boost: VarInt,
            },
            /// SteerVehicle is sent by the client when steers or preforms an action
            /// on a vehicle.
            0x1c => SteerVehicle {
                sideways: f32,
                forward: f32,
                flags: u8,
            },
            0x1d => PlayPong {
                id: i32,
            },
            /// SetDisplayedRecipe replaces CraftingBookData, type 0.
            0x1e => SetDisplayedRecipe {
                recipe_id: String,
            },
            /// SetRecipeBookState replaces CraftingBookData, type 1.
            0x1f => SetRecipeBookState {
                book_id: VarInt, // TO DO: enum, 0: crafting, 1: furnace, 2: blast furnace, 3: smoker
                book_open: bool,
                filter_active: bool,
            },
            0x20 => NameItem {
                item_name: String,
            },
            /// ResourcePackStatus informs the server of the client's current progress
            /// in activating the requested resource pack
            0x21 => ResourcePackStatus {
                result: VarInt,
            },
            0x22 => AdvancementTab {
                action: VarInt,
                tab_id: String where |p| {p.action.0 == 0},
            },
            0x23 => SelectTrade {
                selected_slot: VarInt,
            },
            0x24 => SetBeaconEffect {
                primary_effect: VarInt,
                secondary_effect: VarInt,
            },
            /// HeldItemChange is sent when the player changes the currently active
            /// hotbar slot.
            0x25 => HeldItemChange {
                slot: i16,
            },
            0x26 => UpdateCommandBlock {
                location: Position,
                command: String,
                mode: VarInt,
                flags: u8,
            },
            0x27 => UpdateCommandBlockMinecart {
                entity_id: VarInt,
                command: String,
                track_output: bool,
            },
            /// CreativeInventoryAction is sent when the client clicks in the creative
            /// inventory. This is used to spawn items in creative.
            0x28 => CreativeInventoryAction {
                slot: i16,
                clicked_item: Option<item::Stack>,
            },
            0x29 => UpdateJigsawBlock {
                location: Position,
                name: String,
                target: String,
                pool: String,
                final_state: String,
                joint_type: String,
            },
            0x2a => UpdateStructureBlock {
                location: Position,
                action: VarInt,
                mode: VarInt,
                name: String,
                offset_x: i8,
                offset_y: i8,
                offset_z: i8,
                size_x: i8,
                size_y: i8,
                size_z: i8,
                mirror: VarInt,
                rotation: VarInt,
                metadata: String,
                integrity: f32,
                seed: VarLong,
                flags: i8,
            },
            /// SetSign sets the text on a sign after placing it.
            0x2b => SetSign {
                location: Position,
                line1: String,
                line2: String,
                line3: String,
                line4: String,
            },
            /// ArmSwing is sent by the client when the player left clicks
            /// (to swing their arm).
            0x2c => ArmSwing {
                hand: VarInt,
            },
            /// SpectateTeleport is sent by clients in spectator mode to teleport to a player.
            0x2d => SpectateTeleport {
                target: UUID,
            },
            0x2e => PlayerBlockPlacement {
                hand: VarInt,
                location: Position,
                face: VarInt,
                cursor_x: f32,
                cursor_y: f32,
                cursor_z: f32,
                inside_block: bool,
            },
            /// UseItem is sent when the client tries to use an item.
            0x2f => UseItem {
                hand: VarInt,
            },
        },
        Direction::ClientBound => {
            0x00 => SpawnObject {
                entity_id: VarInt,
                uuid: UUID,
                ty: VarInt,
                x: f64,
                y: f64,
                z: f64,
                pitch: i8,
                yaw: i8,
                data: i32,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            },
            /// SpawnExperienceOrb spawns a single experience orb into the world when
            /// it is in range of the client. The count controls the amount of experience
            /// gained when collected.
            0x01 => SpawnExperienceOrb {
                entity_id: VarInt,
                x: f64,
                y: f64,
                z: f64,
                count: i16,
            },
            /// SpawnMob is used to spawn a living entity into the world when it is in
            /// range of the client.
            0x02 => SpawnMob {
                entity_id: VarInt,
                uuid: UUID,
                ty: VarInt,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
                head_pitch: i8,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            },
            /// SpawnPainting spawns a painting into the world when it is in range of
            /// the client. The title effects the size and the texture of the painting.
            0x03 => SpawnPainting {
                entity_id: VarInt,
                uuid: UUID,
                motive: VarInt,
                location: Position,
                direction: u8,
            },
            /// SpawnPlayer is used to spawn a player when they are in range of the client.
            /// This alone isn't enough to display the player as the skin and username
            /// information is in the player information packet.
            0x04 => SpawnPlayer {
                entity_id: VarInt,
                uuid: UUID,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
            },
            0x05 => SculkVibrationSignal {
                source: Position,
                destination_id: String,
                destination_pos: Option<Position> where |p| {
                    unimplemented!("Not enough info to tell if Position or VarInt with entity ID")
                },
                arrival_ticks: VarInt,
            },
            /// Animation is sent by the server to play an animation on a specific entity.
            0x06 => Animation {
                entity_id: VarInt,
                animation_id: u8,
            },
            /// Statistics is used to update the statistics screen for the client.
            0x07 => Statistics {
                statistices: LenPrefixed<VarInt, packet::Statistic>,
            },
            0x08 => AcknowledgePlayerDigging {
                location: Position,
                block: VarInt,
                status: VarInt,
                successful: bool,
            },
            /// BlockBreakAnimation is used to create and update the block breaking
            /// animation played when a player starts digging a block.
            0x09 => BlockBreakAnimation {
                entity_id: VarInt,
                location: Position,
                stage: i8,
            },
            /// UpdateBlockEntity updates the nbt tag of a block entity in the
            /// world.
            0x0a => UpdateBlockEntity {
                location: Position,
                action: u8,
                nbt: Option<nbt::NamedTag>,
            },
            /// BlockAction triggers different actions depending on the target block.
            0x0b => BlockAction {
                location: Position,
                byte1: u8,
                byte2: u8,
                block_type: VarInt,
            },
            /// BlockChange is used to update a single block on the client.
            0x0c => BlockChange {
                location: Position,
                block_id: VarInt,
            },
            /// BossBar displays and/or changes a boss bar that is displayed on the
            /// top of the client's screen. This is normally used for bosses such as
            /// the ender dragon or the wither.
            0x0d => BossBar {
                uuid: UUID,
                action: VarInt,
                title: format::Component where |p| {
                    p.action.0 == 0 || p.action.0 == 3
                },
                health: f32 where |p| {
                    p.action.0 == 0 || p.action.0 == 2
                },
                color: VarInt where |p| {
                    p.action.0 == 0 || p.action.0 == 4
                },
                style: VarInt where |p| {
                    p.action.0 == 0 || p.action.0 == 4
                },
                flags: u8 where |p| {
                    p.action.0 == 0 || p.action.0 == 5
                }
            },
            /// ServerDifficulty changes the displayed difficulty in the client's menu
            /// as well as some ui changes for hardcore.
            0x0e => ServerDifficulty {
                difficulty: u8,
                locked: bool,
            },
            /// ServerMessage is a message sent by the server. It could be from a player
            /// or just a system message. The Type controls the location the
            /// message is displayed at and when the message is displayed.
            0x0f => ServerMessage {
                message: serde_json::Value,
                /// 0 - Chat message, 1 - System message, 2 - Action bar message
                position: u8,
                sender: UUID,
            },
            /// Clear the client's current title information
            0x10 => ClearTitles {
                reset: bool,
            },
            /// TabCompleteReply is sent as a reply to a tab completion request.
            /// The matches should be possible completions for the command/chat the
            /// player sent.
            0x11 => TabCompleteReply {
                matches: LenPrefixed<VarInt, String>,
            },
            0x12 => DeclareCommands {
                nodes: LenPrefixed<VarInt, packet::CommandNode>,
                root_index: VarInt,
            },
            /// WindowClose forces the client to close the window with the given id,
            /// e.g. a chest getting destroyed.
            0x13 => WindowClose {
                id: u8,
            },
            /// WindowItems sets every item in a window.
            0x14 => WindowItems {
                id: u8,
                items: LenPrefixed<i16, Option<item::Stack>>,
            },
            /// WindowProperty changes the value of a property of a window. Properties
            /// vary depending on the window type.
            0x15 => WindowProperty {
                id: u8,
                property: i16,
                value: i16,
            },
            /// WindowSetSlot changes an itemstack in one of the slots in a window.
            0x16 => WindowSetSlot {
                id: u8,
                property: i16,
                item: Option<item::Stack>,
            },
            /// SetCooldown disables a set item (by id) for the set number of ticks
            0x17 => SetCooldown {
                item_id: VarInt,
                ticks: VarInt,
            },
            /// PluginMessageClientbound is used for custom messages between the client
            /// and server. This is mainly for plugins/mods but vanilla has a few channels
            /// registered too.
            0x18 => PluginMessageClientbound {
                channel: String,
                data: Vec<u8>,
            },
            /// Plays a sound by name on the client
            0x19 => NamedSoundEffect {
                name: String,
                category: VarInt,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: f32,
            },
            /// Disconnect causes the client to disconnect displaying the passed reason.
            0x1a => Disconnect {
                reason: format::Component,
            },
            /// EntityAction causes an entity to preform an action based on the passed
            /// id.
            0x1b => EntityAction {
                entity_id: i32,
                action_id: u8,
            },
            /// Explosion is sent when an explosion is triggered (tnt, creeper etc).
            /// This plays the effect and removes the effected blocks.
            0x1c => Explosion {
                x: f32,
                y: f32,
                z: f32,
                radius: f32,
                records: LenPrefixed<i32, packet::ExplosionRecord>,
                velocity_x: f32,
                velocity_y: f32,
                velocity_z: f32,
            },
            /// ChunkUnload tells the client to unload the chunk at the specified
            /// position.
            0x1d => ChunkUnload {
                x: i32,
                z: i32,
            },
            /// ChangeGameState is used to modify the game's state like gamemode or
            /// weather.
            0x1e => ChangeGameState {
                reason: u8,
                value: f32,
            },
            0x1f => WindowOpenHorse {
                window_id: u8,
                number_of_slots: VarInt,
                entity_id: i32,
            },
            0x20 => InitializeWorldBorder{
                x: f64,
                z: f64,
                old_diameter: f64,
                new_diameter: f64,
                speed: VarLong,
                portal_tp_boundary: VarInt,
                warning_blocks: VarInt,
                warning_time: VarInt,
            },
            /// KeepAliveClientbound is sent by a server to check if the
            /// client is still responding and keep the connection open.
            /// The client should reply with the KeepAliveServerbound
            /// setting ID to the same as this one.
            0x21 => KeepAliveClientbound {
                id: i64,
            },
            0x22 => ChunkData {
                chunk_x: i32,
                chunk_z: i32,
                bitmask: LenPrefixed<VarInt, VarLong>,
                heightmaps: Option<nbt::NamedTag>,
                biomes: LenPrefixed<VarInt, VarInt>,
                data: LenPrefixedBytes<VarInt>,
                block_entities: LenPrefixed<VarInt, Option<nbt::NamedTag>>,
            },
            /// Effect plays a sound effect or particle at the target location with the
            /// volume (of sounds) being relative to the player's position unless
            /// DisableRelative is set to true.
            0x23 => Effect {
                effect_id: i32,
                location: Position,
                data: i32,
                disable_relative: bool,
            },
            /// Particle spawns particles at the target location with the various
            /// modifiers.
            0x24 => Particle {
                particle_id: i32,
                long_distance: bool,
                x: f64,
                y: f64,
                z: f64,
                offset_x: f32,
                offset_y: f32,
                offset_z: f32,
                speed: f32,
                count: i32,
                block_state: VarInt where |p| {
                    p.particle_id == 3 || p.particle_id == 23
                },
                red: f32 where |p| {
                    p.particle_id == 14
                },
                green: f32 where |p| {
                    p.particle_id == 14
                },
                blue: f32 where |p| {
                    p.particle_id == 14
                },
                scale: f32 where |p| {
                    p.particle_id == 14
                },
                item: Option<nbt::NamedTag> where |p| {
                    p.particle_id == 32
                },
            },
            0x25 => UpdateLight {
                chunk_x: VarInt,
                chunk_z: VarInt,
                trust_edges: bool,
                sky_light_mask: LenPrefixed<VarInt, i64>,
                block_light_mask: LenPrefixed<VarInt, i64>,
                empty_sky_light_mask: LenPrefixed<VarInt, i64>,
                empty_block_light_mask: LenPrefixed<VarInt, i64>,
                sky_light: LenPrefixed<VarInt, LenPrefixed<VarInt,u8>>,
                light_array: LenPrefixed<VarInt, LenPrefixed<VarInt,u8>>,
            },
            /// JoinGame is sent after completing the login process. This
            /// sets the initial state for the client.
            0x26 => JoinGame {
                /// The entity id the client will be referenced by
                entity_id: i32,
                /// Whether hardcore mode is enabled
                is_hardcore: bool,
                /// The starting gamemode of the client
                gamemode: u8,
                /// The previous gamemode of the client
                previous_gamemode: u8,
                /// Identifiers for all worlds on the server
                world_names: LenPrefixed<VarInt, String>,
                /// Represents a dimension registry
                dimension_codec: Option<nbt::NamedTag>,
                /// The dimension the client is starting in
                dimension: Option<nbt::NamedTag>,
                /// The world being spawned into
                world_name: String,
                /// Truncated SHA-256 hash of world's seed
                hashed_seed: i64,
                /// The max number of players on the server
                max_players: VarInt,
                /// The render distance (2-32)
                view_distance: VarInt,
                /// Whether the client should reduce the amount of debug
                /// information it displays in F3 mode
                reduced_debug_info: bool,
                /// Whether to prompt or immediately respawn
                enable_respawn_screen: bool,
                /// Whether the world is in debug mode
                is_debug: bool,
                /// Whether the world is a superflat world
                is_flat: bool,
            },
            /// Maps updates a single map's contents
            0x27 => Maps {
                item_damage: VarInt,
                scale: i8,
                tracking_position: bool,
                locked: bool,
                icons: LenPrefixed<VarInt, packet::MapIcon>,
                columns: u8,
                rows: Option<u8> where |p| {
                    p.columns > 0
                },
                x: Option<u8> where |p| {
                    p.columns > 0
                },
                z: Option<u8> where |p| {
                    p.columns > 0
                },
                data: Option<LenPrefixedBytes<VarInt>> where |p| {
                    p.columns > 0
                },
            },
            0x28 => TradeList {
                id: VarInt,
                trades: LenPrefixed<u8, packet::Trade>,
                villager_level: VarInt,
                experience: VarInt,
                is_regular_villager: bool,
                can_restock: bool,
            },
            /// EntityMove moves the entity with the id by the offsets provided.
            0x29 => EntityMove {
                entity_id: VarInt,
                delta_x: FixedPoint12<i16>,
                delta_y: FixedPoint12<i16>,
                delta_z: FixedPoint12<i16>,
                on_ground: bool,
            },
            /// EntityLookAndMove is a combination of EntityMove and EntityLook.
            0x2a => EntityLookAndMove {
                entity_id: VarInt,
                delta_x: FixedPoint12<i16>,
                delta_y: FixedPoint12<i16>,
                delta_z: FixedPoint12<i16>,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            },
            /// EntityLook rotates the entity to the new angles provided.
            0x2b => EntityLook {
                entity_id: VarInt,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            },
            /// Teleports the player's vehicle
            0x2c => VehicleTeleport {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
            },
            /// Opens the book GUI.
            0x2d => OpenBook {
                hand: VarInt,
            },
            /// WindowOpen tells the client to open the inventory window of the given
            /// type. The ID is used to reference the instance of the window in
            /// other packets.
            0x2e => WindowOpen {
                id: u8,
                ty: String,
                title: format::Component,
                slot_count: u8,
                entity_id: i32 where |p|{
                    p.ty == "EntityHorse"
                },
            },
            /// SignEditorOpen causes the client to open the editor for a sign so that
            /// it can write to it. Only sent in vanilla when the player places a sign.
            0x2f => SignEditorOpen {
                location: Position,
            },
            0x30 => PlayPing {
                id: i32,
            },
            /// CraftRecipeResponse is a response to CraftRecipeRequest, notifies the UI.
            0x31 => CraftRecipeResponse {
                window_id: u8,
                recipe: VarInt,
            },
            /// PlayerAbilities is used to modify the players current abilities. Flying,
            /// creative, god mode etc.
            0x32 => PlayerAbilities {
                flags: u8,
                flying_speed: f32,
                walking_speed: f32,
            },
            0x33 => CombatEvent {
                event: VarInt,
                direction: Option<VarInt> where |p| {
                    p.event.0 == 1
                },
                player_id: Option<VarInt> where |p| {
                    p.event.0 == 2
                },
                entity_id: Option<i32> where |p| {
                    p.event.0 == 1 || p.event.0 == 2
                },
                message: Option<format::Component> where |p| {
                    p.event.0 == 2
                },
            },
            0x34 => EnterCombatEvent,
            0x35 => DeathCombatEvent {
                player_id: VarInt,
                killer_id: i32,
                message: format::Component,
            },
            /// PlayerInfo is sent by the server for every player connected to the server
            /// to provide skin and username information as well as ping and gamemode info.
            0x36 => PlayerInfo {
                inner: packet::PlayerInfoData,
            },
            0x37 => FacePlayer {
                feet_eyes: VarInt,
                target_x: f64,
                target_y: f64,
                target_z: f64,
                is_entity: bool,
                entity_id: Option<VarInt> where |p| {p.is_entity},
                entity_feet_eyes: Option<VarInt> where |p| {p.is_entity},
            },
            0x38 => TeleportPlayer {
                x: f64,
                y: f64,
                z: f64,
                yaw: f32,
                pitch: f32,
                flags: u8,
                teleport_id: VarInt,
                dismount: bool,
            },
            0x39 => UnlockRecipes{
                action: VarInt,
                crafting_book_open: bool,
                filtering_craftable: bool,
                smelting_book_open: bool,
                filtering_smeltable: bool,
                blast_furnace_open: bool,
                filtering_blast_furnace: bool,
                smoker_open: bool,
                filtering_smoker: bool,
                recipe_ids: LenPrefixed<VarInt, String>,
                recipe_ids2: LenPrefixed<VarInt, String> where |p| {
                    p.action.0 == 0
                }
            },
            0x3a => EntityDestroy{
                entity_id: VarInt,
            },
            /// EntityRemoveEffect removes an effect from an entity.
            0x3b => EntityRemoveEffect {
                entity_id: VarInt,
                effect_id: i8,
            },
            /// ResourcePackSend causes the client to check its cache for the requested
            /// resource and download it if its missing. Once the resource pack
            /// is obtained the client will use it.
            0x3c => ResourcePackSend {
                url: String,
                hash: String,
            },
            /// Respawn is sent to respawn the player after death or when they move worlds.
            0x3d => Respawn {
                dimension: Option<nbt::NamedTag>,
                world_name: String,
                hashed_seed: i64,
                gamemode: u8,
                previous_gamemode: u8,
                is_debug: bool,
                is_flat: bool,
                copy_metadata: bool,
            },
            /// EntityHeadLook rotates an entity's head to the new angle.
            0x3e => EntityHeadLook {
                entity_id: VarInt,
                head_yaw: i8,
            },
            /// MultiBlockChange is used to update a batch of blocks in a single packet.
            0x3f => MultiBlockChange {
                chunk_section_pos: u64,
                no_trust_edges: bool,
                records: LenPrefixed<VarInt, VarLong>,
            },
            /// SelectAdvancementTab indicates the client should switch the advancement tab.
            0x40 => SelectAdvancementTab {
                has_id: bool,
                tab_id: String where |p| {p.has_id},
            },
            0x41 => ActionBar {
                message: format::Component,
            },
            0x42 => WorldBorderCenter {
                x: f64,
                z: f64,
            },
            0x43 => WorldBorderResize {
                old_diameter: f64,
                new_diameter: f64,
                speed: VarLong,
            },
            0x44 => WorldBorderSize {
                diameter: f64,
            },
            0x45 => WorldBorderWarningTime {
                warning_time: VarInt,
            },
            0x46 => WorldBorderWarningDistance {
                warning_blocks: VarInt,
            },
            /// Camera causes the client to spectate the entity with the passed id.
            /// Use the player's id to de-spectate.
            0x47 => Camera {
                target_id: VarInt,
            },
            /// SetCurrentHotbarSlot changes the player's currently selected hotbar item.
            0x48 => SetCurrentHotbarSlot {
                slot: u8,
            },
            /// UpdateViewPosition is used to determine what chunks should be remain loaded.
            0x49 => UpdateViewPosition {
                chunk_x: VarInt,
                chunk_z: VarInt,
            },
            /// UpdateViewDistance is sent by the integrated server when changing render distance.
            0x4a => UpdateViewDistance {
                view_distance: VarInt,
            },
            /// SpawnPosition is sent to change the player's current spawn point. Currently
            /// only used by the client for the compass.
            0x4b => SpawnPosition {
                location: Position,
            },
            /// ScoreboardDisplay is used to set the display position of a scoreboard.
            0x4c => ScoreboardDisplay {
                position: u8,
                name: String,
            },
            /// EntityMetadata updates the metadata for an entity.
            0x4d => EntityMetadata {
                entity_id: VarInt,
                metadata: types::Metadata,
            },
            /// EntityAttach attaches to entities together, either by mounting or leashing.
            /// -1 can be used at the EntityID to deattach.
            0x4e => EntityAttach {
                entity_id: i32,
                vehicle: i32,
            },
            /// EntityVelocity sets the velocity of an entity in 1/8000 of a block
            /// per a tick.
            0x4f => EntityVelocity {
                entity_id: VarInt,
                velocity_x: i16,
                velocity_y: i16,
                velocity_z: i16,
            },
            /// EntityEquipment is sent to display an item on an entity, like a sword
            /// or armor. Slot 0 is the held item and slots 1 to 4 are boots, leggings
            /// chestplate and helmet respectively.
            0x50 => EntityEquipment {
                entity_id: VarInt,
                equipments: packet::EntityEquipments,
            },
            /// SetExperience updates the experience bar on the client.
            0x51 => SetExperience {
                experience_bar: f32,
                level: VarInt,
                total_experience: VarInt,
            },
            /// UpdateHealth is sent by the server to update the player's health and food.
            0x52 => UpdateHealth {
                health: f32,
                food: VarInt,
                food_saturation: f32,
            },
            /// ScoreboardObjective creates/updates a scoreboard objective.
            0x53 => ScoreboardObjective {
                name: String,
                mode: u8,
                value: String where |p| {
                    p.mode == 0 || p.mode == 2
                },
                ty: String where |p| {
                    p.mode == 0 || p.mode == 2
                }
            },
            /// SetPassengers mounts entities to an entity
            0x54 => SetPassengers {
                entity_id: VarInt,
                passengers: LenPrefixed<VarInt, VarInt>,
            },
            /// Teams creates and updates teams
            0x55 => Teams {
                name: String,
                mode: u8,
                display_name: Option<String> where |p| {p.mode == 0 || p.mode == 2},
                flags: Option<u8> where |p| {p.mode == 0 || p.mode == 2},
                name_tag_visibility: Option<String> where |p| {
                    p.mode == 0 || p.mode == 2
                },
                collision_rule: Option<String> where |p| {
                    p.mode == 0 || p.mode == 2
                },
                formatting: Option<VarInt> where |p| {
                    p.mode == 0 || p.mode == 2
                },
                prefix: Option<String> where |p| {
                    p.mode == 0 || p.mode == 2
                },
                suffix: Option<String> where |p| {
                    p.mode == 0 || p.mode == 2
                },
                players: Option<LenPrefixed<VarInt, String>> where |p| {
                    p.mode == 0 || p.mode == 3 || p.mode == 4
                },
            },
            /// UpdateScore is used to update or remove an item from a scoreboard
            /// objective.
            0x56 => UpdateScore {
                name: String,
                action: u8,
                object_name: String,
                value: Option<VarInt> where |p| {
                    p.action != 1
                },
            },
            0x57 => SetTitleSubtitle {
                subtitle: format::Component,
            },
            /// TimeUpdate is sent to sync the world's time to the client, the client
            /// will manually tick the time itself so this doesn't need to sent repeatedly
            /// but if the server or client has issues keeping up this can fall out of sync
            /// so it is a good idea to send this now and again
            0x58 => TimeUpdate {
                world_age: i64,
                time_of_day: i64,
            },
            /// Title configures an on-screen title.
            0x59 => Title {
                action: VarInt,
                title: Option<format::Component> where |p| {
                    p.action.0 == 0
                },
                sub_title: Option<format::Component> where |p| {
                    p.action.0 == 1
                },
                action_bar_text: Option<String> where |p| {
                    p.action.0 == 2
                },
                fade_in: Option<i32> where |p| {
                    p.action.0 == 3
                },
                fade_stay: Option<i32> where |p| {
                    p.action.0 == 3
                },
                fade_out: Option<i32> where |p| {
                    p.action.0 == 3
                }
            },
            0x5a => SetTitleTimes {
                fade_in: i32,
                stay: i32,
                fade_out: i32,
            },
            /// Plays a sound effect from an entity.
            0x5b => EntitySoundEffect {
                sound_id: VarInt,
                sound_category: VarInt,
                entity_id: VarInt,
                volume: f32,
                pitch: f32,
            },
            /// SoundEffect plays the named sound at the target location.
            0x5c => SoundEffect {
                name: VarInt,
                category: VarInt,
                x: i32,
                y: i32,
                z: i32,
                volume: f32,
                pitch: f32,
            },
            0x5d => StopSound {
                flags: u8,
                source: Option<VarInt> where |p| {
                    p.flags & 0x01 != 0
                },
                sound: Option<String> where |p| {
                    p.flags & 0x02 != 0
                }
            },
            /// PlayerListHeaderFooter updates the header/footer of the player list.
            0x5e => PlayerListHeaderFooter {
                header: format::Component,
                footer: format::Component,
            },
            0x5f => NBTQueryResponse {
                transaction_id: VarInt,
                nbt: Option<nbt::NamedTag>,
            },
            /// CollectItem causes the collected item to fly towards the collector. This
            /// does not destroy the entity.
            0x60 => CollectItem {
                collected_entity_id: VarInt,
                collector_entity_id: VarInt,
                number_of_items: VarInt,
            },
            /// EntityTeleport teleports the entity to the target location. This is
            /// sent if the entity moves further than EntityMove allows.
            0x61 => EntityTeleport {
                entity_id: VarInt,
                x: f64,
                y: f64,
                z: f64,
                yaw: i8,
                pitch: i8,
                on_ground: bool,
            },
            0x62 => Advancements {
                data: Vec<u8>,
                /* TODO: fix parsing modded advancements 1.12.2 (e.g. SevTech Ages)
                 * see https://github.com/iceiix/stevenarella/issues/148
                reset_clear: bool,
                mapping: LenPrefixed<VarInt, packet::Advancement>,
                identifiers: LenPrefixed<VarInt, String>,
                progress: LenPrefixed<VarInt, packet::AdvancementProgress>,
                */
            },
            /// EntityProperties updates the properties for an entity.
            0x63 => EntityProperties{
                entity_id: VarInt,
                properties: LenPrefixed<VarInt, packet::EntityProperty>,
            },
            /// EntityEffect applies a status effect to an entity for a given duration.
            0x64 => EntityEffect {
                entity_id: VarInt,
                effect_id: i8,
                amplifier: i8,
                duration: VarInt,
                hide_particles: bool,
            },
            0x65 => DeclareRecipes {
                recipes: LenPrefixed<VarInt, packet::Recipe>,
            },
            0x66 => Tags {
                block_tags: LenPrefixed<VarInt, packet::Tags>,
                item_tags: LenPrefixed<VarInt, packet::Tags>,
                fluid_tags: LenPrefixed<VarInt, packet::Tags>,
                entity_tags: LenPrefixed<VarInt, packet::Tags>,
            },
        }
    }
});