// Not ported yet: the Java @Inject(HEAD) on AvatarRenderer.isEntityUpsideDown (MixinAvatarRenderer.shouldFlipUpsideDown).
// Why it's off: No Horizon function mapped for AvatarRenderer.isEntityUpsideDown (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AvatarRenderer.isEntityUpsideDown: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AvatarRenderer.isEntityUpsideDown -> (not mapped yet)
//   Avatar.getUUID -> (not mapped yet)
//   Avatar.hasCustomName -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinAvatarRenderer.java:14:
// | @Inject(method = "isEntityUpsideDown(Lnet/minecraft/world/entity/Avatar;)Z", at = @At("HEAD"), cancellable = true)
// | private void shouldFlipUpsideDown(Avatar entity, CallbackInfoReturnable<Boolean> cir) {
// |    if ((entity instanceof Player || entity.hasCustomName()) && RendererLivingEntityHook.shouldBeUpsideDown(entity.getUUID())) {
// |       cir.setReturnValue(true);
// |    }
// | }
