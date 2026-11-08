// Not ported yet: the Java @Inject(INVOKE) on SoundEngine.play (MixinSoundEngine.handleSound).
// Why it's off: No Horizon function mapped for SoundEngine.play (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for SoundEngine.play: run `match`.
// Note: The call it hooks (SoundInstance.getVolume) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundInstance.getIdentifier -> (not mapped yet)
//   SoundInstance.getPitch -> (not mapped yet)
//   SoundInstance.getVolume -> (not mapped yet)
//   SoundInstance.getX -> (not mapped yet)
//   SoundInstance.getY -> (not mapped yet)
//   SoundInstance.getZ -> (not mapped yet)
//   SoundEngine.calculateVolume -> (not mapped yet)
//   SoundEngine.play -> (not mapped yet)
//   PlayResult.NOT_STARTED -> (not mapped yet)
//   Identifier.toString -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinSoundEngine.java:20:
// | @Inject(method = "play", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/resources/sounds/SoundInstance;getVolume()F"), cancellable = true)
// | public void handleSound(SoundInstance soundInstance, CallbackInfoReturnable<PlayResult> cir) {
// |    if (new PlaySoundEvent(
// |          StringUtils.removeStart(soundInstance.getIdentifier().toString(), "minecraft:"),
// |          new LorenzVec(soundInstance.getX(), soundInstance.getY(), soundInstance.getZ()),
// |          soundInstance.getPitch(),
// |          soundInstance.getVolume()
// |       )
// |       .post()
// |       .isCancelled()) {
// |       cir.setReturnValue(PlayResult.NOT_STARTED);
// |    }
// | }
