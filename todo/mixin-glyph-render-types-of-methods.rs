// Not ported yet: the Java @ModifyReturnValue(RETURN) on GlyphRenderTypes.createForColorTexture (MixinGlyphRenderTypes.ofMethods).
// Why it's off: No Horizon function mapped for GlyphRenderTypes.createForColorTexture (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for GlyphRenderTypes.createForColorTexture: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GlyphRenderTypes.createForColorTexture -> (not mapped yet)
//   GlyphRenderTypes.createForIntensityTexture -> (not mapped yet)
//   GlyphRenderTypes.select -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGlyphRenderTypes.java:21:
// | @ModifyReturnValue(method = {"createForColorTexture", "createForIntensityTexture"}, at = @At("RETURN"))
// | private static GlyphRenderTypes ofMethods(GlyphRenderTypes original, @Local(argsOnly = true) Identifier identifier) {
// |    ((MixinGlyphRenderTypes)original).skyhanni$identifier = identifier;
// |    return original;
// | }
