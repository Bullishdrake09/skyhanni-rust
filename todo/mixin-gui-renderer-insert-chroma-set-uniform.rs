// Not ported yet: the Java @Inject(INVOKE) on GuiRenderer.executeDrawRange (MixinGuiRenderer.insertChromaSetUniform).
// Why it's off: No Horizon function mapped for GuiRenderer.executeDrawRange (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for GuiRenderer.executeDrawRange: run `match`.
// Note: The call it hooks (RenderPass.setUniform) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiRenderer.java:46:
// | @Inject(
// |    method = "executeDrawRange",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/blaze3d/systems/RenderPass;setUniform(Ljava/lang/String;Lcom/mojang/blaze3d/buffers/GpuBufferSlice;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | public void insertChromaSetUniform(CallbackInfo ci, @Local RenderPass renderPass) {
// |    GuiRendererHook.INSTANCE.insertChromaSetUniform(renderPass);
// | }
