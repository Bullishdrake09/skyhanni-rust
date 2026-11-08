// Not ported yet: the Java @Inject(INVOKE) on Display.tick (MixinDisplay.onRenderSubStateUpdated).
// Why it's off: No Horizon function mapped for Display.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for Display.tick: run `match`.
// Note: The call it hooks (Display.updateRenderSubState) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Display.tick -> (not mapped yet)
//   Display.updateRenderSubState -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinDisplay.java:13:
// | @Inject(method = "tick", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/Display;updateRenderSubState(ZF)V", shift = Shift.AFTER))
// | private void onRenderSubStateUpdated(CallbackInfo ci) {
// |    EntityData.onDisplayRenderStateUpdate((Display)this);
// | }
