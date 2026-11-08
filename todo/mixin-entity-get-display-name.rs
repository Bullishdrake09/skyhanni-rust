// Not ported yet: the Java @Inject(RETURN) on Entity.getDisplayName (MixinEntity.getDisplayName).
// Why it's off: No Horizon function mapped for Entity.getDisplayName (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Entity.getDisplayName: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.getDisplayName -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntity.java:13:
// | @Inject(method = "getDisplayName", at = @At("RETURN"), cancellable = true)
// | public void getDisplayName(CallbackInfoReturnable<Component> cir) {
// |    cir.setReturnValue(EntityData.getDisplayName((Entity)this, (Component)cir.getReturnValue()));
// | }
