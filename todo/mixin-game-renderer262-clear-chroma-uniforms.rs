// Not ported yet: the Java @Inject(INVOKE) on GameRenderer.render (MixinGameRenderer262.clearChromaUniforms).
// Why it's off: Its target is the `frame` hook point, an empty function: AFTER_CALL has nothing inside it to select. Port it in the `frame` hook's helper.
// horizon-mixins kind: AFTER_CALL
// Note: Hook points are empty functions: use HEAD on the hook instead of AFTER_CALL.
// Note: The call it hooks (GuiRenderer.endFrame) isn't mapped to a Rust function yet.
// Note: Target mapping is approximate.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiRenderer.endFrame -> (not mapped yet)
//   GameRenderer.render -> hook `frame`
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGameRenderer262.java:13:
// | @Inject(method = "render", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/render/GuiRenderer;endFrame()V", shift = Shift.AFTER))
// | private void clearChromaUniforms(CallbackInfo ci) {
// |    GuiRendererHook.clearChromaUniforms();
// | }
