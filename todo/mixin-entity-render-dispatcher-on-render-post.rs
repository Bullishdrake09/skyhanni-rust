// Not ported yet: the Java @Inject(RETURN) on EntityRenderDispatcher.submit (MixinEntityRenderDispatcher.onRenderPost).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for EntityRenderDispatcher.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderDispatcher.shouldRender -> (not mapped yet)
//   EntityRenderDispatcher.submit -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntityRenderDispatcher.java:41:
// | @Inject(method = "submit", at = @At("RETURN"))
// | public void onRenderPost(
// |    EntityRenderState entityRenderState,
// |    CameraRenderState cameraRenderState,
// |    double d,
// |    double e,
// |    double f,
// |    PoseStack poseStack,
// |    SubmitNodeCollector submitNodeCollector,
// |    CallbackInfo ci
// | ) {
// |    if (EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       new SkyHanniRenderEntityEvent.Post<>(livingEntity, d, e, f).post();
// |    }
// |
// |    EntityRenderDispatcherHookKt.clearEntity();
// | }
