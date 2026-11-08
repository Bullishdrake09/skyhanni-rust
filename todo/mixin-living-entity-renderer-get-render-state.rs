// Not ported yet: the Java @Inject(HEAD) on LivingEntityRenderer.getRenderType (MixinLivingEntityRenderer.getRenderState).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.getRenderType (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LivingEntityRenderer.getRenderType: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinLivingEntityRenderer.java:73:
// | @Inject(method = "getRenderType", at = @At("HEAD"), cancellable = true)
// | public void getRenderState(LivingEntityRenderState state, boolean showBody, boolean translucent, boolean showOutline, CallbackInfoReturnable<RenderType> cir) {
// |    if (showBody && EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       if (EntityTransparencyManager.getEntityTransparency(livingEntity) == null) {
// |          return;
// |       }
// |
// |       cir.setReturnValue(RenderTypes.entityTranslucentCullItemTarget(this.getTextureLocation(state)));
// |    }
// | }
