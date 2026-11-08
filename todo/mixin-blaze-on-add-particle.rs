// Not ported yet: the Java @WrapOperation(INVOKE) on Blaze.aiStep (MixinBlaze.onAddParticle).
// Why it's off: No Horizon function mapped for Blaze.aiStep (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Blaze.aiStep: run `match`.
// Note: The call it hooks (Level.addParticle) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Blaze.aiStep -> (not mapped yet)
//   Level.addParticle -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinBlaze.java:14:
// | @WrapOperation(
// |    method = "aiStep",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V")
// | )
// | private void onAddParticle(Level level, ParticleOptions particle, double x, double y, double z, double xd, double yd, double zd, Operation<Void> original) {
// |    if (!ParticleHider.shouldHideBlazeParticles()) {
// |       original.call(new Object[]{level, particle, x, y, z, xd, yd, zd});
// |    }
// | }
