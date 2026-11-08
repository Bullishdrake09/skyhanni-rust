// Not ported yet: the Java @WrapOperation(INVOKE) on ClientPacketListener.handleParticleEvent (MixinClientPacketListener.wrapAddParticle).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleParticleEvent (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for ClientPacketListener.handleParticleEvent: run `match`.
// Note: The call it hooks (ClientLevel.addParticle) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.addParticle -> (not mapped yet)
//   ClientPacketListener.handleParticleEvent -> (not mapped yet)
//   ClientPacketListener.tick -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinClientPacketListener.java:38:
// | @WrapOperation(
// |    method = "handleParticleEvent",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/multiplayer/ClientLevel;addParticle(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDDDD)V")
// | )
// | public void wrapAddParticle(
// |    ClientLevel level,
// |    ParticleOptions particleOptions,
// |    boolean overrideLimiter,
// |    boolean alwaysShow,
// |    double x,
// |    double y,
// |    double z,
// |    double xd,
// |    double yd,
// |    double zd,
// |    Operation<Void> original,
// |    @Local(argsOnly = true) ClientboundLevelParticlesPacket packet
// | ) {
// |    ParticleUtils.wrapAddParticle(level, particleOptions, overrideLimiter, alwaysShow, x, y, z, xd, yd, zd, original, packet);
// | }
