// Not ported yet: the Java @Inject(HEAD) on ScreenEffectRenderer.submitFire (MixinScreenEffectRenderer.renderFire).
// Why it's off: No Horizon function mapped for ScreenEffectRenderer.submitFire (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ScreenEffectRenderer.submitFire: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ScreenEffectRenderer.submitBlockSprite -> (not mapped yet)
//   ScreenEffectRenderer.submitFire -> (not mapped yet)
//   ScreenEffectRenderer.submitWater -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinScreenEffectRenderer.java:13:
// | @Inject(method = "submitFire", at = @At("HEAD"), cancellable = true)
// | private static void renderFire(CallbackInfo ci) {
// |    if (new BlockOverlayRenderEvent(OverlayType.FIRE).post().isCancelled()) {
// |       ci.cancel();
// |    }
// | }
