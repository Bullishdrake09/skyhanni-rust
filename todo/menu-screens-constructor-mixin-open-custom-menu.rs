// Not ported yet: the Java @Inject(HEAD) on ScreenConstructor.fromPacket (MenuScreensConstructorMixin.skyHanni$openCustomMenu).
// Why it's off: No Horizon function mapped for ScreenConstructor.fromPacket (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ScreenConstructor.fromPacket: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ScreenConstructor.fromPacket -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MenuScreensConstructorMixin.java:16:
// | @Inject(method = "fromPacket", at = @At("HEAD"), cancellable = true)
// | private void skyHanni$openCustomMenu(Component name, MenuType<T> type, Minecraft client, int id, CallbackInfo ci) {
// |    if (MenuScreensHook.openCustomMenu(name, type, client, id)) {
// |       ci.cancel();
// |    }
// | }
