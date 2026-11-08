// Not ported yet: the Java @ModifyArg(INVOKE) on Hud.displayScoreboardSidebar (MixinHud.modifyScoreboardLine).
// Why it's off: No Horizon function mapped for Hud.displayScoreboardSidebar (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for Hud.displayScoreboardSidebar: run `match`.
// Note: The call it hooks (GuiGraphicsExtractor.text) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHud.java:118:
// | @ModifyArg(
// |    method = "displayScoreboardSidebar",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V"
// |    ),
// |    index = 1
// | )
// | private Component modifyScoreboardLine(Component str) {
// |    return ScoreboardData.tryToReplaceScoreboardLine(str);
// | }
