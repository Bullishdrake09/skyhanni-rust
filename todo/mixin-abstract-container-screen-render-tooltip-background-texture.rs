// Not ported yet: the Java @Inject(INVOKE) on AbstractContainerScreen.extractContents (MixinAbstractContainerScreen.renderTooltipBackgroundTexture).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.extractContents (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for AbstractContainerScreen.extractContents: run `match`.
// Note: The call it hooks (AbstractContainerScreen.extractSlotHighlightBack) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.setTooltipForNextFrame -> (not mapped yet)
//   GuiGraphicsExtractor.text -> (not mapped yet)
//   Screen.<init> -> (not mapped yet)
//   Screen.extractRenderState -> (not mapped yet)
//   AbstractContainerScreen.extractContents -> (not mapped yet)
//   AbstractContainerScreen.extractLabels -> (not mapped yet)
//   AbstractContainerScreen.extractRenderState -> (not mapped yet)
//   AbstractContainerScreen.extractSlot -> (not mapped yet)
//   AbstractContainerScreen.extractSlotHighlightBack -> (not mapped yet)
//   AbstractContainerScreen.extractSlotHighlightFront -> (not mapped yet)
//   AbstractContainerScreen.extractTooltip -> (not mapped yet)
//   AbstractContainerScreen.getMenu -> (not mapped yet)
//   AbstractContainerScreen.keyPressed -> (not mapped yet)
//   AbstractContainerScreen.mouseClicked -> (not mapped yet)
//   AbstractContainerScreen.onClose -> (not mapped yet)
//   KeyEvent.input -> (not mapped yet)
//   Slot.isHighlightable -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAbstractContainerScreen.java:205:
// | @Inject(
// |    method = "extractContents",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;extractSlotHighlightBack(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | private void renderTooltipBackgroundTexture(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks, CallbackInfo ci) {
// |    ToolTipData.INSTANCE.setLastSlot(this.hoveredSlot);
// | }
