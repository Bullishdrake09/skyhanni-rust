// Not ported yet: the Java @WrapOperation(INVOKE) on StringDecomposer.iterateFormatted (MixinStringDecomposer.skyhanni$onGetByCode).
// Why it's off: No Horizon function mapped for StringDecomposer.iterateFormatted (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for StringDecomposer.iterateFormatted: run `match`.
// Note: The call it hooks (ChatFormatting.getByCode) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinStringDecomposer.java:40:
// | @WrapOperation(
// |    method = "iterateFormatted(Ljava/lang/String;ILnet/minecraft/network/chat/Style;Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;)Z",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/ChatFormatting;getByCode(C)Lnet/minecraft/ChatFormatting;")
// | )
// | private static ChatFormatting skyhanni$onGetByCode(char d, Operation<ChatFormatting> original) {
// |    if (d == '#') {
// |       if (skyhanni$hexState == -1) {
// |          skyhanni$hexState = 0;
// |          skyhanni$hexValue = 0;
// |       }
// |    } else if (d == '/' && (skyhanni$hexState == 6 || skyhanni$hexState == 8)) {
// |       skyhanni$activeHexColor = skyhanni$hexValue & 16777215;
// |       skyhanni$hexState = -1;
// |       skyhanni$hexValue = 0;
// |    }
// |
// |    return (ChatFormatting)original.call(new Object[]{d});
// | }
