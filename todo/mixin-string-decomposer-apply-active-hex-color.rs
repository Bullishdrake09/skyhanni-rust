// Not ported yet: the Java @ModifyArg(INVOKE) on StringDecomposer.iterateFormatted (MixinStringDecomposer.skyhanni$applyActiveHexColor).
// Why it's off: No Horizon function mapped for StringDecomposer.iterateFormatted (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for StringDecomposer.iterateFormatted: run `match`.
// Note: The call it hooks (StringDecomposer.feedChar) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinStringDecomposer.java:89:
// | @ModifyArg(
// |    method = "iterateFormatted(Ljava/lang/String;ILnet/minecraft/network/chat/Style;Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;)Z",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/util/StringDecomposer;feedChar(Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;IC)Z"
// |    ),
// |    index = 0
// | )
// | private static Style skyhanni$applyActiveHexColor(Style style) {
// |    return skyhanni$activeHexColor == -1 ? style : style.withColor(TextColor.fromRgb(skyhanni$activeHexColor));
// | }
