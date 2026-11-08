// Not ported yet: the Java @Inject(HEAD) on AbstractHurtingProjectile.createParticleTrail (MixinAbstractHurtingProjectile.onCreateParticleTrail).
// Why it's off: No Horizon function mapped for AbstractHurtingProjectile.createParticleTrail (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AbstractHurtingProjectile.createParticleTrail: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AbstractHurtingProjectile.createParticleTrail -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAbstractHurtingProjectile.java:13:
// | @Inject(method = "createParticleTrail", at = @At("HEAD"), cancellable = true)
// | private void onCreateParticleTrail(CallbackInfo ci) {
// |    if (this instanceof Fireball && ParticleHider.shouldHideFireballParticles()) {
// |       ci.cancel();
// |    }
// | }
