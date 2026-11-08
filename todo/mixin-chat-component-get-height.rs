// Not ported yet: the Java @Inject(HEAD) on ChatComponent.getHeight (MixinChatComponent.getHeight).
// Why it's off: No Horizon function mapped for ChatComponent.getHeight (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChatComponent.getHeight: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.options -> (not mapped yet)
//   OptionInstance.get -> (not mapped yet)
//   Options.chatHeightFocused -> (not mapped yet)
//   ChatComponent.getHeight -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinChatComponent.java:24:
// | @Shadow
// | public static int getHeight(double heightOption) {
// |    return 0;
// | }
