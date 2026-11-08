// Not ported yet: the Java @ModifyArg(INVOKE) on EntityRenderer.submitNameDisplay (MixinEntityRenderer.modifyRenderLabelIfPresentArgs).
// Why it's off: No Horizon function mapped for EntityRenderer.submitNameDisplay (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for EntityRenderer.submitNameDisplay: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitNameTag) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitNameTag -> (not mapped yet)
//   EntityRenderer.extractRenderState -> (not mapped yet)
//   EntityRenderer.submitNameDisplay -> (not mapped yet)
//   EntityRenderState.skyhanni$setEntity -> (not mapped yet)
//   EntityRenderState.x -> (not mapped yet)
//   EntityRenderState.y -> (not mapped yet)
//   EntityRenderState.z -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinEntityRenderer.java:54:
// | @ModifyArg(
// |    method = "submitNameDisplay(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;I)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitNameTag(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/phys/Vec3;ILnet/minecraft/network/chat/Component;ZILnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |       ordinal = 0
// |    ),
// |    index = 3
// | )
// | private Component modifyRenderLabelIfPresentArgs(Component text) {
// |    return SkyBlockUtils.getInSkyBlock() ? EntityData.getHealthDisplay(text) : text;
// | }
