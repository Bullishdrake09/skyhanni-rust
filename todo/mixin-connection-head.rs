// Not ported yet: the Java @Inject(HEAD) on Connection.genericsFtw (MixinConnection.handlePacket$Inject$HEAD).
// Why it's off: No Horizon function mapped for Connection.genericsFtw (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Connection.genericsFtw: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Connection.genericsFtw -> (not mapped yet)
//   Connection.send -> (not mapped yet)
//   BundlePacket.subPackets -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinConnection.java:17:
// | @Inject(method = "genericsFtw", at = @At("HEAD"))
// | private static void handlePacket$Inject$HEAD(Packet<?> packet, PacketListener listener, CallbackInfo ci) {
// |    if (packet instanceof BundlePacket<?> bundle) {
// |       for (Packet<?> subPacket : bundle.subPackets()) {
// |          new PacketReceivedEvent(subPacket).post();
// |       }
// |    } else {
// |       new PacketReceivedEvent(packet).post();
// |    }
// | }
