// Not ported yet: the Java @Inject(INVOKE) on Hud.extractHotbarAndDecorations (MixinHud.renderExperienceBarTail).
// Why it's off: No Horizon function mapped for Hud.extractHotbarAndDecorations (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for Hud.extractHotbarAndDecorations: run `match`.
// Note: The call it hooks (ContextualBar.extractBackground) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHud.java:79:
// | @Inject(
// |    method = "extractHotbarAndDecorations",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/contextualbar/ContextualBar;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | public void renderExperienceBarTail(GuiGraphicsExtractor context, DeltaTracker deltaTracker, CallbackInfo ci) {
// |    RenderEvents.postExperienceBarLayerEventPost(context);
// | }
