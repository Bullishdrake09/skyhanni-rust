// Not ported yet: the Java @Inject(HEAD) on GlyphRenderTypes.select (MixinGlyphRenderTypes.getRenderLayer).
// Why it's off: No Horizon function mapped for GlyphRenderTypes.select (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GlyphRenderTypes.select: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GlyphRenderTypes.createForColorTexture -> (not mapped yet)
//   GlyphRenderTypes.createForIntensityTexture -> (not mapped yet)
//   GlyphRenderTypes.select -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGlyphRenderTypes.java:27:
// | @Inject(method = "select", at = @At("HEAD"), cancellable = true)
// | private void getRenderLayer(CallbackInfoReturnable<RenderType> cir) {
// |    if (ChromaFontManager.getGlyphIsChroma()) {
// |       cir.setReturnValue(SkyHanniRenderLayers.INSTANCE.getChromaTexturedWithIdentifier(this.skyhanni$identifier));
// |    }
// | }
