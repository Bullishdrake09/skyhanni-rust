// Not ported yet: the Java @Inject(RETURN) on Hud.extractOverlayMessage (MixinHud.renderOverlayMessagePost).
// Why it's off: No Horizon function mapped for Hud.extractOverlayMessage (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Hud.extractOverlayMessage: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHud.java:177:
// | @Inject(method = "extractOverlayMessage", at = @At("RETURN"))
// | public void renderOverlayMessagePost(GuiGraphicsExtractor context, DeltaTracker deltaTracker, CallbackInfo ci) {
// |    RenderEvents.postActionBarLayerEventPost(context);
// | }
