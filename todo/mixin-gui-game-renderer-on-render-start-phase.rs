// Not ported yet: the Java @Inject(INVOKE) on Gui.extractRenderState (MixinGui_GameRenderer.onRenderStartPhase).
// Why it's off: No Horizon function mapped for Gui.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for Gui.extractRenderState: run `match`.
// Note: The call it hooks (Hud.extractRenderState) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Gui.extractRenderState -> (not mapped yet)
//   Hud.extractRenderState -> (not mapped yet)
//   Hud.extractSavingIndicator -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGui_GameRenderer.java:28:
// | @Inject(
// |    method = "extractRenderState(Lnet/minecraft/client/DeltaTracker;ZZ)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/Hud;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V"
// |    )
// | )
// | private void onRenderStartPhase(DeltaTracker deltaTracker, boolean shouldRenderLevel, boolean resourcesLoaded, CallbackInfo ci) {
// |    if (MinecraftCompat.getLocalPlayerExists()) {
// |       new RenderingTickEvent(this.skyhanni$guiGraphics, true).post();
// |    }
// | }
