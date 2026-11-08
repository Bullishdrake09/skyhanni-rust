// Not ported yet: the Java @Inject(HEAD) on EntityRenderDispatcher.submit (MixinEntityRenderDispatcher.onRenderPre).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EntityRenderDispatcher.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderDispatcher.shouldRender -> (not mapped yet)
//   EntityRenderDispatcher.submit -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntityRenderDispatcher.java:22:
// | @Inject(method = "submit", at = @At("HEAD"), cancellable = true)
// | public void onRenderPre(
// |    EntityRenderState renderState,
// |    CameraRenderState cameraRenderState,
// |    double d,
// |    double e,
// |    double f,
// |    PoseStack poseStack,
// |    SubmitNodeCollector submitNodeCollector,
// |    CallbackInfo ci
// | ) {
// |    EntityRenderDispatcherHookKt.setEntity(renderState);
// |    if (EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity
// |       && !EntityRenderDispatcherHookKt.getActiveHolographicEntities().contains(livingEntity)
// |       && new SkyHanniRenderEntityEvent.Pre<>(livingEntity, d, e, f).post().isCancelled()) {
// |       ci.cancel();
// |    }
// | }
