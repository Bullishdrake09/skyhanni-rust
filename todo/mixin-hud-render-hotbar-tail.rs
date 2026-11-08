// Not ported yet: the Java @Inject(TAIL) on Hud.extractItemHotbar (MixinHud.renderHotbarTail).
// Why it's off: No Horizon function mapped for Hud.extractItemHotbar (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Hud.extractItemHotbar: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHud.java:44:
// | @Inject(method = "extractItemHotbar", at = @At("TAIL"))
// | public void renderHotbarTail(GuiGraphicsExtractor context, DeltaTracker tickCounter, CallbackInfo ci) {
// |    RenderEvents.postHotbarLayerEventPost(context);
// | }
