// Not ported yet: the Java @WrapOperation(INVOKE) on GuiRenderer.addElementToMesh (MixinGuiRenderer.replacePipeline).
// Why it's off: No Horizon function mapped for GuiRenderer.addElementToMesh (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for GuiRenderer.addElementToMesh: run `match`.
// Note: The call it hooks (GuiElementRenderState.pipeline) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiRenderer.java:58:
// | @WrapOperation(
// |    method = "addElementToMesh",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/state/gui/GuiElementRenderState;pipeline()Lcom/mojang/blaze3d/pipeline/RenderPipeline;"
// |    )
// | )
// | public RenderPipeline replacePipeline(GuiElementRenderState state, Operation<RenderPipeline> original) {
// |    return GuiRendererHook.INSTANCE.replacePipeline(state, original);
// | }
