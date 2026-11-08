// Not ported yet: the Java @WrapOperation(INVOKE) on BaseFireBlock.animateTick (MixinBaseFireBlock.onAddParticle).
// Why it's off: No Horizon function mapped for BaseFireBlock.animateTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for BaseFireBlock.animateTick: run `match`.
// Note: The call it hooks (Level.addParticle) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Level.addParticle -> (not mapped yet)
//   BaseFireBlock.animateTick -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinBaseFireBlock.java:14:
// | @WrapOperation(
// |    method = "animateTick",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V")
// | )
// | private void onAddParticle(Level level, ParticleOptions particle, double x, double y, double z, double xd, double yd, double zd, Operation<Void> original) {
// |    if (!ParticleHider.shouldHideFireParticles()) {
// |       original.call(new Object[]{level, particle, x, y, z, xd, yd, zd});
// |    }
// | }
