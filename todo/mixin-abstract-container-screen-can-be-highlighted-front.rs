// Not ported yet: the Java @WrapOperation(INVOKE) on AbstractContainerScreen.extractSlotHighlightFront (MixinAbstractContainerScreen.canBeHighlightedFront).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.extractSlotHighlightFront (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for AbstractContainerScreen.extractSlotHighlightFront: run `match`.
// Note: The call it hooks (Slot.isHighlightable) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAbstractContainerScreen.java:138:
// | @WrapOperation(method = "extractSlotHighlightFront", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/inventory/Slot;isHighlightable()Z"))
// | private boolean canBeHighlightedFront(Slot slot, Operation<Boolean> original) {
// |    return BetterContainers.slotCanBeHighlighted(slot, (Boolean)original.call(new Object[]{slot}));
// | }
