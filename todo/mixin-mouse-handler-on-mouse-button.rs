// Not ported yet: the Java @Inject(RETURN) on MouseHandler.onMove (MixinMouseHandler.onMouseButton).
// Why it's off: No Horizon function mapped for MouseHandler.onMove (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for MouseHandler.onMove: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MouseHandler.onButton -> (not mapped yet)
//   MouseHandler.onMove -> (not mapped yet)
//   MouseHandler.onScroll -> hook `wheel`. While playing; set claimed to keep the steps from the hotbar.
//   MouseHandler.turnPlayer -> (not mapped yet)
//   LocalPlayer.turn -> horizon_client_input::mouse::Rotation::turn (suggested, high)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinMouseHandler.java:24:
// | @Inject(method = "onMove", at = @At("RETURN"))
// | private void onMouseButton(long window, double x, double y, CallbackInfo ci) {
// |    MouseCompat.setDeltaMouseX(this.accumulatedDX);
// |    MouseCompat.setDeltaMouseY(this.accumulatedDY);
// | }
