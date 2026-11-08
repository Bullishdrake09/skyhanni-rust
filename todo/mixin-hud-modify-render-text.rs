// Not ported yet: the Java @ModifyArg(INVOKE) on Hud.extractChat (MixinHud.modifyRenderText).
// Why it's off: No Horizon function mapped for Hud.extractChat (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for Hud.extractChat: run `match`.
// Note: The call it hooks (ChatComponent.extractRenderState) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHud.java:130:
// | @ModifyArg(
// |    method = "extractChat",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/components/ChatComponent;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;IIILnet/minecraft/client/gui/components/ChatComponent$DisplayMode;Z)V"
// |    ),
// |    index = 5
// | )
// | private DisplayMode modifyRenderText(DisplayMode mode) {
// |    return ChatPeek.peek() ? DisplayMode.FOREGROUND : mode;
// | }
