// Not ported yet: the Java @WrapOperation(INVOKE) on StringDecomposer.iterateFormatted (MixinStringDecomposer.skyhanni$onApplyLegacyFormat).
// Why it's off: No Horizon function mapped for StringDecomposer.iterateFormatted (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for StringDecomposer.iterateFormatted: run `match`.
// Note: The call it hooks (Style.applyLegacyFormat) isn't mapped to a Rust function yet.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinStringDecomposer.java:59:
// | @WrapOperation(
// |    method = "iterateFormatted(Ljava/lang/String;ILnet/minecraft/network/chat/Style;Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;)Z",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/network/chat/Style;applyLegacyFormat(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/Style;"
// |    )
// | )
// | private static Style skyhanni$onApplyLegacyFormat(Style style, ChatFormatting chatFormatting, Operation<Style> original) {
// |    if (skyhanni$hexState >= 0 && skyhanni$hexState < 8) {
// |       int hexDigit = "0123456789abcdef".indexOf(chatFormatting.toString().charAt(1));
// |       if (hexDigit >= 0) {
// |          skyhanni$hexState++;
// |          skyhanni$hexValue = skyhanni$hexValue << 4 | hexDigit;
// |          return style;
// |       }
// |
// |       skyhanni$hexState = -1;
// |       skyhanni$hexValue = 0;
// |    } else if (skyhanni$hexState >= 8) {
// |       skyhanni$hexState = -1;
// |       skyhanni$hexValue = 0;
// |    }
// |
// |    if (TextColor.fromLegacyFormat(chatFormatting) != null || chatFormatting == ChatFormatting.RESET) {
// |       skyhanni$activeHexColor = -1;
// |    }
// |
// |    return (Style)original.call(new Object[]{style, chatFormatting});
// | }
