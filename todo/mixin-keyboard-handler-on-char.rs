// Not ported yet: the Java @Inject(HEAD) on KeyboardHandler.charTyped (MixinKeyboardHandler.onChar).
// Why it's off: No Horizon function mapped for KeyboardHandler.charTyped (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for KeyboardHandler.charTyped: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyboardHandler.charTyped -> (not mapped yet)
//   KeyboardHandler.keyPress -> hook `key`. Down and up, by vanilla key name; repeats left out.
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.player -> `self.native_player()`. Option<NativePlayer>: x, y, z, yaw, pitch, dimension only.
//   CharacterEvent.codepoint -> (not mapped yet)
//   KeyEvent.key -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinKeyboardHandler.java:42:
// | @Inject(method = "charTyped", at = @At("HEAD"))
// | private void onChar(long window, CharacterEvent input, CallbackInfo ci) {
// |    if (Minecraft.getInstance().player != null) {
// |       new CharEvent(input.codepoint()).post();
// |    }
// | }
