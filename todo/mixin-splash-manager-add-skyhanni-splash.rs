// Not ported yet: the Java @Inject(HEAD) on SplashManager.getSplash (MixinSplashManager.addSkyhanniSplash).
// Why it's off: No Horizon function mapped for SplashManager.getSplash (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SplashManager.getSplash: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SplashRenderer.<init> -> (not mapped yet)
//   SplashManager.getSplash -> (not mapped yet)
//   RandomSource.nextInt -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinSplashManager.java:25:
// | @Inject(method = "getSplash", at = @At("HEAD"), cancellable = true)
// | public void addSkyhanniSplash(CallbackInfoReturnable<SplashRenderer> cir) {
// |    if (RANDOM.nextInt(this.splashes.size() + 1) == this.splashes.size()) {
// |       cir.setReturnValue(new SplashRenderer(TextHelper.INSTANCE.createGradientText(LorenzColor.YELLOW, LorenzColor.GOLD, "SkyHanni!")));
// |    }
// | }
