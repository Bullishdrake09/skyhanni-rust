// Not ported yet: the Java @Inject(RETURN) on GuiGraphicsExtractor.item (MixinGuiGraphicsExtractor.drawItemPost).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.item (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for GuiGraphicsExtractor.item: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinGuiGraphicsExtractor.java:25:
// | @Inject(
// |    method = "item(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;III)V",
// |    at = @At("RETURN")
// | )
// | private void drawItemPost(LivingEntity entity, Level world, ItemStack stack, int x, int y, int seed, CallbackInfo ci) {
// |    RenderItemHookKt.renderItemReturn((GuiGraphicsExtractor)this, stack, x, y);
// | }
