// Not ported yet: the Java @WrapWithCondition(INVOKE) on CustomHeadLayer.submit (MixinCustomHeadLayer.onRenderItemstackOnHead).
// Why it's off: No Horizon function mapped for CustomHeadLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for CustomHeadLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SkullBlockRenderer.submitSkull -> (not mapped yet)
//   CustomHeadLayer.submit -> (not mapped yet)
//   ItemStackRenderState.submit -> horizon_client::item_render::ItemRenderState::submit (suggested, high)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinCustomHeadLayer.java:37:
// | @WrapWithCondition(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;FF)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/item/ItemStackRenderState;submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V"
// |    )
// | )
// | private boolean onRenderItemstackOnHead(ItemStackRenderState instance, PoseStack matrices, SubmitNodeCollector submitNodeCollector, int i, int j, int k) {
// |    return !HideArmorHookKt.shouldHideArmor();
// | }
