// Not ported yet: the Java @WrapOperation(INVOKE) on PreparedTextBuilder.accept (MixinPreparedTextBuilder.wrapGetColor).
// Why it's off: No Horizon function mapped for PreparedTextBuilder.accept (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for PreparedTextBuilder.accept: run `match`.
// Note: The call it hooks (Style.getColor) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GlyphVisitor.acceptGlyph -> (not mapped yet)
//   PreparedTextBuilder.accept -> (not mapped yet)
//   PreparedTextBuilder.visit -> (not mapped yet)
//   BakedGlyph.createGlyph -> (not mapped yet)
//   Style.getColor -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinPreparedTextBuilder.java:30:
// | @WrapOperation(
// |    method = "accept(ILnet/minecraft/network/chat/Style;Lnet/minecraft/client/gui/font/glyphs/BakedGlyph;)Z",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/network/chat/Style;getColor()Lnet/minecraft/network/chat/TextColor;")
// | )
// | private TextColor wrapGetColor(Style original, Operation<TextColor> operation) {
// |    return ChromaFontManager.forceWhiteTextColorForChroma((TextColor)operation.call(new Object[]{original}));
// | }
