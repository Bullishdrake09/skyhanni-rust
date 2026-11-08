// Not ported yet: the Java @Inject(HEAD) on GuiRenderer.executeDrawRange (MixinGuiRenderer.computeChromaBufferSlice).
// Why it's off: No Horizon function mapped for GuiRenderer.executeDrawRange (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GuiRenderer.executeDrawRange: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RenderPass.setUniform -> (not mapped yet)
//   GuiRenderer.addElementToMesh -> (not mapped yet)
//   GuiRenderer.executeDrawRange -> (not mapped yet)
//   GuiRenderer.preparePictureInPicture -> (not mapped yet)
//   GuiRenderer.preparePictureInPictureState -> (not mapped yet)
//   GuiRenderer.render -> (not mapped yet)
//   GuiElementRenderState.pipeline -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiRenderer.java:41:
// | @Inject(method = "executeDrawRange", at = @At("HEAD"))
// | public void computeChromaBufferSlice(CallbackInfo ci) {
// |    GuiRendererHook.INSTANCE.computeChromaBufferSlice();
// | }
