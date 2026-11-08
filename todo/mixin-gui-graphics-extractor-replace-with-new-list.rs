// Not ported yet: the Java @ModifyArg(INVOKE) on GuiGraphicsExtractor.componentHoverEffect (MixinGuiGraphicsExtractor.replaceWithNewList).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.componentHoverEffect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for GuiGraphicsExtractor.componentHoverEffect: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Objects.requireNonNull -> (not mapped yet)
//   Font.split -> horizon_client_render::font::Font::split_formatted (suggested, high)
//   GuiGraphicsExtractor.componentHoverEffect -> (not mapped yet)
//   GuiGraphicsExtractor.item -> (not mapped yet)
//   GuiGraphicsExtractor.itemDecorations -> horizon_client_render::hud_overlays::item_decorations (suggested, high)
//   GuiGraphicsExtractor.text -> (not mapped yet)
//   GuiRenderState.addText -> (not mapped yet)
//   GuiTextRenderState.<init> -> (not mapped yet)
//   GuiTextRenderState.backgroundColor -> (not mapped yet)
//   GuiTextRenderState.color -> (not mapped yet)
//   GuiTextRenderState.dropShadow -> (not mapped yet)
//   GuiTextRenderState.font -> (not mapped yet)
//   GuiTextRenderState.pose -> (not mapped yet)
//   GuiTextRenderState.scissor -> (not mapped yet)
//   GuiTextRenderState.text -> (not mapped yet)
//   GuiTextRenderState.x -> (not mapped yet)
//   GuiTextRenderState.y -> (not mapped yet)
//   Style.getHoverEvent -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiGraphicsExtractor.java:47:
// | @ModifyArg(
// |    method = "componentHoverEffect",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Font;split(Lnet/minecraft/network/chat/FormattedText;I)Ljava/util/List;"),
// |    index = 0
// | )
// | private FormattedText replaceWithNewList(FormattedText originalComponent) {
// |    return (FormattedText)(GuiChatHook.getReplacementComponent() != null ? GuiChatHook.getReplacement() : originalComponent);
// | }
