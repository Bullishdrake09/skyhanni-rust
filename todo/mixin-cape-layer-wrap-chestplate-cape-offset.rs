// Not ported yet: the Java @WrapWithCondition(INVOKE) on CapeLayer.submit (MixinCapeLayer.wrapChestplateCapeOffset).
// Why it's off: No Horizon function mapped for CapeLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for CapeLayer.submit: run `match`.
// Note: The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.translate -> (not mapped yet)
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   CapeLayer.submit -> (not mapped yet)
//   RenderTypes.entityTranslucentCullItemTarget -> (not mapped yet)
//   Texture.texturePath -> (not mapped yet)
//   EquipmentSlot.CHEST -> (not mapped yet)
//   PlayerSkin.cape -> horizon_client_render::skins::cape (suggested, high)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinCapeLayer.java:21:
// | @WrapWithCondition(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V",
// |    at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V")
// | )
// | private boolean wrapChestplateCapeOffset(PoseStack instance, float x, float y, float z) {
// |    return !HideArmorHookKt.shouldHideSlot(EquipmentSlot.CHEST);
// | }
