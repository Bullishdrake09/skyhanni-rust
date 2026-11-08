// Not ported yet: the Java @WrapOperation(INVOKE) on Screen.extractRenderStateWithTooltipAndSubtitles (MixinScreen.wrapExtractBackground).
// Why it's off: No Horizon function mapped for Screen.extractRenderStateWithTooltipAndSubtitles (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for Screen.extractRenderStateWithTooltipAndSubtitles: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Screen.extractBackground -> horizon_client::game::screens::Screen::blurs_background (suggested, high)
//   Screen.extractRenderStateWithTooltipAndSubtitles -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinScreen.java:14:
// | @WrapOperation(
// |    method = "extractRenderStateWithTooltipAndSubtitles",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V")
// | )
// | private void wrapExtractBackground(Screen instance, GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks, Operation<Void> original) {
// |    original.call(new Object[]{instance, context, mouseX, mouseY, deltaTicks});
// |    new ScreenDrawnEvent(context, MinecraftCompat.getScreen()).post();
// | }
