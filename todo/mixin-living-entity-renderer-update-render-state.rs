// Not ported yet: the Java @Inject(TAIL) on LivingEntityRenderer.extractRenderState (MixinLivingEntityRenderer.updateRenderState).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LivingEntityRenderer.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.extractRenderState -> (not mapped yet)
//   LivingEntityRenderer.getRenderType -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//   LivingEntityRenderState.bodyRot -> (not mapped yet)
//   RenderTypes.entityTranslucentCullItemTarget -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinLivingEntityRenderer.java:36:
// | @Inject(
// |    method = "extractRenderState(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;F)V",
// |    at = @At("TAIL")
// | )
// | public void updateRenderState(LivingEntity livingEntity, LivingEntityRenderState livingEntityRenderState, float f, CallbackInfo ci) {
// |    if (livingEntity instanceof Player playerEntity) {
// |       Float yaw = RendererLivingEntityHook.rotatePlayer(playerEntity);
// |       if (yaw != null) {
// |          livingEntityRenderState.bodyRot = yaw;
// |       }
// |    }
// | }
