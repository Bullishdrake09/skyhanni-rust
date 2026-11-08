// Not ported yet: the Java @Inject(TAIL) on TextColor.equals (MixinTextColor.chromaEqualityFix).
// Why it's off: No Horizon function mapped for TextColor.equals (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for TextColor.equals: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   TextColor.equals -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinTextColor.java:12:
// | @Inject(method = "equals", at = @At("TAIL"), cancellable = true)
// | public void chromaEqualityFix(Object o, CallbackInfoReturnable<Boolean> cir) {
// |    if (ChromaFontManager.isNotActuallyEqualBecauseOfChroma((TextColor)this, o)) {
// |       cir.setReturnValue(false);
// |    }
// | }
