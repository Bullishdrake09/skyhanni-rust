// Not ported yet: the Java @ModifyArg(INVOKE) on ItemFeatureRenderer.prepareOutlineSubmit (MixinItemFeatureRenderer.modifyAlpha).
// Why it's off: No Horizon function mapped for ItemFeatureRenderer.prepareOutlineSubmit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for ItemFeatureRenderer.prepareOutlineSubmit: run `match`.
// Note: The call it hooks (QuadInstance.setColor) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinItemFeatureRenderer.java:17:
// | @ModifyArg(method = "prepareOutlineSubmit", at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/vertex/QuadInstance;setColor(I)V"), index = 0)
// | private int modifyAlpha(int originalColor) {
// |    if (EntityRenderDispatcherHookKt.getEntity() instanceof LivingEntity livingEntity) {
// |       Integer entityAlpha = EntityTransparencyManager.getEntityTransparency(livingEntity);
// |       if (entityAlpha == null) {
// |          return originalColor;
// |       }
// |
// |       int newAlpha = Math.min(ARGB.alpha(originalColor), entityAlpha);
// |       return ARGB.color(newAlpha, ARGB.red(originalColor), ARGB.green(originalColor), ARGB.blue(originalColor));
// |    } else {
// |       return originalColor;
// |    }
// | }
