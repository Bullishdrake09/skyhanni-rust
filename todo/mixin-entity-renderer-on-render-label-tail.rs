// Not ported yet: the Java @Inject(TAIL) on EntityRenderer.submitNameDisplay (MixinEntityRenderer.onRenderLabelTail).
// Why it's off: No Horizon function mapped for EntityRenderer.submitNameDisplay (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EntityRenderer.submitNameDisplay: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntityRenderer.java:37:
// | @Inject(
// |    method = "submitNameDisplay(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At("TAIL")
// | )
// | public void onRenderLabelTail(
// |    EntityRenderState state, PoseStack poseStack, SubmitNodeCollector submitNodeCollector, CameraRenderState cameraRenderState, CallbackInfo ci
// | ) {
// |    if (EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       new SkyHanniRenderEntityEvent.Specials.Post<>(livingEntity, state.x, state.y, state.z).post();
// |    }
// | }
