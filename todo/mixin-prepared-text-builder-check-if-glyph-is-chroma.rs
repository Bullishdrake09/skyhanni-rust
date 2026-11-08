// Not ported yet: the Java @Inject(INVOKE) on PreparedTextBuilder.visit (MixinPreparedTextBuilder.checkIfGlyphIsChroma).
// Why it's off: No Horizon function mapped for PreparedTextBuilder.visit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for PreparedTextBuilder.visit: run `match`.
// Note: The call it hooks (GlyphVisitor.acceptGlyph) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GlyphVisitor.acceptGlyph -> (not mapped yet)
//   PreparedTextBuilder.accept -> (not mapped yet)
//   PreparedTextBuilder.visit -> (not mapped yet)
//   BakedGlyph.createGlyph -> (not mapped yet)
//   Style.getColor -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinPreparedTextBuilder.java:20:
// | @Inject(
// |    method = "visit",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Font$GlyphVisitor;acceptGlyph(Lnet/minecraft/client/gui/font/TextRenderable$Styled;)V")
// | )
// | private void checkIfGlyphIsChroma(CallbackInfo ci, @Local Styled textDrawable) {
// |    if (textDrawable instanceof GlyphInstance drawnGlyph) {
// |       ChromaFontManager.checkIfGlyphIsChroma(drawnGlyph);
// |    }
// | }
