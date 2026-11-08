// Not ported yet: the Java @ModifyVariable(HEAD) on Font.width (MixinFont.modifyWidthFormattedText).
// Why it's off: Mapped to a helper call, not a function to inject into.
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: Horizon has no function to inject into here, only the helper `self.native_text_width(text)`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Font.prepareText -> (not mapped yet)
//   Font.width -> `self.native_text_width(text)`
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinFont.java:38:
// | @ModifyVariable(method = "width(Lnet/minecraft/network/chat/FormattedText;)I", index = 1, at = @At("HEAD"), argsOnly = true)
// | private FormattedText modifyWidthFormattedText(FormattedText value) {
// |    return VisualWordsHook.modifyFormattedText(value);
// | }
