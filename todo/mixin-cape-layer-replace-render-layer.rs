// Not ported yet: the Java @ModifyArg(INVOKE) on CapeLayer.submit (MixinCapeLayer.replaceRenderLayer).
// Why it's off: No Horizon function mapped for CapeLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for CapeLayer.submit: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinCapeLayer.java:29:
// | @ModifyArg(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V"
// |    ),
// |    index = 3
// | )
// | private RenderType replaceRenderLayer(RenderType original, @Local PlayerSkin skinTextures) {
// |    if (skinTextures.cape() != null && EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       Integer entityAlpha = EntityTransparencyManager.getEntityTransparency(livingEntity);
// |       return entityAlpha == null ? original : RenderTypes.entityTranslucentCullItemTarget(skinTextures.cape().texturePath());
// |    } else {
// |       return original;
// |    }
// | }
