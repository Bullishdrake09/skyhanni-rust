// Not ported yet: the Java @Inject(HEAD) on ClientLevel.addDestroyBlockEffect (MixinClientLevel.onAddBlockBreakParticles).
// Why it's off: No Horizon function mapped for ClientLevel.addDestroyBlockEffect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientLevel.addDestroyBlockEffect: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.addBreakingBlockEffect -> (not mapped yet)
//   ClientLevel.addDestroyBlockEffect -> (not mapped yet)
//   ClientLevel.addEntity -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinClientLevel.java:22:
// | @Inject(method = "addDestroyBlockEffect", at = @At("HEAD"), cancellable = true)
// | private void onAddBlockBreakParticles(BlockPos blockPos, BlockState blockState, CallbackInfo ci) {
// |    if (ParticleHider.shouldHideBlockParticles()) {
// |       ci.cancel();
// |    }
// | }
