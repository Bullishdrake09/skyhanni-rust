// Not ported yet: the Java @ModifyArg(INVOKE) on GuiGraphicsExtractor.text (MixinGuiGraphicsExtractor.modifyVisualWordsTextState).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.text (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for GuiGraphicsExtractor.text: run `match`.
// Note: The call it hooks (GuiRenderState.addText) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiGraphicsExtractor.java:56:
// | @ModifyArg(
// |    method = "text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/util/FormattedCharSequence;IIIZ)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/state/gui/GuiRenderState;addText(Lnet/minecraft/client/renderer/state/gui/GuiTextRenderState;)V"
// |    ),
// |    index = 0
// | )
// | private GuiTextRenderState modifyVisualWordsTextState(GuiTextRenderState textState) {
// |    if (!VisualWordsHook.isCaxtonLoaded()) {
// |       return textState;
// |    }
// |
// |    FormattedCharSequence text = VisualWordsHook.modifyOrderedText(textState.text);
// |    return text == textState.text
// |       ? textState
// |       : new GuiTextRenderState(
// |          textState.font,
// |          text,
// |          textState.pose,
// |          textState.x,
// |          textState.y,
// |          textState.color,
// |          textState.backgroundColor,
// |          textState.dropShadow,
// |          false,
// |          textState.scissor
// |       );
// | }
