// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on SoundEngine.play (MixinSoundEngine.overrideSkyHanniSoundVolume).
// Why it's off: No Horizon function mapped for SoundEngine.play (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for SoundEngine.play: run `match`.
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
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinSoundEngine.java:34:
// | @ModifyExpressionValue(
// |    method = "play",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/sounds/SoundEngine;calculateVolume(FLnet/minecraft/sounds/SoundSource;)F")
// | )
// | public float overrideSkyHanniSoundVolume(float original, @Local(argsOnly = true) SoundInstance soundInstance) {
// |    return SoundEngineHook.modifyVolume(soundInstance, original);
// | }
