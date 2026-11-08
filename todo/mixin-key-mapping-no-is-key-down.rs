// Not ported yet: the Java @Inject(HEAD) on KeyMapping.isDown (MixinKeyMapping.noIsKeyDown).
// Why it's off: Mapped to a helper call, not a function to inject into.
// horizon-mixins kind: HEAD
// Note: Horizon has no function to inject into here, only the helper `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own`.
// Note: Target mapping is approximate.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   KeyMapping.consumeClick -> `key == self.native_mod_key(..) && pressed, in the key hook`. Horizon gives presses as key hook calls, not a queue.
//   KeyMapping.isDown -> `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own`
//   ToggleKeyMapping.needsToggle -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinKeyMapping.java:23:
// | @Inject(method = "isDown", at = @At("HEAD"), cancellable = true)
// | public void noIsKeyDown(CallbackInfoReturnable<Boolean> cir) {
// |    KeyMapping keyBinding = (KeyMapping)this;
// |    GardenCustomKeybinds.isKeyDown(keyBinding, this.isDown, cir);
// |    if (!(keyBinding instanceof ToggleKeyMapping stickyKeyBinding && stickyKeyBinding.needsToggle.getAsBoolean())) {
// |       TextInput.Companion.onMinecraftInput(keyBinding, cir);
// |       GraphEditor.INSTANCE.onMinecraftInput(keyBinding, cir);
// |    }
// | }
