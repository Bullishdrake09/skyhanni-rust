// Not ported yet: the Java @Inject(INVOKE) on GuiGraphicsExtractor.componentHoverEffect (MixinGuiGraphicsExtractor.onRenderComponentHoverEffect).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.componentHoverEffect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for GuiGraphicsExtractor.componentHoverEffect: run `match`.
// Note: The call it hooks (Objects.requireNonNull) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiGraphicsExtractor.java:38:
// | @Inject(
// |    method = "componentHoverEffect",
// |    at = @At(value = "INVOKE", target = "Ljava/util/Objects;requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;", shift = Shift.AFTER)
// | )
// | private void onRenderComponentHoverEffect(Font font, Style style, int i, int j, CallbackInfo ci) {
// |    GuiChatHook.setReplacementComponent(null);
// |    new ChatHoverEvent(style.getHoverEvent()).post();
// | }
