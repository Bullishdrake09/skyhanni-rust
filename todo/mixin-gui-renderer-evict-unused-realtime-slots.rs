// Not ported yet: the Java @Inject(TAIL) on GuiRenderer.preparePictureInPicture (MixinGuiRenderer.skyhanni$evictUnusedRealtimeSlots).
// Why it's off: No Horizon function mapped for GuiRenderer.preparePictureInPicture (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for GuiRenderer.preparePictureInPicture: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiRenderer.java:79:
// | @Inject(method = "preparePictureInPicture", at = @At("TAIL"))
// | private void skyhanni$evictUnusedRealtimeSlots(CallbackInfo ci) {
// |    SkyHanniItemRenderCoordinator.evictUnusedRealtimeSlots(this.skyhanni$frameNumber);
// | }
