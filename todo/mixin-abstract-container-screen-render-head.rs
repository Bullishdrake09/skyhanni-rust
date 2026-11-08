// Not ported yet: the Java @Inject(HEAD) on AbstractContainerScreen.extractRenderState (MixinAbstractContainerScreen.renderHead).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AbstractContainerScreen.extractRenderState: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAbstractContainerScreen.java:55:
// | @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
// | private void renderHead(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks, CallbackInfo ci) {
// |    if (!GlobalRender.getRenderDisabled()) {
// |       AbstractContainerScreen<?> gui = (AbstractContainerScreen<?>)this;
// |       if (new GuiContainerEvent.PreDraw(context, gui, gui.getMenu(), mouseX, mouseY, deltaTicks).post().isCancelled()) {
// |          GuiData.setPreDrawEventCancelled(true);
// |          ci.cancel();
// |       } else {
// |          DelayedRun.runNextTick(() -> GuiData.setPreDrawEventCancelled(false));
// |       }
// |    }
// | }
