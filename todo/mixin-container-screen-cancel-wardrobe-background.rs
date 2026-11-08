// Not ported yet: the Java @Inject(HEAD) on ContainerScreen.extractBackground (MixinContainerScreen.cancelWardrobeBackground).
// Why it's off: No Horizon function mapped for ContainerScreen.extractBackground (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ContainerScreen.extractBackground: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.blit -> (not mapped yet)
//   ContainerScreen.extractBackground -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinContainerScreen.java:32:
// | @Inject(method = "extractBackground", at = @At("HEAD"), cancellable = true)
// | private void cancelWardrobeBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float a, CallbackInfo ci) {
// |    if (GuiData.getPreDrawEventCancelled()) {
// |       ci.cancel();
// |    }
// | }
