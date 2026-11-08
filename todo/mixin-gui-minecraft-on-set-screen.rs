// Not ported yet: the Java @Inject(FIELD) on Gui.setScreen (MixinGui_Minecraft.onSetScreen).
// Why it's off: No horizon-mixins counterpart: FIELD, NEW, JUMP and CONSTANT points have no horizon-mixins counterpart yet.
// Note: FIELD, NEW, JUMP and CONSTANT points have no horizon-mixins counterpart yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Gui.setScreen -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGui_Minecraft.java:13:
// | @Inject(method = "setScreen", at = @At(value = "FIELD", target = "Lnet/minecraft/client/gui/Gui;screen:Lnet/minecraft/client/gui/screens/Screen;"))
// | private void onSetScreen(Screen screen, CallbackInfo ci) {
// |    new GuiScreenOpenEvent(screen).post();
// | }
