// Not ported yet: the Java @ModifyArg(INVOKE) on Model.renderToBuffer (MixinModel.modifyRenderAlpha).
// Why it's off: No Horizon function mapped for Model.renderToBuffer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for Model.renderToBuffer: run `match`.
// Note: The call it hooks (ModelPart.render) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Model.renderToBuffer -> (not mapped yet)
//   ModelPart.render -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinModel.java:13:
// | @ModifyArg(
// |    method = "renderToBuffer(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/model/geom/ModelPart;render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V"
// |    ),
// |    index = 4
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
