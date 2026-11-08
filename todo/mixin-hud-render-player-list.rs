// Not ported yet: the Java @Inject(INVOKE) on Hud.extractTabList (MixinHud.renderPlayerList).
// Why it's off: No Horizon function mapped for Hud.extractTabList (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for Hud.extractTabList: run `match`.
// Note: The call it hooks (PlayerTabOverlay.extractRenderState) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.text -> (not mapped yet)
//   Hud.displayScoreboardSidebar -> (not mapped yet)
//   Hud.extractChat -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//   Hud.extractItemHotbar -> (not mapped yet)
//   Hud.extractOverlayMessage -> (not mapped yet)
//   Hud.extractSelectedItemName -> (not mapped yet)
//   Hud.extractTabList -> (not mapped yet)
//   ChatComponent.extractRenderState -> (not mapped yet)
//   DisplayMode.FOREGROUND -> (not mapped yet)
//   PlayerTabOverlay.extractRenderState -> (not mapped yet)
//   ContextualBar.extractBackground -> (not mapped yet)
//   ContextualBar.extractExperienceLevel -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHud.java:49:
// | @Inject(
// |    method = "extractTabList",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/components/PlayerTabOverlay;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;ILnet/minecraft/world/scores/Scoreboard;Lnet/minecraft/world/scores/Objective;)V",
// |       shift = Shift.BEFORE
// |    ),
// |    cancellable = true
// | )
// | public void renderPlayerList(GuiGraphicsExtractor context, DeltaTracker tickCounter, CallbackInfo ci) {
// |    if (RenderEvents.postTablistLayerEventPre(context).isCancelled()) {
// |       ci.cancel();
// |    }
// | }
