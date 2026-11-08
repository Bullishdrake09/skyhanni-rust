// Not ported yet: the Java @Inject(RETURN) on FluidStateModelSet.bake (MixinFluidStateModelSet.bake).
// Why it's off: No Horizon function mapped for FluidStateModelSet.bake (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for FluidStateModelSet.bake: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FluidStateModelSet.bake -> (not mapped yet)
//   FluidStateModelSet.get -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinFluidStateModelSet.java:17:
// | @Inject(method = "bake", at = @At("RETURN"))
// | private static void bake(CallbackInfoReturnable<Map<Fluid, FluidModel>> cir) {
// |    LavaReplacement.onModelsBaked((Map<Fluid, FluidModel>)cir.getReturnValue());
// | }
