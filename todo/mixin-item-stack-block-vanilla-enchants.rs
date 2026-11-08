// Not ported yet: the Java @Inject(INVOKE) on ItemStack.addToTooltip (MixinItemStack.blockVanillaEnchants).
// Why it's off: No Horizon function mapped for ItemStack.addToTooltip (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ItemStack.addToTooltip: run `match`.
// Note: The call it hooks (TooltipProvider.addToTooltip) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DataComponents.ENCHANTMENTS -> (not mapped yet)
//   ItemStack.addToTooltip -> (not mapped yet)
//   ItemStack.isEmpty -> (not mapped yet)
//   TooltipProvider.addToTooltip -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinItemStack.java:35:
// | @Inject(
// |    method = "addToTooltip",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/item/component/TooltipProvider;addToTooltip(Lnet/minecraft/world/item/Item$TooltipContext;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;Lnet/minecraft/core/component/DataComponentGetter;)V"
// |    ),
// |    cancellable = true
// | )
// | public <T extends TooltipProvider> void blockVanillaEnchants(
// |    DataComponentType<T> componentType,
// |    TooltipContext context,
// |    TooltipDisplay displayComponent,
// |    Consumer<Component> textConsumer,
// |    TooltipFlag type,
// |    CallbackInfo ci
// | ) {
// |    if (SkyBlockUtils.getInSkyBlock()
// |       && SkyHanniMod.feature.getInventory().getEnchantParsing().getHideVanillaEnchants().get()
// |       && componentType == DataComponents.ENCHANTMENTS) {
// |       ci.cancel();
// |    }
// | }
