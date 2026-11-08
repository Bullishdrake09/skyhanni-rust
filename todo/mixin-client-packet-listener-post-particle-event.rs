// Not ported yet: the Java @Inject(INVOKE) on ClientPacketListener.handleParticleEvent (MixinClientPacketListener.postParticleEvent).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleParticleEvent (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ClientPacketListener.handleParticleEvent: run `match`.
// Note: The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.addParticle -> (not mapped yet)
//   ClientPacketListener.handleParticleEvent -> (not mapped yet)
//   ClientPacketListener.tick -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinClientPacketListener.java:26:
// | @Inject(
// |    method = "handleParticleEvent",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | public void postParticleEvent(ClientboundLevelParticlesPacket packet, CallbackInfo ci) {
// |    ParticleUtils.postParticleEvent(packet);
// | }
