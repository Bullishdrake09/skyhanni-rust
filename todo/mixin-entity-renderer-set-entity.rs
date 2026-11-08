// Not ported yet: the Java @Inject(TAIL) on EntityRenderer.extractRenderState (MixinEntityRenderer.setEntity).
// Why it's off: No Horizon function mapped for EntityRenderer.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EntityRenderer.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitNameTag -> (not mapped yet)
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   EntityRenderer.submitNameDisplay -> (not mapped yet)
//   EntityRenderState.skyhanni$setEntity -> (not mapped yet)
//   EntityRenderState.x -> (not mapped yet)
//   EntityRenderState.y -> (not mapped yet)
//   EntityRenderState.z -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntityRenderer.java:49:
// | @Inject(method = "extractRenderState", at = @At("TAIL"))
// | public void setEntity(Entity entity, EntityRenderState state, float tickProgress, CallbackInfo ci) {
// |    state.skyhanni$setEntity(entity);
// | }
