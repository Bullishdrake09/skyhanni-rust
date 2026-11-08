// Not ported yet: the Java @Inject(HEAD) on ItemStack.isEmpty (MixinItemStack.skyhanni$handleNullHolder).
// Why it's off: No Horizon function mapped for ItemStack.isEmpty (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemStack.isEmpty: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DataComponents.ENCHANTMENTS -> (not mapped yet)
//   ItemStack.addToTooltip -> (not mapped yet)
//   ItemStack.isEmpty -> (not mapped yet)
//   TooltipProvider.addToTooltip -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinItemStack.java:28:
// | @Inject(method = "isEmpty", at = @At("HEAD"), cancellable = true)
// | private void skyhanni$handleNullHolder(CallbackInfoReturnable<Boolean> cir) {
// |    if (this.item == null) {
// |       cir.setReturnValue(true);
// |    }
// | }
