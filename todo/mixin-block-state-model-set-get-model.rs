// Not ported yet: the Java @Inject(HEAD) on BlockStateModelSet.get (MixinBlockStateModelSet.getModel).
// Why it's off: No Horizon function mapped for BlockStateModelSet.get (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BlockStateModelSet.get: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockStateModelSet.get -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinBlockStateModelSet.java:14:
// | @Inject(method = "get", at = @At("HEAD"), cancellable = true)
// | public void getModel(BlockState state, CallbackInfoReturnable<BlockStateModel> cir) {
// |    BlockRendererDispatcherHookKt.modifyGetModelFromBlockState((BlockStateModelSet)this, state, cir);
// | }
