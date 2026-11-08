// Not ported yet: the Java @ModifyArg(INVOKE) on ContainerScreen.extractBackground (MixinContainerScreen.getCustomTexture).
// Why it's off: No Horizon function mapped for ContainerScreen.extractBackground (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for ContainerScreen.extractBackground: run `match`.
// Note: The call it hooks (GuiGraphicsExtractor.blit) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.blit -> (not mapped yet)
//   ContainerScreen.extractBackground -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinContainerScreen.java:20:
// | @ModifyArg(
// |    method = "extractBackground",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/GuiGraphicsExtractor;blit(Lcom/mojang/blaze3d/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIFFIIII)V"
// |    ),
// |    index = 1
// | )
// | private Identifier getCustomTexture(Identifier sprite) {
// |    return this.skyhanni$hook.getTexture(sprite);
// | }
