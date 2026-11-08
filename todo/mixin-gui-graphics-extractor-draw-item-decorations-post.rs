// Not ported yet: the Java @Inject(RETURN) on GuiGraphicsExtractor.itemDecorations (MixinGuiGraphicsExtractor.drawItemDecorationsPost).
// Why it's off: `horizon_client_render::hud_overlays::item_decorations` has a closure in its body: horizon-mixins doesn't rewrite such bodies with RETURN. A HEAD hook on it, or a RETURN on a function it calls, may work instead.
// horizon-mixins kind: RETURN on `horizon_client_render::hud_overlays::item_decorations`
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Objects.requireNonNull -> (not mapped yet)
//   Font.split -> horizon_client_render::font::Font::split_formatted (suggested, high)
//   GuiGraphicsExtractor.componentHoverEffect -> (not mapped yet)
//   GuiGraphicsExtractor.item -> (not mapped yet)
//   GuiGraphicsExtractor.itemDecorations -> horizon_client_render::hud_overlays::item_decorations (suggested, high)
//   GuiGraphicsExtractor.text -> (not mapped yet)
//   GuiRenderState.addText -> (not mapped yet)
//   GuiTextRenderState.<init> -> (not mapped yet)
//   GuiTextRenderState.backgroundColor -> (not mapped yet)
//   GuiTextRenderState.color -> (not mapped yet)
//   GuiTextRenderState.dropShadow -> (not mapped yet)
//   GuiTextRenderState.font -> (not mapped yet)
//   GuiTextRenderState.pose -> (not mapped yet)
//   GuiTextRenderState.scissor -> (not mapped yet)
//   GuiTextRenderState.text -> (not mapped yet)
//   GuiTextRenderState.x -> (not mapped yet)
//   GuiTextRenderState.y -> (not mapped yet)
//   Style.getHoverEvent -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiGraphicsExtractor.java:33:
// | @Inject(method = "itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V", at = @At("RETURN"))
// | private void drawItemDecorationsPost(Font textRenderer, ItemStack stack, int x, int y, String stackCountText, CallbackInfo ci) {
// |    RenderItemHookKt.renderItemOverlayPost((GuiGraphicsExtractor)this, stack, x, y, stackCountText);
// | }
