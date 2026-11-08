// Not ported yet: the Java @ModifyVariable(HEAD) on Font.prepareText (MixinFont.modifyOrderedText).
// Why it's off: No Horizon function mapped for Font.prepareText (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for Font.prepareText: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Font.prepareText -> (not mapped yet)
//   Font.width -> `self.native_text_width(text)`
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinFont.java:13:
// | @ModifyVariable(
// |    method = "prepareText(Lnet/minecraft/util/FormattedCharSequence;FFIZZI)Lnet/minecraft/client/gui/Font$PreparedText;",
// |    index = 1,
// |    at = @At("HEAD"),
// |    argsOnly = true
// | )
// | private FormattedCharSequence modifyOrderedText(FormattedCharSequence value) {
// |    return VisualWordsHook.modifyOrderedText(value);
// | }
