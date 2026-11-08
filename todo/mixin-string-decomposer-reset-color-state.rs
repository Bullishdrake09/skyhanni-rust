// Not ported yet: the Java @Inject(HEAD) on StringDecomposer.iterateFormatted (MixinStringDecomposer.skyhanni$resetColorState).
// Why it's off: No Horizon function mapped for StringDecomposer.iterateFormatted (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinStringDecomposer.java:30:
// | @Inject(
// |    method = "iterateFormatted(Ljava/lang/String;ILnet/minecraft/network/chat/Style;Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;)Z",
// |    at = @At("HEAD")
// | )
// | private static void skyhanni$resetColorState(CallbackInfoReturnable<Boolean> cir) {
// |    skyhanni$hexState = -1;
// |    skyhanni$hexValue = 0;
// |    skyhanni$activeHexColor = -1;
// | }
