// Not ported yet: the Java @Inject(HEAD) on AbstractContainerScreen.mouseClicked (MixinAbstractContainerScreen.mouseClicked).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.mouseClicked (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AbstractContainerScreen.mouseClicked: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAbstractContainerScreen.java:109:
// | @Inject(method = "mouseClicked", at = @At("HEAD"), cancellable = true)
// | private void mouseClicked(MouseButtonEvent mouseButtonEvent, boolean bl, CallbackInfoReturnable<Boolean> cir) {
// |    AbstractContainerScreen<?> screen = (AbstractContainerScreen<?>)this;
// |    if (new GuiKeyPressEvent.GuiMouseButtonPressEvent(screen).post().isCancelled()) {
// |       cir.setReturnValue(false);
// |    }
// |
// |    if (new GuiMouseInputEvent(screen).post().isCancelled()) {
// |       cir.setReturnValue(false);
// |    }
// | }
