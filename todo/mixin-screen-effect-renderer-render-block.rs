// Not ported yet: the Java @Inject(HEAD) on ScreenEffectRenderer.submitBlockSprite (MixinScreenEffectRenderer.renderBlock).
// Why it's off: No Horizon function mapped for ScreenEffectRenderer.submitBlockSprite (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ScreenEffectRenderer.submitBlockSprite: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ScreenEffectRenderer.submitBlockSprite -> (not mapped yet)
//   ScreenEffectRenderer.submitFire -> (not mapped yet)
//   ScreenEffectRenderer.submitWater -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinScreenEffectRenderer.java:27:
// | @Inject(method = "submitBlockSprite", at = @At("HEAD"), cancellable = true)
// | private static void renderBlock(CallbackInfo ci) {
// |    if (new BlockOverlayRenderEvent(OverlayType.BLOCK).post().isCancelled()) {
// |       ci.cancel();
// |    }
// | }
