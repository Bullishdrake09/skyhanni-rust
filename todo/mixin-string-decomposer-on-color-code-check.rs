// Not ported yet: the Java @ModifyVariable(INVOKE_ASSIGN) on StringDecomposer.iterateFormatted (MixinStringDecomposer.onColorCodeCheck).
// Why it's off: No Horizon function mapped for StringDecomposer.iterateFormatted (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for StringDecomposer.iterateFormatted: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChatFormatting.RESET -> (not mapped yet)
//   ChatFormatting.getByCode -> (not mapped yet)
//   ChatFormatting.toString -> (not mapped yet)
//   Style.applyLegacyFormat -> (not mapped yet)
//   Style.withColor -> (not mapped yet)
//   TextColor.fromLegacyFormat -> (not mapped yet)
//   TextColor.fromRgb -> (not mapped yet)
//   StringDecomposer.feedChar -> (not mapped yet)
//   StringDecomposer.iterateFormatted -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinStringDecomposer.java:101:
// | @ModifyVariable(
// |    method = "iterateFormatted(Ljava/lang/String;ILnet/minecraft/network/chat/Style;Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;)Z",
// |    at = @At(value = "INVOKE_ASSIGN", target = "Lnet/minecraft/ChatFormatting;getByCode(C)Lnet/minecraft/ChatFormatting;"),
// |    ordinal = 2
// | )
// | private static Style onColorCodeCheck(Style style, @Local(argsOnly = true) String text, @Local(index = 9) char colorCode) {
// |    return ChromaFontManager.setChromaColorStyle(style, text, colorCode);
// | }
