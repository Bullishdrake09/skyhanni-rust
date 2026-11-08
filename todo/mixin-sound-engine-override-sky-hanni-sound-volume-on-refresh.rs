// Not ported yet: the Java @ModifyReturnValue(RETURN) on SoundEngine.calculateVolume (MixinSoundEngine.overrideSkyHanniSoundVolumeOnRefresh).
// Why it's off: No Horizon function mapped for SoundEngine.calculateVolume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for SoundEngine.calculateVolume: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinSoundEngine.java:42:
// | @ModifyReturnValue(method = "calculateVolume(Lnet/minecraft/client/resources/sounds/SoundInstance;)F", at = @At("RETURN"))
// | public float overrideSkyHanniSoundVolumeOnRefresh(float original, @Local(argsOnly = true) SoundInstance soundInstance) {
// |    return SoundEngineHook.modifyVolume(soundInstance, original);
// | }
