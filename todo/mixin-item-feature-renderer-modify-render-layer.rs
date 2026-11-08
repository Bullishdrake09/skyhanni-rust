// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on ItemFeatureRenderer.prepareOutlineSubmit (MixinItemFeatureRenderer.modifyRenderLayer).
// Why it's off: No Horizon function mapped for ItemFeatureRenderer.prepareOutlineSubmit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for ItemFeatureRenderer.prepareOutlineSubmit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   QuadInstance.setColor -> (not mapped yet)
//   ItemFeatureRenderer.prepareOutlineSubmit -> (not mapped yet)
//   RenderTypes.glintTranslucent -> (not mapped yet)
//   ARGB.alpha -> (not mapped yet)
//   ARGB.blue -> (not mapped yet)
//   ARGB.color -> (not mapped yet)
//   ARGB.green -> (not mapped yet)
//   ARGB.red -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinItemFeatureRenderer.java:32:
// | @ModifyExpressionValue(
// |    method = "prepareOutlineSubmit",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo;itemRenderType()Lnet/minecraft/client/renderer/rendertype/RenderType;"
// |    )
// | )
// | private RenderType modifyRenderLayer(RenderType layer) {
// |    if (EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       return EntityTransparencyManager.getEntityTransparency(livingEntity) == null ? layer : RenderTypes.glintTranslucent();
// |    } else {
// |       return layer;
// |    }
// | }
