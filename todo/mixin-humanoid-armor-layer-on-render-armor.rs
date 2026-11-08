// Not ported yet: the Java @Inject(HEAD) on HumanoidArmorLayer.renderArmorPiece (MixinHumanoidArmorLayer.onRenderArmor).
// Why it's off: No Horizon function mapped for HumanoidArmorLayer.renderArmorPiece (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for HumanoidArmorLayer.renderArmorPiece: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidArmorLayer.renderArmorPiece -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinHumanoidArmorLayer.java:17:
// | @Inject(method = "renderArmorPiece", at = @At("HEAD"), cancellable = true)
// | private void onRenderArmor(
// |    PoseStack poseStack,
// |    SubmitNodeCollector submitNodeCollector,
// |    ItemStack itemStack,
// |    EquipmentSlot slot,
// |    int i,
// |    HumanoidRenderState humanoidRenderState,
// |    CallbackInfo ci
// | ) {
// |    if (HideArmorHookKt.shouldHideSlot(slot)) {
// |       ci.cancel();
// |    }
// | }
