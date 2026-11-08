// Not ported yet: the Java @Inject(RETURN) on Player.getDisplayName (MixinPlayer.getDisplayName).
// Why it's off: Horizon splits Player.getDisplayName across 5 functions; pick one: horizon_client::lan::hub::display_name (crates/horizon-client/src/lan/hub.rs:230); horizon_client::lan::hub::Hub::player_display_name (crates/horizon-client/src/lan/hub.rs:263); horizon_client::game::commands::players::Game<'_>::player_text (crates/horizon-client/src/game/commands/players.rs:130); horizon_client::game::scoreboard::Game<'_>::player_display_comp (crates/horizon-client/src/game/scoreboard.rs:516); horizon_client::game::commands::Game<'_>::player_display_name (crates/horizon-client/src/game/commands.rs:238)
// horizon-mixins kind: RETURN on `horizon_client::lan::hub::display_name`
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: Horizon splits Player.getDisplayName across 5 functions: pick the piece this hooks, then remove the others from `pieces`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Player.getDisplayName -> horizon_client::lan::hub::display_name (suggested, high)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinPlayer.java:13:
// | @Inject(method = "getDisplayName", at = @At("RETURN"), cancellable = true)
// | public void getDisplayName(CallbackInfoReturnable<Component> cir) {
// |    cir.setReturnValue(EntityData.getDisplayName((Player)this, (Component)cir.getReturnValue()));
// | }
