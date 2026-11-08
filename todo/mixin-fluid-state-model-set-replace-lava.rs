// Not ported yet: the Java @ModifyReturnValue(RETURN) on FluidStateModelSet.get (MixinFluidStateModelSet.replaceLava).
// Why it's off: No Horizon function mapped for FluidStateModelSet.get (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for FluidStateModelSet.get: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FluidStateModelSet.bake -> (not mapped yet)
//   FluidStateModelSet.get -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinFluidStateModelSet.java:22:
// | @ModifyReturnValue(method = "get", at = @At("RETURN"))
// | private FluidModel replaceLava(FluidModel original, FluidState state) {
// |    return LavaReplacement.getReplacementModel(state, original);
// | }
