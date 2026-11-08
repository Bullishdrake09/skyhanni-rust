// Not ported yet: the Java @Inject(HEAD) on Connection.send (MixinConnection.sendPacketNew).
// Why it's off: No Horizon function mapped for Connection.send (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Connection.send: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Connection.genericsFtw -> (not mapped yet)
//   Connection.send -> (not mapped yet)
//   BundlePacket.subPackets -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinConnection.java:28:
// | @Inject(method = "send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;Z)V", at = @At("HEAD"), cancellable = true)
// | private void sendPacketNew(Packet<?> packet, ChannelFutureListener channelFutureListener, boolean flush, CallbackInfo ci) {
// |    if (new PacketSentEvent(packet).post().isCancelled()) {
// |       ci.cancel();
// |    }
// | }
