// Not ported yet: the Java @Inject(HEAD) on ClientLevel.addBreakingBlockEffect (MixinClientLevel.onAddBlockBreakingParticles).
// Why it's off: No Horizon function mapped for ClientLevel.addBreakingBlockEffect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientLevel.addBreakingBlockEffect: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.addBreakingBlockEffect -> (not mapped yet)
//   ClientLevel.addDestroyBlockEffect -> (not mapped yet)
//   ClientLevel.addEntity -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinClientLevel.java:29:
// | @Inject(method = "addBreakingBlockEffect", at = @At("HEAD"), cancellable = true)
// | private void onAddBlockBreakingParticles(BlockPos blockPos, Direction direction, CallbackInfo ci) {
// |    if (ParticleHider.shouldHideBlockParticles()) {
// |       ci.cancel();
// |    }
// | }
