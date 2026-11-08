// Not ported yet: the Java @ModifyArg(INVOKE) on PreparedTextBuilder.accept (MixinPreparedTextBuilder.forceChromaIfNecessary).
// Why it's off: No Horizon function mapped for PreparedTextBuilder.accept (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for PreparedTextBuilder.accept: run `match`.
// Note: The call it hooks (BakedGlyph.createGlyph) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GlyphVisitor.acceptGlyph -> (not mapped yet)
//   PreparedTextBuilder.accept -> (not mapped yet)
//   PreparedTextBuilder.visit -> (not mapped yet)
//   BakedGlyph.createGlyph -> (not mapped yet)
//   Style.getColor -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinPreparedTextBuilder.java:38:
// | @ModifyArg(
// |    method = "accept(ILnet/minecraft/network/chat/Style;Lnet/minecraft/client/gui/font/glyphs/BakedGlyph;)Z",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/font/glyphs/BakedGlyph;createGlyph(FFIILnet/minecraft/network/chat/Style;FF)Lnet/minecraft/client/gui/font/TextRenderable$Styled;"
// |    )
// | )
// | private Style forceChromaIfNecessary(Style style) {
// |    return ChromaFontManager.forceChromaStyleIfNecessary(style);
// | }
