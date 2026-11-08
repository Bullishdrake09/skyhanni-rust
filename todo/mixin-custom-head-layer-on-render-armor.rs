// Not ported yet: the Java @WrapWithCondition(INVOKE) on CustomHeadLayer.submit (MixinCustomHeadLayer.onRenderArmor).
// Why it's off: No Horizon function mapped for CustomHeadLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for CustomHeadLayer.submit: run `match`.
// Note: The call it hooks (SkullBlockRenderer.submitSkull) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SkullBlockRenderer.submitSkull -> (not mapped yet)
//   CustomHeadLayer.submit -> (not mapped yet)
//   ItemStackRenderState.submit -> horizon_client::item_render::ItemRenderState::submit (suggested, high)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinCustomHeadLayer.java:17:
// | @WrapWithCondition(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;FF)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/blockentity/SkullBlockRenderer;submitSkull(FLcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/model/object/skull/SkullModelBase;Lnet/minecraft/client/renderer/rendertype/RenderType;ILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V"
// |    )
// | )
// | private boolean onRenderArmor(
// |    float animationPos,
// |    PoseStack matrices,
// |    SubmitNodeCollector submitNodeCollector,
// |    int i,
// |    SkullModelBase skullModelBase,
// |    RenderType renderType,
// |    int j,
// |    CrumblingOverlay crumblingOverlay
// | ) {
// |    return !HideArmorHookKt.shouldHideArmor();
// | }
