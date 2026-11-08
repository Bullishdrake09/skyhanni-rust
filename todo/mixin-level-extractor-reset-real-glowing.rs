// Not ported yet: the Java @Inject(HEAD) on LevelExtractor.extractVisibleEntities (MixinLevelExtractor.resetRealGlowing).
// Why it's off: No Horizon function mapped for LevelExtractor.extractVisibleEntities (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LevelExtractor.extractVisibleEntities: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelExtractor.extractVisibleEntities -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinLevelExtractor.java:12:
// | @Inject(method = "extractVisibleEntities", at = @At("HEAD"))
// | public void resetRealGlowing(CallbackInfo ci) {
// |    RenderLivingEntityHelper.postNoXrayOutlineEvent();
// | }
