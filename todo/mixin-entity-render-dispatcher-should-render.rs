// Not ported yet: the Java @Inject(HEAD) on EntityRenderDispatcher.shouldRender (MixinEntityRenderDispatcher.shouldRender).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.shouldRender (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EntityRenderDispatcher.shouldRender: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderDispatcher.shouldRender -> (not mapped yet)
//   EntityRenderDispatcher.submit -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntityRenderDispatcher.java:59:
// | @Inject(method = "shouldRender", at = @At("HEAD"), cancellable = true)
// | private void shouldRender(Entity entity, Frustum camera, double camX, double camY, double camZ, CallbackInfoReturnable<Boolean> cir) {
// |    if (!EntityData.shouldRender(entity, camX, camY, camZ)) {
// |       cir.setReturnValue(false);
// |    }
// | }
