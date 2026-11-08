// Not ported yet: the Java @Inject(INVOKE) on Gui.extractRenderState (MixinGui_GameRenderer.onRenderTail).
// Why it's off: No Horizon function mapped for Gui.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for Gui.extractRenderState: run `match`.
// Note: The call it hooks (Hud.extractRenderState) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Gui.extractRenderState -> (not mapped yet)
//   Hud.extractRenderState -> (not mapped yet)
//   Hud.extractSavingIndicator -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGui_GameRenderer.java:54:
// | @Inject(
// |    method = "extractRenderState(Lnet/minecraft/client/DeltaTracker;ZZ)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/Hud;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | private void onRenderTail(DeltaTracker deltaTracker, boolean shouldRenderLevel, boolean resourcesLoaded, CallbackInfo ci) {
// |    GuiEditManager.renderLast(this.skyhanni$guiGraphics);
// | }
