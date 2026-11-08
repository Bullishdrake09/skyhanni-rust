// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on SimpleContainer.getItem (MixinSimpleContainer.getStack).
// Why it's off: No Horizon function mapped for SimpleContainer.getItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for SimpleContainer.getItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SimpleContainer.getItem -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinSimpleContainer.java:13:
// | @ModifyExpressionValue(method = "getItem", at = @At(value = "INVOKE", target = "Lnet/minecraft/core/NonNullList;get(I)Ljava/lang/Object;"))
// | public Object getStack(Object original, @Local(argsOnly = true) int slot) {
// |    return ReplaceItemEvent.postEvent((SimpleContainer)this, (ItemStack)original, slot);
// | }
