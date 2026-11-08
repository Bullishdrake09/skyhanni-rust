// Not ported yet: the Java @WrapOperation(INVOKE) on MouseHandler.turnPlayer (MixinMouseHandler.modifyMouseSensitivity).
// Why it's off: No Horizon function mapped for MouseHandler.turnPlayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for MouseHandler.turnPlayer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MouseHandler.onButton -> (not mapped yet)
//   MouseHandler.onMove -> (not mapped yet)
//   MouseHandler.onScroll -> hook `wheel`. While playing; set claimed to keep the steps from the hotbar.
//   MouseHandler.turnPlayer -> (not mapped yet)
//   LocalPlayer.turn -> horizon_client_input::mouse::Rotation::turn (suggested, high)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinMouseHandler.java:41:
// | @WrapOperation(method = "turnPlayer", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/player/LocalPlayer;turn(DD)V"))
// | private void modifyMouseSensitivity(LocalPlayer instance, double x, double y, Operation<Void> original) {
// |    original.call(new Object[]{instance, MouseSensitivityReducer.remapSensitivity(x), MouseSensitivityReducer.remapSensitivity(y)});
// | }
