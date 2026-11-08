// Not ported yet: the Java @Inject(HEAD) on AbstractContainerScreen.keyPressed (MixinAbstractContainerScreen.keyPressed).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.keyPressed (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AbstractContainerScreen.keyPressed: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAbstractContainerScreen.java:99:
// | @Inject(method = "keyPressed", at = @At("HEAD"), cancellable = true)
// | private void keyPressed(KeyEvent input, CallbackInfoReturnable<Boolean> cir) {
// |    int keyCode = input.input();
// |    TextInput.Companion.onGuiInput(cir);
// |    boolean shouldCancelInventoryClose = KeyboardManager.checkIsInventoryClosure(keyCode);
// |    if (new GuiKeyPressEvent.GuiKeyboardKeyPressEvent((AbstractContainerScreen<?>)this).post().isCancelled() || shouldCancelInventoryClose) {
// |       cir.setReturnValue(false);
// |    }
// | }
