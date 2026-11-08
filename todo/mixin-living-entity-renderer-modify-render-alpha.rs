// Not ported yet: the Java @ModifyArg(INVOKE) on LivingEntityRenderer.submit (MixinLivingEntityRenderer.modifyRenderAlpha).
// Why it's off: No Horizon function mapped for LivingEntityRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for LivingEntityRenderer.submit: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   LivingEntityRenderer.extractRenderState -> (not mapped yet)
//   LivingEntityRenderer.getRenderType -> (not mapped yet)
//   LivingEntityRenderer.submit -> (not mapped yet)
//   LivingEntityRenderState.bodyRot -> (not mapped yet)
//   RenderTypes.entityTranslucentCullItemTarget -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinLivingEntityRenderer.java:49:
// | @ModifyArg(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/texture/TextureAtlasSprite;ILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V"
// |    ),
// |    index = 6
// | )
// | private int modifyRenderAlpha(int argb) {
// |    if (EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       Integer entityAlpha = EntityTransparencyManager.getEntityTransparency(livingEntity);
// |       if (entityAlpha == null) {
// |          return argb;
// |       }
// |
// |       int oldAlpha = argb >> 24 & 0xFF;
// |       int newAlpha = Math.min(oldAlpha, entityAlpha);
// |       argb &= 16777215;
// |       argb |= newAlpha << 24;
// |    }
// |
// |    return argb;
// | }
