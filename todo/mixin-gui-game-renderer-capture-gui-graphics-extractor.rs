// Not ported yet: the Java @ModifyVariable(STORE) on Gui.extractRenderState (MixinGui_GameRenderer.skyhanni$captureGuiGraphicsExtractor).
// Why it's off: No Horizon function mapped for Gui.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for Gui.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Gui.extractRenderState -> (not mapped yet)
//   Hud.extractRenderState -> (not mapped yet)
//   Hud.extractSavingIndicator -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGui_GameRenderer.java:22:
// | @ModifyVariable(method = "extractRenderState(Lnet/minecraft/client/DeltaTracker;ZZ)V", at = @At("STORE"), name = "graphics")
// | private GuiGraphicsExtractor skyhanni$captureGuiGraphicsExtractor(GuiGraphicsExtractor graphics) {
// |    this.skyhanni$guiGraphics = graphics;
// |    return graphics;
// | }
