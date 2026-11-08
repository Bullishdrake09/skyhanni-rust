// Not ported yet: the Java @Inject(HEAD) on EnderMan.teleport (MixinEnderMan.onLivingUpdate).
// Why it's off: No Horizon function mapped for EnderMan.teleport (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EnderMan.teleport: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EnderMan.teleport -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEnderMan.java:12:
// | @Inject(method = "teleport(DDD)Z", at = @At("HEAD"), cancellable = true)
// | private void onLivingUpdate(double x, double y, double z, CallbackInfoReturnable<Boolean> cir) {
// |    if (new EndermanTeleportEvent().post().isCancelled()) {
// |       cir.setReturnValue(false);
// |    }
// | }
