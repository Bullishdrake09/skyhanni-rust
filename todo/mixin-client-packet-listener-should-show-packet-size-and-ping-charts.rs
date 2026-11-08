// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on ClientPacketListener.tick (MixinClientPacketListener.shouldShowPacketSizeAndPingCharts).
// Why it's off: No Horizon function mapped for ClientPacketListener.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for ClientPacketListener.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.addParticle -> (not mapped yet)
//   ClientPacketListener.handleParticleEvent -> (not mapped yet)
//   ClientPacketListener.tick -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinClientPacketListener.java:21:
// | @ModifyExpressionValue(method = "tick", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/components/DebugScreenOverlay;showNetworkCharts()Z"))
// | public boolean shouldShowPacketSizeAndPingCharts(boolean original) {
// |    return !CurrentPing.INSTANCE.isEnabled() ? original : true;
// | }
