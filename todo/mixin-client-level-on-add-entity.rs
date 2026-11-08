// Not ported yet: the Java @Inject(HEAD) on ClientLevel.addEntity (MixinClientLevel.onAddEntity).
// Why it's off: No Horizon function mapped for ClientLevel.addEntity (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientLevel.addEntity: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.addBreakingBlockEffect -> (not mapped yet)
//   ClientLevel.addDestroyBlockEffect -> (not mapped yet)
//   ClientLevel.addEntity -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinClientLevel.java:17:
// | @Inject(method = "addEntity", at = @At("HEAD"))
// | private void onAddEntity(Entity entity, CallbackInfo ci) {
// |    new EntityEnterWorldEvent<>(entity).post();
// | }
